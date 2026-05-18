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
	let [layout] = Layout::vertical([Constraint::Fill(1)])
		.margin(1)
		.areas(area);

	let list_items: Vec<ListItem> = app
		.toggles
		.iter()
		.map(|toggle| {
			// Format status
			let (status_str, status_color) = if toggle.active {
				("ACTIVE", Color::Green)
			} else {
				("Inactive", Color::Gray)
			};

			// Build lines of a single item on the list
			let lines = vec![
				Line::from(vec![Span::raw("Keys: "), Span::raw(&toggle.key)]).fg(Color::Gray),
				Line::from(status_str).fg(status_color),
				Line::from("\n"),
				Line::from("─".repeat(layout.width as usize)).fg(Color::Yellow),
				Line::from("\n"),
			];

			ListItem::new(Text::from(lines))
		})
		.collect();

	let list = List::new(list_items).fg(Color::Gray);

	frame.render_widget(list, layout);
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

			// Build lines of a single item on the list
			let lines = vec![
				Line::from(vec![Span::raw("Keys: "), Span::raw(&click.key)]).fg(Color::Gray),
				Line::from(vec![
					Span::raw("Interval: ").fg(Color::Gray),
					Span::raw(format!("{} ms", click.interval.to_string())).fg(Color::Magenta),
				]),
				Line::from(status_str).fg(status_color),
				Line::from("\n"),
				Line::from("─".repeat(layout.width as usize)).fg(Color::Yellow),
				Line::from("\n"),
			];

			ListItem::new(Text::from(lines))
		})
		.collect();

	let list = List::new(list_items).fg(Color::Gray);

	frame.render_widget(list, layout);
}
