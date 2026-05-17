use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event},
	layout::{Constraint, HorizontalAlignment, Layout, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
	widgets::{Block, BorderType, Paragraph},
};

fn main() -> Result<()> {
	let mut app = AppState::default();
	color_eyre::install()?;

	let terminal = ratatui::init();
	let result = run(terminal);

	ratatui::restore();
	result
}

#[derive(Debug, Default)]
enum Pane {
	#[default]
	Toggles,
	Clicks,
}

#[derive(Debug, Default)]
struct AppState {
	focus_pane: Pane,
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
	loop {
		// Rendering
		terminal.draw(render)?;

		// Input handling
		if let Event::Key(key) = event::read()? {
			match key.code {
				event::KeyCode::Char('q') => break,
				event::KeyCode::Esc => break,
				_ => {}
			}
		}
	}

	Ok(())
}

fn render(frame: &mut Frame) {
	let outline = Block::bordered()
		.border_type(BorderType::Rounded)
		.fg(Color::Green)
		.title(" Zenful Clicks ")
		.title_alignment(HorizontalAlignment::Center);

	frame.render_widget(&outline, frame.area());

	// Layouts
	let app_area = outline.inner(frame.area());
	let v_constraints = [Constraint::Length(3), Constraint::Min(3)];
	let outer_layout = Layout::vertical(v_constraints)
		.spacing(Spacing::Overlap(1))
		.split(app_area);

	let h_constraints = [Constraint::Fill(1), Constraint::Fill(1)];
	let inner_layout = Layout::horizontal(h_constraints).split(outer_layout[0]);

	// Widgets
	let toggle_tab = Paragraph::new(" Toggles ").block(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(Color::Yellow),
	);

	let clicker_tab = Paragraph::new(" Clicks ").block(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(Color::Gray),
	);

	let main_pane = Block::bordered()
		.border_type(BorderType::Rounded)
		.fg(Color::Yellow)
		.merge_borders(MergeStrategy::Fuzzy);

	let main_pane_layout = main_pane.inner(outer_layout[1]);

	frame.render_widget(toggle_tab, inner_layout[0]);

	frame.render_widget(clicker_tab, inner_layout[1]);

	frame.render_widget(main_pane, outer_layout[1]);
}
