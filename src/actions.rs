use crate::appstate::{AppState, Click, FormField, Pane, Toggle};
use crate::key_parser::normalize_key_combo;

pub fn start_add_form(app: &mut AppState) {
	app.show_add_form = true;
	app.form_field = FormField::Keys;
	app.form_keys.clear();
	app.form_interval.clear();
}

pub fn discard_form(app: &mut AppState) {
	app.show_add_form = false;
	app.form_field = FormField::Keys;
	app.form_keys.clear();
	app.form_interval.clear();
}

pub fn submit_form(app: &mut AppState) {
	let keys = normalize_key_combo(app.form_keys.trim());

	if keys.is_empty() {
		return;
	}

	match app.active_pane {
		Pane::Toggles => {
			app.toggles.push(Toggle {
				keys,
				active: false,
			});
			let last = app.toggles.len().saturating_sub(1);
			app.list_state.borrow_mut().select(Some(last));
			discard_form(app);
		}
		Pane::Clicks => {
			let Ok(interval) = app.form_interval.trim().parse::<u32>() else {
				return;
			};

			app.clicks.push(Click {
				key: keys,
				interval,
				active: false,
			});
			let last = app.clicks.len().saturating_sub(1);
			app.list_state.borrow_mut().select(Some(last));
			discard_form(app);
		}
	}

	app.save_config();
}

pub fn delete_selected(app: &mut AppState) {
	let index = app.list_state.borrow().selected();
	if let Some(i) = index {
		match app.active_pane {
			Pane::Toggles => {
				if i < app.toggles.len() {
					app.toggle_controller.stop_toggle(i);

					app.toggles.remove(i);
					let len = app.toggles.len();
					if len == 0 {
						app.list_state.borrow_mut().select(None);
					} else {
						let new = if i >= len { len - 1 } else { i };
						app.list_state.borrow_mut().select(Some(new));
					}

					app.toggle_controller.remove_toggle_index(i);
				}
			}
			Pane::Clicks => {
				if i < app.clicks.len() {
					app.clicks.remove(i);
					app.click_controller.stop_click(i);
					let len = app.clicks.len();
					if len == 0 {
						app.list_state.borrow_mut().select(None);
					} else {
						let new = if i >= len { len - 1 } else { i };
						app.list_state.borrow_mut().select(Some(new));
					}
					app.click_controller.remove_click_index(i);
				}
			}
		}

		app.save_config();
	}
}

pub fn request_delete_selected(app: &mut AppState) {
	if app.list_state.borrow().selected().is_some() {
		app.show_delete_confirm = true;
	}
}

pub fn toggle_status(app: &mut AppState) {
	let index = app.list_state.borrow().selected();

	if let Some(i) = index {
		match app.active_pane {
			Pane::Toggles => {
				if let Some(t) = app.toggles.get_mut(i) {
					t.active = !t.active;

					if t.active {
						let keys = t.keys.clone();
						app.toggle_controller
							.start_toggle(i, keys, &app.focus_signal);
					} else {
						app.toggle_controller.stop_toggle(i);
					}
				}
			}
			Pane::Clicks => {
				if let Some(c) = app.clicks.get_mut(i) {
					c.active = !c.active;
					if c.active {
						let key = c.key.clone();
						let interval = c.interval;
						app.click_controller
							.start_click(i, key, interval, &app.focus_signal);
					} else {
						app.click_controller.stop_click(i);
					}
				}
			}
		}
	}
}
