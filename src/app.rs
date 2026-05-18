use std::cell::RefCell;

use ratatui::{crossterm::event::KeyCode, widgets::ListState};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Pane {
	#[default]
	Toggles,
	Clicks,
}

#[derive(Debug, Default)]
pub struct AppState {
	pub focus_pane: Pane,
	pub show_add_form: bool,
	pub toggles: Vec<Toggle>,
	pub clicks: Vec<Click>,
	pub list_state: RefCell<ListState>,
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

impl AppState {
	pub fn handle_key(&mut self, code: KeyCode) -> bool {
		match code {
			KeyCode::Esc => true,
			KeyCode::Char('Q') => true,
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
			KeyCode::Char('n') => {
				self.show_add_form = true;
				false
			}
			KeyCode::Char('x') => {
				self.show_add_form = false;
				false
			}
			KeyCode::Enter => {
				self.toggle_status();
				false
			}
			_ => false,
		}
	}

	fn toggle_status(&mut self) {
		let index = self.list_state.borrow().selected();

		if let Some(i) = index {
			match self.focus_pane {
				Pane::Toggles => {
					if let Some(t) = self.toggles.get_mut(i) {
						t.active = !t.active
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
