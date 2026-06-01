use std::cell::RefCell;
use std::sync::Arc;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use ratatui::{
	crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode},
	widgets::ListState,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum FormField {
	#[default]
	Keys,
	Interval,
	Activate,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Pane {
	#[default]
	Toggles,
	Clicks,
}

#[derive(Debug)]
pub struct AppState {
	pub active_pane: Pane,
	pub focus_signal: Arc<(Mutex<bool>, Condvar)>,
	pub focus_regained_time: Instant,
	pub show_add_form: bool,
	pub show_delete_confirm: bool,
	pub form_field: FormField,
	pub form_keys: String,
	pub form_interval: String,
	pub form_activate: String,
	pub toggles: Vec<Toggle>,
	pub clicks: Vec<Click>,
	pub list_state: RefCell<ListState>,
	pub toggle_controller: crate::os_input::ToggleController,
}

#[derive(Debug)]
pub struct Toggle {
	pub keys: String,
	pub active: bool,
}

#[derive(Debug)]
pub struct Click {
	pub keys: String,
	pub activate: String,
	pub interval: u32,
	pub active: bool,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			active_pane: Pane::Toggles,
			focus_signal: Arc::new((Mutex::new(true), Condvar::new())),
			focus_regained_time: Instant::now(),
			show_add_form: false,
			show_delete_confirm: false,
			form_field: FormField::Keys,
			form_keys: String::new(),
			form_interval: String::new(),
			form_activate: String::new(),
			toggles: Vec::new(),
			clicks: Vec::new(),
			list_state: RefCell::new(ListState::default()),
			toggle_controller: crate::os_input::ToggleController::new(),
		}
	}
}

impl AppState {
	pub fn handle_key(&mut self, event: KeyEvent) -> bool {
		let code = event.code;
		let modifiers = event.modifiers;

		if self.show_add_form {
			return self.handle_form_key(event);
		}

		if self.show_delete_confirm {
			return self.handle_delete_confirm_key(code);
		}

		let mut quit_application = false;

		match code {
			KeyCode::Char('q') if modifiers.contains(KeyModifiers::SHIFT) => {
				quit_application = true;
			}
			KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
				quit_application = true;
			}
			KeyCode::Char('h') if modifiers.contains(KeyModifiers::SHIFT) => {
				// TODO: display a help popup that explains keybinds
			}
			KeyCode::Char('h') | KeyCode::Char('t') => {
				self.active_pane = Pane::Toggles;
			}
			KeyCode::Char('j') => {
				self.list_state.borrow_mut().select_next();
			}
			KeyCode::Char('k') => {
				self.list_state.borrow_mut().select_previous();
			}
			KeyCode::Char('l') | KeyCode::Char('c') => {
				self.active_pane = Pane::Clicks;
			}
			KeyCode::Char('a') => {
				self.show_add_form = true;
				self.form_field = FormField::Keys;
				self.form_keys.clear();
				self.form_interval.clear();
				self.form_activate.clear();
			}
			KeyCode::Char('d') if modifiers.contains(KeyModifiers::SHIFT) => {
				self.request_delete_selected();
			}
			KeyCode::Enter => {
				self.toggle_status();
			}
			KeyCode::F(8) => {}
			_ => {}
		}

