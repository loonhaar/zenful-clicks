use crate::appstate::{AppState, Pane};
use ratatui::{
	Frame,
	layout::{Constraint, Layout, Rect},
	style::{Color, Stylize},
	widgets::{Block, BorderType, Paragraph},
};

pub fn render_tabs(app: &AppState, frame: &mut Frame, area: Rect) {
	let h_constraints = [Constraint::Fill(1), Constraint::Fill(1)];
	let inner_layout = Layout::horizontal(h_constraints).split(area);

	let (toggle_color, clicks_color) = match app.active_pane {
		Pane::Toggles => (Color::Yellow, Color::White),
		Pane::Clicks => (Color::White, Color::Yellow),
	};

	let toggle_tab = Paragraph::new(" Toggles ").block(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(toggle_color),
	);

	let clicker_tab = Paragraph::new(" Clicks ").block(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(clicks_color),
	);

	frame.render_widget(toggle_tab, inner_layout[0]);
	frame.render_widget(clicker_tab, inner_layout[1]);
}
