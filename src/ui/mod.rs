use ratatui::{
	Frame,
	layout::{Constraint, HorizontalAlignment, Layout, Spacing},
	style::Color,
	symbols::merge::MergeStrategy,
	widgets::{Block, BorderType, Clear},
};

mod list;
mod popups;
mod tabs;

use crate::app::{AppState, Pane};

pub fn render(frame: &mut Frame, app: &AppState) {
	let (lock, _cvar) = &*app.focus_signal;
	let is_focused = *lock.lock().unwrap();

	let (outline_color, title) = if is_focused {
		(Color::White, " Zenful Clicks [PAUSE] ")
	} else {
		(Color::Green, " Zenful Clicks ")
	};

	let outline = Block::bordered()
		.border_type(BorderType::Rounded)
		.border_style(outline_color)
		.title(title)
		.title_alignment(HorizontalAlignment::Center);

	frame.render_widget(&outline, frame.area());

	let app_area = outline.inner(frame.area());
	let v_constraints = [Constraint::Length(3), Constraint::Min(3)];
	let outer_layout = Layout::vertical(v_constraints)
		.spacing(Spacing::Overlap(1))
		.horizontal_margin(1)
		.split(app_area);

	tabs::render_tabs(app, frame, outer_layout[0]);

	let main_pane = Block::bordered()
		.border_type(BorderType::Rounded)
		.border_style(Color::Yellow)
		.merge_borders(MergeStrategy::Fuzzy);

	frame.render_widget(&main_pane, outer_layout[1]);

	list::render_list(app, frame, main_pane.inner(outer_layout[1]));

	if app.show_add_form {
		let popup_block = Block::bordered()
			.border_type(BorderType::Rounded)
			.title_alignment(HorizontalAlignment::Center)
			.border_style(Color::Magenta);

		let popup_height = match app.active_pane {
			Pane::Toggles => 7,
			Pane::Clicks => 8,
		};

		let centered_area = frame
			.area()
			.centered(Constraint::Length(45), Constraint::Length(popup_height));

		// Clear the background for the popup
		frame.render_widget(Clear, centered_area);

		match app.active_pane {
			Pane::Toggles => popups::render_toggle_form(frame, popup_block, centered_area, app),
			Pane::Clicks => popups::render_click_form(frame, popup_block, centered_area, app),
		}
	} else if app.show_delete_confirm {
		let popup_block = Block::bordered()
			.border_type(BorderType::Rounded)
			.title_alignment(HorizontalAlignment::Center)
			.border_style(Color::Red);

		let centered_area = frame
			.area()
			.centered(Constraint::Length(42), Constraint::Length(6));

		frame.render_widget(Clear, centered_area);
		popups::render_delete_confirm(frame, popup_block, centered_area, app);
	}
}
