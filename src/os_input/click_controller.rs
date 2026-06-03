use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crate::os_input::common::*;

#[derive(Debug)]
pub struct ClickController {
	active: HashMap<usize, Arc<Mutex<bool>>>,
}

impl ClickController {
	pub fn new() -> Self {
		Self {
			active: HashMap::new(),
		}
	}

	pub fn start_click(
		&mut self,
		idx: usize,
		key: String,
		interval_ms: u32,
		focus_signal: &Arc<(Mutex<bool>, Condvar)>,
	) {
		if self.active.contains_key(&idx) {
			return;
		}

		let stop_signal = Arc::new(Mutex::new(false));
		let stop_signal_clone = Arc::clone(&stop_signal);
		let focus_signal_clone = Arc::clone(focus_signal);
		let key_str = key.clone();

		thread::spawn(move || {
			simulate_click_loop(&key_str, interval_ms, stop_signal_clone, focus_signal_clone);
		});

		self.active.insert(idx, stop_signal);
	}

	pub fn stop_click(&mut self, idx: usize) {
		if let Some(sig) = self.active.remove(&idx) {
			if let Ok(mut v) = sig.lock() {
				*v = true;
			}
		}
	}

	pub fn remove_index(&mut self, removed_idx: usize) {
		let mut new_active = HashMap::new();
		for (&old_idx, sig) in self.active.iter() {
			let new_idx = if old_idx > removed_idx {
				old_idx - 1
			} else {
				old_idx
			};
			new_active.insert(new_idx, Arc::clone(sig));
		}
		self.active = new_active;
	}

	pub fn shutdown(&mut self) {
		for (_idx, sig) in self.active.iter() {
			if let Ok(mut v) = sig.lock() {
				*v = true;
			}
		}
	}
}

fn simulate_click_loop(
	key: &str,
	interval_ms: u32,
	stop_signal: Arc<Mutex<bool>>,
	focus_signal: Arc<(Mutex<bool>, Condvar)>,
) {
	use enigo::{Button, Direction, Enigo, Keyboard, Mouse, Settings};

	let (modifier_keys, main_key) = parse_key_string(key);

	let lower = key.trim().to_lowercase();
	let mouse_button = if lower == "left" {
		Some("left")
	} else if lower == "right" {
		Some("right")
	} else if lower == "middle" {
		Some("middle")
	} else {
		None
	};

	let Ok(mut enigo) = Enigo::new(&Settings::default()) else {
		return;
	};

	loop {
		if let Ok(signal) = stop_signal.lock() {
			if *signal {
				break;
			}
		}

		{
			let (lock, cvar) = &*focus_signal;
			let mut focused = lock.lock().unwrap();

			if *focused {
				while *focused {
					focused = cvar.wait(focused).unwrap();
				}
			}
		}

		if let Some(btn) = mouse_button {
			let button = match btn {
				"left" => Button::Left,
				"right" => Button::Right,
				"middle" => Button::Middle,
				_ => Button::Left,
			};
			let _ = enigo.button(button, Direction::Click);
		} else {
			for &m in &modifier_keys {
				let _ = enigo.key(m, Direction::Press);
			}
			if let Some(k) = main_key {
				let _ = enigo.key(k, Direction::Press);
				let _ = enigo.key(k, Direction::Release);
			}
			for &m in modifier_keys.iter().rev() {
				let _ = enigo.key(m, Direction::Release);
			}
		}

		std::thread::sleep(std::time::Duration::from_millis(interval_ms as u64));
	}
}
