use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct ToggleController {
	active: HashMap<usize, Arc<Mutex<bool>>>,
}

impl ToggleController {
	pub fn new() -> Self {
		Self {
			active: HashMap::new(),
		}
	}

	pub fn start_toggle(
		&mut self,
		idx: usize,
		keys: String,
		focus_signal: &Arc<(Mutex<bool>, Condvar)>,
	) {
		if self.active.contains_key(&idx) {
			return;
		}

		let stop_signal = Arc::new(Mutex::new(false));
		let stop_signal_clone = Arc::clone(&stop_signal);
		let focus_signal_clone = Arc::clone(focus_signal);

		thread::spawn(move || {
			simulate_key_hold(&keys, stop_signal_clone, focus_signal_clone);
		});

		self.active.insert(idx, stop_signal);
	}

	pub fn stop_toggle(&mut self, idx: usize) {
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

fn simulate_key_hold(
	keys: &str,
	stop_signal: Arc<Mutex<bool>>,
	focus_signal: Arc<(Mutex<bool>, Condvar)>,
) {
	use enigo::{Direction, Enigo, Keyboard, Settings};

	let (modifiers, key_char) = parse_key_string(keys);
	let modifier_keys = get_modifier_keys(&modifiers);

	let Ok(mut enigo) = Enigo::new(&Settings::default()) else {
		return;
	};

	let main_key = char_to_enigo_key(key_char);

	let mut keys_pressed = false;

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
				if keys_pressed {
					if let Some(k) = main_key {
						let _ = enigo.key(k, Direction::Release);
					}
					for &m in modifier_keys.iter().rev() {
						let _ = enigo.key(m, Direction::Release);
					}
					keys_pressed = false;
				}

				while *focused {
					focused = cvar.wait(focused).unwrap();
				}
			}
		}

		if !keys_pressed {
			for &m in &modifier_keys {
				let _ = enigo.key(m, Direction::Press);
			}
			if let Some(k) = main_key {
				let _ = enigo.key(k, Direction::Press);
			}
			keys_pressed = true;
		}

		thread::sleep(Duration::from_millis(50));
	}

	if keys_pressed {
		if let Some(k) = main_key {
			let _ = enigo.key(k, Direction::Release);
		}
		for &m in modifier_keys.iter().rev() {
			let _ = enigo.key(m, Direction::Release);
		}
	}
}

fn parse_key_string(keys: &str) -> (Vec<&str>, char) {
	let parts: Vec<&str> = keys.split('+').collect();

	if parts.is_empty() {
		return (vec![], ' ');
	}

	let key_char = parts[parts.len() - 1].chars().next().unwrap_or(' ');
	let modifiers = parts[..parts.len() - 1].to_vec();

	(modifiers, key_char)
}

fn get_modifier_keys(modifiers: &[&str]) -> Vec<enigo::Key> {
	use enigo::Key;

	modifiers
		.iter()
		.filter_map(|&m| match m.to_lowercase().as_str() {
			"shift" => Some(Key::Shift),
			"ctrl" | "control" => Some(Key::Control),
			"alt" => Some(Key::Alt),
			"meta" | "cmd" | "super" => Some(Key::Meta),
			_ => None,
		})
		.collect()
}

fn char_to_enigo_key(ch: char) -> Option<enigo::Key> {
	use enigo::Key;

	match ch {
		'a'..='z' | 'A'..='Z' => Some(Key::Unicode(ch)),
		'0'..='9' => Some(Key::Unicode(ch)),
		' ' => Some(Key::Space),
		'\t' => Some(Key::Tab),
		'\n' | '\r' => Some(Key::Return),
		_ => None,
	}
}
