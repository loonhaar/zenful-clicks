use ratatui::{
	Frame,
	layout::{Constraint, Layout, Rect},
	style::{Color, Stylize},
	text::{Line, Span, Text},
	widgets::{List, ListItem},
};

use crate::app::{AppState, Pane};

pub fn render_list(app: &AppState, frame: &mut Frame, area: Rect) {
	match app.focus_pane {
		Pane::Toggles => toggles_list(app, frame, area),
		Pane::Clicks => clicks_list(app, frame, area),
	}
}

fn toggles_list(app: &AppState, frame: &mut Frame, area: Rect) {
	let pointer = ">> ";
	let pointer_len = pointer.len() as u16;

	let layout = Layout::horizontal([Constraint::Min(0), Constraint::Length(pointer_len)])
		.margin(1)
		.split(area);

	let list_area = layout[0];

	let list_items: Vec<ListItem> = app
		.toggles
		.iter()
		.map(|toggle| {
			// Format status
			let (status_str, status_color) = if toggle.active {
				("ACTIVE", Color::Green)
			} else {
				("Inactive", Color::White)
			};

			let separator = "─".repeat(list_area.width as usize);

			// Build lines of a single item on the list
			let lines = vec![
				Line::from(vec![Span::raw("Keys: "), Span::raw(&toggle.keys)]).fg(Color::Reset),
				Line::from(status_str).fg(status_color),
				Line::from(""),
				Line::from(separator).fg(Color::Yellow),
				Line::from(""),
			];

			ListItem::new(Text::from(lines))
		})
		.collect();

	let list = List::new(list_items)
		.highlight_symbol(pointer)
		.fg(Color::Cyan);

	let mut list_state = app.list_state.borrow_mut();
	frame.render_stateful_widget(list, list_area, &mut *list_state);
}

fn clicks_list(app: &AppState, frame: &mut Frame, area: Rect) {
	let [layout] = Layout::vertical([Constraint::Fill(1)])
		.margin(1)
		.areas(area);

	let list_items: Vec<ListItem> = app
		.clicks
		.iter()
		.map(|click| {
			// Format status
			let (status_str, status_color) = if click.active {
				("ACTIVE", Color::Green)
			} else {
				("Inactive", Color::Gray)
			};

			let separator_width = layout.width.saturating_sub(3) as usize;

			// Build lines of a single item on the list
			let lines = vec![
				Line::from(vec![Span::raw("Keys: "), Span::raw(&click.keys)]).fg(Color::Gray),
				Line::from(vec![
					Span::raw("Interval: ").fg(Color::Gray),
					Span::raw(format!("{} ms", click.interval.to_string())).fg(Color::Magenta),
				]),
				Line::from(status_str).fg(status_color),
				Line::from("\n"),
				Line::from("─".repeat(separator_width)).fg(Color::Yellow),
				Line::from("\n"),
			];

			ListItem::new(Text::from(lines))
		})
		.collect();

	let list = List::new(list_items)
		.fg(Color::Gray)
		.highlight_symbol(">> ")
		.fg(Color::Cyan);

	let mut list_state = app.list_state.borrow_mut();
	frame.render_stateful_widget(list, layout, &mut *list_state);
}
