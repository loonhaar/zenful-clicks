use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct ToggleController {
	active: HashMap<usize, Arc<Mutex<bool>>>,
	paused: HashSet<usize>,
}

impl ToggleController {
	pub fn new() -> Self {
		Self {
			active: HashMap::new(),
			paused: HashSet::new(),
		}
	}

	pub fn start_toggle(&mut self, idx: usize, keys: String) {
		if self.active.contains_key(&idx) {
			return;
		}

		let stop_signal = Arc::new(Mutex::new(false));
		let stop_signal_clone = Arc::clone(&stop_signal);

		thread::spawn(move || {
			simulate_key_hold(&keys, stop_signal_clone);
		});

		self.active.insert(idx, stop_signal);
		self.paused.remove(&idx);
	}

	pub fn stop_toggle(&mut self, idx: usize) {
		if let Some(sig) = self.active.remove(&idx) {
			if let Ok(mut v) = sig.lock() {
				*v = true;
			}
		}
	}

	pub fn pause_all(&mut self) {
		for (idx, sig) in self.active.drain() {
			if let Ok(mut v) = sig.lock() {
				*v = true;
			}
			self.paused.insert(idx);
		}
	}

	pub fn resume_all(&mut self, toggles: &[crate::app::Toggle]) {
		for idx in self.paused.drain() {
			if let Some(t) = toggles.get(idx) {
				let keys = t.keys.clone();
				let stop_signal = Arc::new(Mutex::new(false));
				let stop_signal_clone = Arc::clone(&stop_signal);

				thread::spawn(move || {
					simulate_key_hold(&keys, stop_signal_clone);
				});

				self.active.insert(idx, stop_signal);
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

		let new_paused: HashSet<usize> = self
			.paused
			.iter()
			.map(|&old_idx| {
				if old_idx > removed_idx {
					old_idx - 1
				} else {
					old_idx
				}
			})
			.collect();
		self.paused = new_paused;
	}

	pub fn shutdown(&mut self) {
		for (_idx, sig) in self.active.iter() {
			if let Ok(mut v) = sig.lock() {
				*v = true;
			}
		}
	}
}

fn simulate_key_hold(keys: &str, stop_signal: Arc<Mutex<bool>>) {
	use enigo::{Direction, Enigo, Keyboard, Settings};

	let (modifiers, key_char) = parse_key_string(keys);
	let modifier_keys = get_modifier_keys(&modifiers);

	let Ok(mut enigo) = Enigo::new(&Settings::default()) else {
		return;
	};

	for &m in &modifier_keys {
		let _ = enigo.key(m, Direction::Press);
	}

	let main_key = char_to_enigo_key(key_char);
	if let Some(k) = main_key {
		let _ = enigo.key(k, Direction::Press);
	}

	loop {
		if let Ok(signal) = stop_signal.lock() {
			if *signal {
				break;
			}
		}
		thread::sleep(Duration::from_millis(50));
	}

	if let Some(k) = main_key {
		let _ = enigo.key(k, Direction::Release);
	}
	for &m in modifier_keys.iter().rev() {
		let _ = enigo.key(m, Direction::Release);
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
