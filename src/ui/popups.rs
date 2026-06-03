use std::rc::Rc;

use ratatui::{
	Frame,
	layout::{Constraint, HorizontalAlignment, Layout, Rect},
	style::{Color, Style, Stylize},
	text::{Line, Span},
	widgets::{Block, Padding, Paragraph},
};

use crate::appstate::{AppState, FormField};

pub fn render_toggle_form(
	frame: &mut Frame,
	popup_block: Block,
	centered_area: Rect,
	app: &AppState,
) {
	let popup = popup_block.title(" Add a new toggle ");
	let inner_area = Layout::vertical([Constraint::Fill(1), Constraint::Length(2)])
		.split(popup.inner(centered_area));

	frame.render_widget(popup, centered_area);

	let key_p = Paragraph::new(render_input_line(
		"Set keys: ",
		&app.form_keys,
		true,
		Color::Cyan,
	))
	.block(Block::default().padding(Padding::uniform(1)));
	let bottom = layout_popup_buttons(inner_area[1]);

	let cancel = Paragraph::new("[Esc] Cancel   ")
		.fg(Color::Red)
		.alignment(HorizontalAlignment::Right);
	let confirm = Paragraph::new("[Enter] Confirm").fg(Color::Green);

	frame.render_widget(key_p, inner_area[0]);
	frame.render_widget(cancel, bottom[1]);
	frame.render_widget(confirm, bottom[2]);
}

pub fn render_click_form(
	frame: &mut Frame,
	popup_block: Block,
	centered_area: Rect,
	app: &AppState,
) {
	let popup = popup_block.title(" Add a new click ");
	let inner_area = Layout::vertical([Constraint::Fill(1), Constraint::Length(2)])
		.split(popup.inner(centered_area));

	frame.render_widget(popup, centered_area);

	let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
		.margin(1)
		.split(inner_area[0]);

	let keys_focused = app.form_field == FormField::Keys;
	let interval_focused = app.form_field == FormField::Interval;

	let key_p = Paragraph::new(render_input_line(
		"Set key: ",
		&app.form_keys,
		keys_focused,
		if keys_focused {
			Color::Green
		} else {
			Color::Reset
		},
	));

	let interval_p = Paragraph::new(render_input_line(
		"Interval (ms): ",
		&app.form_interval,
		interval_focused,
		if interval_focused {
			Color::Green
		} else {
			Color::Reset
		},
	));

	let bottom = layout_popup_buttons(inner_area[1]);

	let cancel = Paragraph::new("[Esc] Cancel   ")
		.fg(Color::Red)
		.alignment(HorizontalAlignment::Right);
	let confirm = Paragraph::new("[Enter] Confirm").fg(Color::Green);

	frame.render_widget(key_p, chunks[0]);
	frame.render_widget(interval_p, chunks[1]);
	frame.render_widget(cancel, bottom[1]);
	frame.render_widget(confirm, bottom[2]);
}

pub fn render_delete_confirm(
	frame: &mut Frame,
	popup_block: Block,
	centered_area: Rect,
	app: &AppState,
) {
	let popup = popup_block.title(" Delete selected ");
	let inner_area = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
		.split(popup.inner(centered_area));

	frame.render_widget(popup, centered_area);

	let message = match app.active_pane {
		crate::appstate::Pane::Toggles => "This will delete the selected toggle.",
		crate::appstate::Pane::Clicks => "This will delete the selected click.",
	};

	let body = Paragraph::new(message)
		.fg(Color::White)
		.alignment(HorizontalAlignment::Center)
		.block(Block::default().padding(Padding::uniform(1)));
	let bottom = layout_popup_buttons(inner_area[1]);

	let cancel = Paragraph::new("[Esc] Cancel   ")
		.fg(Color::Red)
		.alignment(HorizontalAlignment::Right);
	let confirm = Paragraph::new("[Enter] Delete").fg(Color::Green);

	frame.render_widget(body, inner_area[0]);
	frame.render_widget(cancel, bottom[1]);
	frame.render_widget(confirm, bottom[2]);
}

fn layout_popup_buttons(area: Rect) -> Rc<[Rect]> {
	Layout::horizontal([
		Constraint::Fill(1),
		Constraint::Length(15),
		Constraint::Length(15),
		Constraint::Fill(1),
	])
	.split(area)
}

fn render_input_line<'a>(label: &'a str, value: &'a str, focused: bool, color: Color) -> Line<'a> {
	let mut spans = vec![Span::raw(label), Span::styled(value.to_string(), color)];

	if focused {
		spans.push(Span::styled(
			" ",
			Style::default().fg(Color::Black).bg(color),
		));
	}

	Line::from(spans)
}
