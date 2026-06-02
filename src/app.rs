use std::cell::RefCell;
use std::sync::Arc;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use ratatui::{
	crossterm::event::KeyEvent,
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
		crate::handlers::handle_key(self, event)
	}
}

impl Drop for AppState {
	fn drop(&mut self) {
		self.toggle_controller.shutdown();
		thread::sleep(Duration::from_millis(100));
	}
}
