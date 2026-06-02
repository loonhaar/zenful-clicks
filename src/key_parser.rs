use ratatui::crossterm::event::{KeyModifiers, ModifierKeyCode};

pub fn normalize_key_combo(raw: &str) -> String {
	let mut modifiers: Vec<String> = Vec::new();
	let mut main_key: Option<String> = None;

	for part in raw.split('+') {
		let token = part.trim();
		if token.is_empty() {
			continue;
		}

		if let Some(mod_name) = normalize_modifier_name(token) {
			let tok = format!("<{}>", mod_name);
			if !modifiers.contains(&tok) {
				modifiers.push(tok);
			}
		} else {
			main_key = Some(format_key_token(token));
		}
	}

	let mut out = String::new();
	if !modifiers.is_empty() {
		out.push_str(&modifiers.join("+"));
	}
	if let Some(k) = main_key {
		if !out.is_empty() {
			out.push('+');
		}
		out.push_str(&k);
	}

	out
}

pub fn merge_and_normalize(buf: &str, combo: &str) -> String {
	if buf.trim().is_empty() {
		return normalize_key_combo(combo);
	}
	if combo.trim().is_empty() {
		return normalize_key_combo(buf);
	}

	let combined = format!("{}+{}", buf.trim(), combo.trim());
	normalize_key_combo(&combined)
}

fn format_key_token(token: &str) -> String {
	if let Some(modifier) = normalize_modifier_name(token) {
		return format!("<{}>", modifier);
	}

	if token.chars().count() == 1 {
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

pub fn format_modifier_combo(modifiers: KeyModifiers, ch: char) -> Option<String> {
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

pub fn format_modifier_only(modifiers: KeyModifiers) -> Option<String> {
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

pub fn modifier_token_from_modifier_keycode(m: ModifierKeyCode) -> Option<String> {
	use ModifierKeyCode::*;

	match m {
		LeftShift | RightShift => Some(String::from("<Shift>")),
		LeftControl | RightControl => Some(String::from("<Ctrl>")),
		LeftAlt | RightAlt => Some(String::from("<Alt>")),
		LeftSuper | RightSuper => Some(String::from("<Meta>")),
		_ => None,
	}
}
