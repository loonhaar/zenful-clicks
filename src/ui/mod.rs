use ratatui::{
	Frame,
	layout::{Constraint, HorizontalAlignment, Layout, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
	widgets::{Block, BorderType, Clear},
};

mod list;
mod popups;
mod tabs;

use crate::app::{AppState, Pane};

pub fn render(frame: &mut Frame, app: &AppState) {
	let outline = Block::bordered()
		.border_type(BorderType::Rounded)
		.fg(Color::Green)
		.title(" Zenful Clicks ")
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
		.fg(Color::Yellow)
		.merge_borders(MergeStrategy::Fuzzy);

	frame.render_widget(&main_pane, outer_layout[1]);

	list::render_list(app, frame, main_pane.inner(outer_layout[1]));

	if app.show_add_form {
		let popup_block = Block::bordered()
			.border_type(BorderType::Rounded)
			.title_alignment(HorizontalAlignment::Center)
			.border_style(Color::Magenta);

		let centered_area = frame
			.area()
			.centered(Constraint::Length(45), Constraint::Length(7));

		// Clear the background for the popup
		frame.render_widget(Clear, centered_area);

		match app.focus_pane {
			Pane::Toggles => popups::render_toggle_form(frame, popup_block, centered_area),
			Pane::Clicks => popups::render_click_form(frame, popup_block, centered_area),
		}
	}
}
