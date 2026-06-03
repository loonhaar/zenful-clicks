pub fn parse_key_string(keys: &str) -> (Vec<enigo::Key>, Option<enigo::Key>) {
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

pub fn get_modifier_key(token: &str) -> Option<enigo::Key> {
	use enigo::Key;

	match token
		.trim_matches(|c| c == '<' || c == '>')
		.to_lowercase()
		.as_str()
	{
		"shift" => Some(Key::Shift),
		"ctrl" | "control" => Some(Key::Control),
		"alt" => Some(Key::Alt),
		"meta" | "cmd" | "super" | "win" | "windows" => Some(Key::Meta),
		_ => None,
	}
}

pub fn parse_main_key(token: &str) -> Option<enigo::Key> {
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

pub fn release_keys(
	enigo: &mut enigo::Enigo,
	modifier_keys: &[enigo::Key],
	main_key: Option<enigo::Key>,
) {
	use enigo::{Direction, Keyboard};

	if let Some(k) = main_key {
		let _ = enigo.key(k, Direction::Release);
	}
	for &m in modifier_keys.iter().rev() {
		let _ = enigo.key(m, Direction::Release);
	}
}

pub fn char_to_enigo_key(ch: char) -> Option<enigo::Key> {
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
