use ratatui::{
	Frame,
	layout::{Constraint, HorizontalAlignment, Layout, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
	text::{Line, Span},
	widgets::{Block, BorderType, Clear, Paragraph},
};

mod list;
mod popups;
mod tabs;

use crate::appstate::{AppState, Pane};

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
	} else if app.show_help {
		let help_window = Block::bordered()
			.border_type(BorderType::Rounded)
			.title(" Help ")
			.border_style(Color::Yellow);

		let help_area = Layout::horizontal([Constraint::Percentage(100)])
			.horizontal_margin(1)
			.split(app_area)[0];

		frame.render_widget(Clear, help_area);

		let inner_area = help_window.inner(help_area);

		let text_area = Layout::horizontal([Constraint::Percentage(100)])
			.horizontal_margin(2)
			.split(inner_area)[0];

		let help_text = vec![
			Line::raw(""),
			Line::from(vec![
				Span::raw("Zenful Clicks - keyboard input helper").bold(),
			]),
			Line::raw(""),
			Line::from(vec![Span::raw("How to use the app")]),
			Line::from(vec![Span::raw("    H       - Display help.")]),
			Line::from(vec![Span::raw(
				"              To hide the help press any key except j and k",
			)]),
			Line::raw(""),
			Line::from(vec![Span::raw("    Q or")]),
			Line::from(vec![Span::raw("    Ctrl+c  - Quit.")]),
			Line::raw(""),
			Line::from(vec![Span::raw("    t       - Go to the Toggles tab.")]),
			Line::raw(""),
			Line::from(vec![Span::raw("    c       - Go to the Clicks tab.")]),
			Line::raw(""),
			Line::from(vec![Span::raw("    h, l    - Swithc between tabs.")]),
			Line::raw(""),
			Line::from(vec![Span::raw(
				"    j, k    - Select next/previous action on the list or scroll help up and down.",
			)]),
			Line::raw(""),
			Line::from(vec![Span::raw(
				"    a       - Create and configure a new action.",
			)]),
			Line::raw(""),
			Line::from(vec![Span::raw("    D       - Delete selected action.")]),
			Line::raw(""),
			Line::from(vec![Span::raw(
				"    Enter   - Toggle status of the selected action.",
			)]),
			Line::raw(""),
			Line::from(vec![Span::raw(
				"All active actions are automatically paused if the terminal window is focused.",
			)]),
			Line::from(vec![Span::raw(
				"Active actions will only be unpaused once the terminal loses focus.",
			)]),
		];

		let help_p = Paragraph::new(help_text).scroll((app.help_scroll, 0));

		frame.render_widget(help_window, help_area);
		frame.render_widget(help_p, text_area);
	}
}
