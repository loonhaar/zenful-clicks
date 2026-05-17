use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event, KeyCode},
	layout::{Constraint, HorizontalAlignment, Layout, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
	widgets::{Block, BorderType, Paragraph},
};

fn main() -> Result<()> {
	let mut app = AppState::default();
	color_eyre::install()?;

	let terminal = ratatui::init();
	let result = run(terminal, &mut app);

	ratatui::restore();
	result
}

#[derive(Debug, Default, PartialEq)]
enum Pane {
	#[default]
	Toggles,
	Clicks,
}

#[derive(Debug, Default)]
struct AppState {
	focus_pane: Pane,
	//toggles: Vec<Toggle>,
	//clicks: Vec<Click>,
}

//struct Toggle {
//	key: ???,
//	active: bool,
//}

//struct Click {
//	key: ???,
//	interval: ???,
//	active: bool,
//}

fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
	loop {
		// Rendering
		terminal.draw(|t| app.render(t))?;

		// Input handling
		if let Event::Key(key) = event::read()?
			&& key.kind == event::KeyEventKind::Press
			&& app.handle_key(key.code)
		{
			break;
		}
	}

	Ok(())
}

impl AppState {
	fn handle_key(&mut self, code: KeyCode) -> bool {
		match code {
			event::KeyCode::Esc => true,
			event::KeyCode::Char('Q') => true,
			event::KeyCode::Char('T') => {
				self.focus_pane = Pane::Toggles;
				false
			}
			event::KeyCode::Char('C') => {
				self.focus_pane = Pane::Clicks;
				false
			}
			_ => false,
		}
	}

	fn render(&mut self, frame: &mut Frame) {
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

		let toggle_color;
		let clicks_color;

		if self.focus_pane == Pane::Toggles {
			toggle_color = Color::Yellow;
			clicks_color = Color::Gray;
		} else {
			toggle_color = Color::Gray;
			clicks_color = Color::Yellow;
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

		let main_pane = Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(Color::Yellow)
			.merge_borders(MergeStrategy::Fuzzy);

		//let main_pane_area = main_pane.inner(outer_layout[1]);
		//let main_pane_layout = Layout::vertical([Constraint::Length(3)]);

		frame.render_widget(toggle_tab, inner_layout[0]);

		frame.render_widget(clicker_tab, inner_layout[1]);

		frame.render_widget(main_pane, outer_layout[1]);
	}
}