		quit_application
	}

	fn handle_delete_confirm_key(&mut self, code: KeyCode) -> bool {
		match code {
			KeyCode::Esc => {
				self.show_delete_confirm = false;
				false
			}
			KeyCode::Enter => {
				self.delete_selected();
				self.show_delete_confirm = false;
				false
			}
			_ => false,
		}
	}

	fn handle_form_key(&mut self, event: KeyEvent) -> bool {
		let code = event.code;
		let modifiers = event.modifiers;

		match code {
			KeyCode::Esc => {
				self.discard_form();
				false
			}
			KeyCode::Enter => {
				self.submit_form();
				false
			}
			KeyCode::Tab => {
				if self.active_pane == Pane::Clicks {
					self.form_field = match self.form_field {
						FormField::Keys => FormField::Interval,
						FormField::Interval => FormField::Activate,
						FormField::Activate => FormField::Keys,
					};
				}
				false
			}
			KeyCode::Backspace => {
				match self.form_field {
					FormField::Keys => {
						self.form_keys.pop();
					}
					FormField::Interval => {
						self.form_interval.pop();
					}
					FormField::Activate => {
						self.form_activate.pop();
					}
				}
				false
			}
			KeyCode::Null => {
				if self.form_field == FormField::Keys {
					if let Some(modifier_combo) = format_modifier_only(modifiers) {
						self.form_keys.push_str(&modifier_combo);
					}
				} else if self.form_field == FormField::Activate {
					if let Some(modifier_combo) = format_modifier_only(modifiers) {
						self.form_activate.push_str(&modifier_combo);
					}
				}
				false
			}
			KeyCode::Modifier(m) => {
				if self.form_field == FormField::Keys {
					if let Some(tok) = modifier_token_from_modifier_keycode(m) {
						self.form_keys.push_str(&tok);
					}
				} else if self.form_field == FormField::Activate {
					if let Some(tok) = modifier_token_from_modifier_keycode(m) {
						self.form_activate.push_str(&tok);
					}
				}
				false
			}
			KeyCode::Char(ch) if self.form_field == FormField::Keys => {
				if let Some(modifier_combo) = format_modifier_combo(modifiers, ch) {
					self.form_keys.push_str(&modifier_combo);
				} else {
					self.form_keys.push(ch);
				}
				false
			}
			KeyCode::Char(ch) => {
				match self.form_field {
					FormField::Keys => {
						if let Some(modifier_combo) = format_modifier_combo(modifiers, ch) {
							self.form_keys.push_str(&modifier_combo);
						} else {
							self.form_keys.push(ch);
						}
					}
					FormField::Interval => {
						if ch.is_ascii_digit() {
							self.form_interval.push(ch);
						}
					}
					FormField::Activate => {
						if let Some(modifier_combo) = format_modifier_combo(modifiers, ch) {
							self.form_activate.push_str(&modifier_combo);
						} else {
							self.form_activate.push(ch);
						}
					}
				}
				false
			}
			_ => false,
		}
	}

	fn discard_form(&mut self) {
		self.show_add_form = false;
		self.form_field = FormField::Keys;
		self.form_keys.clear();
		self.form_interval.clear();
		self.form_activate.clear();
	}

	fn submit_form(&mut self) {
		let keys = normalize_key_combo(self.form_keys.trim());

		if keys.is_empty() {
			return;
		}

		match self.active_pane {
			Pane::Toggles => {
				self.toggles.push(Toggle {
					keys,
					active: false,
				});
				let last = self.toggles.len().saturating_sub(1);
				self.list_state.borrow_mut().select(Some(last));
				self.discard_form();
			}
			Pane::Clicks => {
				let Ok(interval) = self.form_interval.trim().parse::<u32>() else {
					return;
				};

				let activate = normalize_key_combo(self.form_activate.trim());

				if activate.is_empty() {
					return;
				}

				self.clicks.push(Click {
					keys,
					activate,
					interval,
					active: false,
				});
				let last = self.clicks.len().saturating_sub(1);
				self.list_state.borrow_mut().select(Some(last));
				self.discard_form();
			}
		}
	}

	fn delete_selected(&mut self) {
		let index = self.list_state.borrow().selected();
		if let Some(i) = index {
			match self.active_pane {
				Pane::Toggles => {
					if i < self.toggles.len() {
						self.toggle_controller.stop_toggle(i);

						self.toggles.remove(i);
						let len = self.toggles.len();
						if len == 0 {
							self.list_state.borrow_mut().select(None);
						} else {
							let new = if i >= len { len - 1 } else { i };
							self.list_state.borrow_mut().select(Some(new));
						}

						self.toggle_controller.remove_index(i);
					}
				}
				Pane::Clicks => {
					if i < self.clicks.len() {
						self.clicks.remove(i);
						let len = self.clicks.len();
						if len == 0 {
							self.list_state.borrow_mut().select(None);
						} else {
							let new = if i >= len { len - 1 } else { i };
							self.list_state.borrow_mut().select(Some(new));
						}
					}
				}
			}
		}
	}

	pub fn request_delete_selected(&mut self) {
		if self.list_state.borrow().selected().is_some() {
			self.show_delete_confirm = true;
		}
	}

	fn toggle_status(&mut self) {
		let index = self.list_state.borrow().selected();

		if let Some(i) = index {
			match self.active_pane {
				Pane::Toggles => {
					if let Some(t) = self.toggles.get_mut(i) {
						t.active = !t.active;

						if t.active {
							let keys = t.keys.clone();
							self.toggle_controller
								.start_toggle(i, keys, &self.focus_signal);
						} else {
							self.toggle_controller.stop_toggle(i);
						}
					}
				}
				Pane::Clicks => {
					if let Some(c) = self.clicks.get_mut(i) {
						c.active = !c.active
					}
				}
			}
		}
	}
}

