mod click_controller;
mod toggle_controller;
mod common;

use self::{click_controller::ClickController, toggle_controller::ToggleController};
use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug)]
pub struct OsInputManager {
	toggle_controller: ToggleController,
	click_controller: ClickController,
}

impl OsInputManager {
	pub fn new() -> Self {
		Self {
			toggle_controller: ToggleController::new(),
			click_controller: ClickController::new(),
		}
	}

	pub fn start_toggle(
		&mut self,
		idx: usize,
		keys: String,
		focus_signal: &Arc<(Mutex<bool>, Condvar)>,
	) {
		self.toggle_controller.start_toggle(idx, keys, focus_signal);
	}

	pub fn stop_toggle(&mut self, idx: usize) {
		self.toggle_controller.stop_toggle(idx);
	}

	pub fn remove_toggle_index(&mut self, removed_idx: usize) {
		self.toggle_controller.remove_index(removed_idx);
	}

	pub fn start_click(
		&mut self,
		idx: usize,
		key: String,
		interval_ms: u32,
		focus_signal: &Arc<(Mutex<bool>, Condvar)>,
	) {
		self.click_controller
			.start_click(idx, key, interval_ms, focus_signal);
	}

	pub fn stop_click(&mut self, idx: usize) {
		self.click_controller.stop_click(idx);
	}

	pub fn remove_click_index(&mut self, removed_idx: usize) {
		self.click_controller.remove_index(removed_idx);
	}

	pub fn shutdown(&mut self) {
		self.toggle_controller.shutdown();
		self.click_controller.shutdown();
	}
}
