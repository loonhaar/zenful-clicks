use ratatui::{
	Frame,
	layout::{Constraint, Layout, Rect},
	style::{Color, Stylize},
	text::{Line, Span, Text},
	widgets::{List, ListItem},
};

use crate::appstate::{AppState, Pane};

pub fn render_list(app: &AppState, frame: &mut Frame, area: Rect) {
	let pointer = ">> ";
	let layout = Layout::horizontal([Constraint::Min(0), Constraint::Length(pointer.len() as u16)])
		.margin(1)
		.split(area);
	let list_area = layout[0];

	let list_items: Vec<ListItem> = match app.active_pane {
		Pane::Toggles => make_toggle_items(app, list_area.width as usize),
		Pane::Clicks => make_click_items(app, list_area.width as usize),
	};

	let list = List::new(list_items)
		.highlight_symbol(pointer)
		.fg(Color::Cyan);

	let mut list_state = app.list_state.borrow_mut();
	frame.render_stateful_widget(list, list_area, &mut *list_state);
}

fn make_toggle_items(app: &AppState, width: usize) -> Vec<ListItem<'_>> {
	app.toggles
		.iter()
		.map(|t| {
			let (status, color) = if t.active {
				("ACTIVE", Color::Green)
			} else {
				("Inactive", Color::White)
			};
			let lines = vec![
				Line::from(vec![Span::raw("Keys: "), Span::raw(&t.keys)]).fg(Color::Reset),
				Line::from(status).fg(color),
				Line::from(""),
				Line::from("─".repeat(width)).fg(Color::Yellow),
				Line::from(""),
			];
			ListItem::new(Text::from(lines))
		})
		.collect()
}

fn make_click_items(app: &AppState, width: usize) -> Vec<ListItem<'_>> {
	let sep_width = width.saturating_sub(3);
	app.clicks
		.iter()
		.map(|c| {
			let (status, color) = if c.active {
				("ACTIVE", Color::Green)
			} else {
				("Inactive", Color::White)
			};
			let lines = vec![
				Line::from(vec![Span::raw("Keys: "), Span::raw(&c.key)]).fg(Color::Reset),
				Line::from(vec![
					Span::raw("Interval: ").fg(Color::Reset),
					Span::raw(format!("{} ms", c.interval)).fg(Color::Magenta),
				]),
				Line::from(vec![Span::raw("Activate: "), Span::raw(&c.activate)]).fg(Color::Reset),
				Line::from(status).fg(color),
				Line::from("\n"),
				Line::from("─".repeat(sep_width)).fg(Color::Yellow),
				Line::from("\n"),
			];
			ListItem::new(Text::from(lines))
		})
		.collect()
}
