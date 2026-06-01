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

	let (modifier_keys, main_key) = parse_key_string(keys);

	let Ok(mut enigo) = Enigo::new(&Settings::default()) else {
		return;
	};

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
					release_keys(&mut enigo, &modifier_keys, main_key);
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
		release_keys(&mut enigo, &modifier_keys, main_key);
	}
}

fn parse_key_string(keys: &str) -> (Vec<enigo::Key>, Option<enigo::Key>) {
	let mut modifiers = Vec::new();
	let mut main_key = None;

	for part in keys.split('+') {
		let token = part.trim();
		if token.is_empty() {
			continue;
		}

		if let Some(modifier) = get_modifier_key(token) {
			modifiers.push(modifier);
			continue;
		}

		if main_key.is_none() {
			main_key = parse_main_key(token);
		}
	}

	(modifiers, main_key)
}

fn get_modifier_key(token: &str) -> Option<enigo::Key> {
	use enigo::Key;

	match token.trim_matches(|c| c == '<' || c == '>').to_lowercase().as_str() {
		"shift" => Some(Key::Shift),
		"ctrl" | "control" => Some(Key::Control),
		"alt" => Some(Key::Alt),
		"meta" | "cmd" | "super" | "win" | "windows" => Some(Key::Meta),
		_ => None,
	}
}

fn parse_main_key(token: &str) -> Option<enigo::Key> {
	use enigo::Key;

	let lower = token.trim_matches(|c| c == '<' || c == '>').to_lowercase();

	match lower.as_str() {
		"space" => Some(Key::Space),
		"tab" => Some(Key::Tab),
		"enter" | "return" => Some(Key::Return),
		"esc" | "escape" => None,
		_ if token.chars().count() == 1 => token.chars().next().and_then(char_to_enigo_key),
		_ => None,
	}
}

fn release_keys(enigo: &mut enigo::Enigo, modifier_keys: &[enigo::Key], main_key: Option<enigo::Key>) {
	use enigo::{Direction, Keyboard};

	if let Some(k) = main_key {
		let _ = enigo.key(k, Direction::Release);
	}
	for &m in modifier_keys.iter().rev() {
		let _ = enigo.key(m, Direction::Release);
	}
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