fn normalize_key_combo(raw: &str) -> String {
	raw.split('+')
		.filter_map(|part| {
			let token = part.trim();
			if token.is_empty() {
				return None;
			}

			Some(format_key_token(token))
		})
		.collect::<Vec<String>>()
		.join("+")
}

fn format_key_token(token: &str) -> String {
	if let Some(modifier) = normalize_modifier_name(token) {
		return format!("<{}>", modifier);
	}

	if token.chars().count() == 1 {
		// Preserve the user's letter case for single-character keys
		let ch = token.chars().next().unwrap();
		if ch.is_ascii_alphabetic() {
			return ch.to_string();
		}
	}

	token.to_string()
}

fn normalize_modifier_name(token: &str) -> Option<&'static str> {
	let name = token.trim_matches(|c| c == '<' || c == '>').to_lowercase();

	match name.as_str() {
		"shift" => Some("Shift"),
		"ctrl" | "control" => Some("Ctrl"),
		"alt" => Some("Alt"),
		"meta" | "cmd" | "super" | "win" | "windows" => Some("Meta"),
		_ => None,
	}
}

fn format_modifier_combo(modifiers: KeyModifiers, ch: char) -> Option<String> {
	if modifiers.is_empty() {
		return None;
	}

	let mut parts = modifier_tokens(modifiers);

	if parts.is_empty() {
		return None;
	}

	let key = if ch.is_ascii_alphabetic() {
		ch.to_ascii_uppercase().to_string()
	} else {
		ch.to_string()
	};

	parts.push(key);
	Some(parts.join("+"))
}

fn format_modifier_only(modifiers: KeyModifiers) -> Option<String> {
	if modifiers.is_empty() {
		return None;
	}

	let parts = modifier_tokens(modifiers);

	if parts.is_empty() {
		return None;
	}

	Some(parts.join("+"))
}

fn modifier_tokens(modifiers: KeyModifiers) -> Vec<String> {
	let mut parts = Vec::new();

	if modifiers.contains(KeyModifiers::SHIFT) {
		parts.push(String::from("<Shift>"));
	}
	if modifiers.contains(KeyModifiers::CONTROL) {
		parts.push(String::from("<Ctrl>"));
	}
	if modifiers.contains(KeyModifiers::ALT) {
		parts.push(String::from("<Alt>"));
	}
	if modifiers.contains(KeyModifiers::SUPER) {
		parts.push(String::from("<Meta>"));
	}

	parts
}

fn modifier_token_from_modifier_keycode(m: ModifierKeyCode) -> Option<String> {
	use ModifierKeyCode::*;

	match m {
		LeftShift | RightShift => Some(String::from("<Shift>")),
		LeftControl | RightControl => Some(String::from("<Ctrl>")),
		LeftAlt | RightAlt => Some(String::from("<Alt>")),
		LeftSuper | RightSuper => Some(String::from("<Meta>")),
		_ => None,
	}
}

impl Drop for AppState {
	fn drop(&mut self) {
		self.toggle_controller.shutdown();
		thread::sleep(Duration::from_millis(100));
	}
}
