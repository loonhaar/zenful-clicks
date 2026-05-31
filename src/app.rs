use std::cell::RefCell;
use std::sync::{Arc, atomic::AtomicBool};
use std::thread;
use std::time::Duration;

use ratatui::{crossterm::event::KeyCode, widgets::ListState};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum FormField {
	#[default]
	Keys,
	Interval,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Pane {
	#[default]
	Toggles,
	Clicks,
}

#[derive(Debug)]
pub struct AppState {
	pub focus_pane: Pane,
	pub is_focused: Arc<AtomicBool>,
	pub show_add_form: bool,
	pub show_delete_confirm: bool,
	pub form_field: FormField,
	pub form_keys: String,
	pub form_interval: String,
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
	pub interval: u32,
	pub active: bool,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			focus_pane: Pane::Toggles,
			is_focused: Arc::new(AtomicBool::new(true)),
			show_add_form: false,
			show_delete_confirm: false,
			form_field: FormField::Keys,
			form_keys: String::new(),
			form_interval: String::new(),
			toggles: Vec::new(),
			clicks: Vec::new(),
			list_state: RefCell::new(ListState::default()),
			toggle_controller: crate::os_input::ToggleController::new(),
		}
	}
}

impl AppState {
	pub fn handle_key(&mut self, code: KeyCode) -> bool {
		if self.show_add_form {
			return self.handle_form_key(code);
		}

		if self.show_delete_confirm {
			return self.handle_delete_confirm_key(code);
		}

		match code {
			KeyCode::Char('Q') => true,
			KeyCode::Char('H') => {
				// TODO: display a help popup that explains keybinds
				false
			}
			KeyCode::Char('h') | KeyCode::Char('t') => {
				self.focus_pane = Pane::Toggles;
				false
			}
			KeyCode::Char('j') => {
				self.list_state.borrow_mut().select_next();
				false
			}
			KeyCode::Char('k') => {
				self.list_state.borrow_mut().select_previous();
				false
			}
			KeyCode::Char('l') | KeyCode::Char('c') => {
				self.focus_pane = Pane::Clicks;
				false
			}
			KeyCode::Char('a') => {
				self.show_add_form = true;
				self.form_field = FormField::Keys;
				self.form_keys.clear();
				self.form_interval.clear();
				false
			}
			KeyCode::Char('D') => {
				self.request_delete_selected();
				false
			}
			KeyCode::Enter => {
				self.toggle_status();

				false
			}
			_ => false,
		}
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

	fn handle_form_key(&mut self, code: KeyCode) -> bool {
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
				if self.focus_pane == Pane::Clicks {
					self.form_field = match self.form_field {
						FormField::Keys => FormField::Interval,
						FormField::Interval => FormField::Keys,
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
				}
				false
			}
			KeyCode::Char(ch) => {
				match self.form_field {
					FormField::Keys => self.form_keys.push(ch),
					FormField::Interval => {
						if ch.is_ascii_digit() {
							self.form_interval.push(ch);
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
	}

	fn submit_form(&mut self) {
		let keys = self.form_keys.trim().to_string();

		if keys.is_empty() {
			return;
		}

		match self.focus_pane {
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

				self.clicks.push(Click {
					keys,
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
			match self.focus_pane {
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

	pub fn pause_all_toggles(&mut self) {
		self.toggle_controller.pause_all();
	}

	pub fn resume_all_toggles(&mut self) {
		self.toggle_controller.resume_all(&self.toggles);
	}

	fn toggle_status(&mut self) {
		let index = self.list_state.borrow().selected();

		if let Some(i) = index {
			match self.focus_pane {
				Pane::Toggles => {
					if let Some(t) = self.toggles.get_mut(i) {
						t.active = !t.active;

						if t.active {
							let keys = t.keys.clone();
							self.toggle_controller.start_toggle(i, keys);
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

impl Drop for AppState {
	fn drop(&mut self) {
		self.toggle_controller.shutdown();
		thread::sleep(Duration::from_millis(100));
	}
}
