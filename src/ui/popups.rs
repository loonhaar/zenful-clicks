use std::rc::Rc;

use ratatui::{
	Frame,
	layout::{Constraint, HorizontalAlignment, Layout, Rect},
	style::{Color, Stylize},
	widgets::{Block, Padding, Paragraph},
};

pub fn render_toggle_form(frame: &mut Frame, popup_block: Block, centered_area: Rect) {
	let popup = popup_block.title(" Add a new toggle ");
	let inner_area = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
		.split(popup.inner(centered_area));

	frame.render_widget(popup, centered_area);

	let key_p = Paragraph::new("Set key: ").block(Block::default().padding(Padding::uniform(1))); //GG
	let bottom = layout_popup_buttons(inner_area[1]);

	let cancel = Paragraph::new("[x] Cancel  ")
		.fg(Color::Red)
		.alignment(HorizontalAlignment::Right);
	let confirm = Paragraph::new("[Enter] Confirm").fg(Color::Green);

	frame.render_widget(key_p, inner_area[0]);
	frame.render_widget(cancel, bottom[1]);
	frame.render_widget(confirm, bottom[2]);
}

pub fn render_click_form(frame: &mut Frame, popup_block: Block, centered_area: Rect) {
	let popup = popup_block.title(" Add a new click ");
	let inner_area = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
		.split(popup.inner(centered_area));

	frame.render_widget(popup, centered_area);

	let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
		.margin(1)
		.split(inner_area[0]);

	let key_p = Paragraph::new("Set key: ");
	let interval_p = Paragraph::new("Interval (ms): ");
	let bottom = layout_popup_buttons(inner_area[1]);

	let cancel = Paragraph::new("[x] Cancel  ")
		.fg(Color::Red)
		.alignment(HorizontalAlignment::Right);
	let confirm = Paragraph::new("[Enter] Confirm").fg(Color::Green);

	frame.render_widget(key_p, chunks[0]);
	frame.render_widget(interval_p, chunks[1]);
	frame.render_widget(cancel, bottom[1]);
	frame.render_widget(confirm, bottom[2]);
}

fn layout_popup_buttons(area: Rect) -> Rc<[Rect]> {
	Layout::horizontal([
		Constraint::Fill(1),
		Constraint::Length(12),
		Constraint::Length(15),
		Constraint::Fill(1),
	])
	.split(area)
}
