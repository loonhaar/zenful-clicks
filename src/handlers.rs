use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::appstate::{AppState, FormField, Pane};
use crate::key_parser::{
	format_modifier_combo, format_modifier_only, merge_and_normalize,
	modifier_token_from_modifier_keycode,
};

pub fn handle_key(app: &mut AppState, event: KeyEvent) -> bool {
	let code = event.code;
	let modifiers = event.modifiers;

	if app.show_add_form {
		return handle_form_key(app, event);
	}

	if app.show_delete_confirm {
		return handle_delete_confirm_key(app, code);
	}

	let mut quit_application = false;

	match code {
		KeyCode::Char('q') if modifiers.contains(KeyModifiers::SHIFT) => {
			quit_application = true;
		}
		KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
			quit_application = true;
		}
		KeyCode::Char('h') if modifiers.contains(KeyModifiers::SHIFT) => {
			// TODO: display a help popup that explains keybinds
		}
		KeyCode::Char('h') | KeyCode::Char('t') => {
			app.active_pane = Pane::Toggles;
		}
		KeyCode::Char('j') => {
			app.list_state.borrow_mut().select_next();
		}
		KeyCode::Char('k') => {
			app.list_state.borrow_mut().select_previous();
		}
		KeyCode::Char('l') | KeyCode::Char('c') => {
			app.active_pane = Pane::Clicks;
		}
		KeyCode::Char('a') => {
			crate::actions::start_add_form(app);
		}
		KeyCode::Char('d') if modifiers.contains(KeyModifiers::SHIFT) => {
			crate::actions::request_delete_selected(app);
		}
		KeyCode::Enter => {
			crate::actions::toggle_status(app);
		}
		KeyCode::F(8) => {}
		_ => {}
	}

	quit_application
}

fn handle_form_key(app: &mut AppState, event: KeyEvent) -> bool {
	let code = event.code;
	let modifiers = event.modifiers;

	match code {
		KeyCode::Esc => {
			crate::actions::discard_form(app);
			false
		}
		KeyCode::Enter => {
			crate::actions::submit_form(app);
			false
		}
		KeyCode::Tab => {
			if app.active_pane == Pane::Clicks {
				app.form_field = match app.form_field {
					FormField::Keys => FormField::Interval,
					FormField::Interval => FormField::Activate,
					FormField::Activate => FormField::Keys,
				};
			}
			false
		}
		KeyCode::Backspace => {
			match app.form_field {
				FormField::Keys => {
					app.form_keys.pop();
				}
				FormField::Interval => {
					app.form_interval.pop();
				}
				FormField::Activate => {
					app.form_activate.pop();
				}
			}
			false
		}
		KeyCode::Null => {
			if app.form_field == FormField::Keys {
				if let Some(modifier_combo) = format_modifier_only(modifiers) {
					app.form_keys = merge_and_normalize(&app.form_keys, &modifier_combo);
				}
			} else if app.form_field == FormField::Activate {
				if let Some(modifier_combo) = format_modifier_only(modifiers) {
					app.form_activate = merge_and_normalize(&app.form_activate, &modifier_combo);
				}
			}
			false
		}
		KeyCode::Modifier(m) => {
			if app.form_field == FormField::Keys {
				if let Some(tok) = modifier_token_from_modifier_keycode(m) {
					app.form_keys = merge_and_normalize(&app.form_keys, &tok);
				}
			} else if app.form_field == FormField::Activate {
				if let Some(tok) = modifier_token_from_modifier_keycode(m) {
					app.form_activate = merge_and_normalize(&app.form_activate, &tok);
				}
			}
			false
		}
		KeyCode::Char(ch) if app.form_field == FormField::Keys => {
			if let Some(modifier_combo) = format_modifier_combo(modifiers, ch) {
				app.form_keys = merge_and_normalize(&app.form_keys, &modifier_combo);
			} else {
				app.form_keys.push(ch);
			}
			false
		}
		KeyCode::Char(ch) => {
			match app.form_field {
				FormField::Keys => {
					if let Some(modifier_combo) = format_modifier_combo(modifiers, ch) {
						app.form_keys = merge_and_normalize(&app.form_keys, &modifier_combo);
					} else {
						app.form_keys.push(ch);
					}
				}
				FormField::Interval => {
					if ch.is_ascii_digit() {
						app.form_interval.push(ch);
					}
				}
				FormField::Activate => {
					if let Some(modifier_combo) = format_modifier_combo(modifiers, ch) {
						app.form_activate =
							merge_and_normalize(&app.form_activate, &modifier_combo);
					} else {
						app.form_activate.push(ch);
					}
				}
			}
			false
		}
		_ => false,
	}
}

fn handle_delete_confirm_key(app: &mut AppState, code: KeyCode) -> bool {
	match code {
		KeyCode::Esc => {
			app.show_delete_confirm = false;
			false
		}
		KeyCode::Enter => {
			crate::actions::delete_selected(app);
			app.show_delete_confirm = false;
			false
		}
		_ => false,
	}
}
