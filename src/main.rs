use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event, KeyCode},
	layout::{Constraint, HorizontalAlignment, Layout, Rect, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
	widgets::{Block, BorderType, Clear, Padding, Paragraph},
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
	show_add_form: bool,
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
			event::KeyCode::Char('n') => {
				self.show_add_form = true;
				false
			}
			event::KeyCode::Char('x') => {
				self.show_add_form = false;
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

		match self.focus_pane {
			Pane::Toggles => {
				toggle_color = Color::Yellow;
				clicks_color = Color::Gray;
			}
			Pane::Clicks => {
				toggle_color = Color::Gray;
				clicks_color = Color::Yellow;
			}
		}

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

		// This will be used to draw the lists of jobs that have been set up
		//let main_pane_area = main_pane.inner(outer_layout[1]);
		//let main_pane_layout = Layout::vertical([Constraint::Length(3)]);

		frame.render_widget(toggle_tab, inner_layout[0]);

		frame.render_widget(clicker_tab, inner_layout[1]);

		frame.render_widget(main_pane, outer_layout[1]);

		if self.show_add_form {
			let popup_block = Block::bordered()
				.border_type(BorderType::Rounded)
				.title_alignment(HorizontalAlignment::Center)
				.border_style(Color::Magenta);

			let centered_area = frame
				.area()
				.centered(Constraint::Length(45), Constraint::Length(7));

			// Clear the background for the popup
			frame.render_widget(Clear, centered_area);

			match self.focus_pane {
				Pane::Toggles => self.add_toggle_form(frame, popup_block, centered_area),
				Pane::Clicks => self.add_click_form(frame, popup_block, centered_area),
			}
		}
	}

	fn add_toggle_form(&mut self, frame: &mut Frame, popup_block: Block, centered_area: Rect) {
		let popup = popup_block.title(" Add a new toggle ");

		let inner_area = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
			.split(popup.inner(centered_area));

		frame.render_widget(popup, centered_area);

		let key_p =
			Paragraph::new("Set key: ").block(Block::default().padding(Padding::uniform(1))); //GG

		let bottom = Layout::horizontal([
			Constraint::Fill(1),
			Constraint::Length(12), // TODO: magic numbers
			Constraint::Length(15),
			Constraint::Fill(1),
		])
		.split(inner_area[1]);

		let x = Paragraph::new("[x] Cancel  ")
			.fg(Color::Red)
			.alignment(HorizontalAlignment::Right);
		let enter = Paragraph::new("[Enter] Confirm").fg(Color::Green);

		frame.render_widget(key_p, inner_area[0]);
		frame.render_widget(x, bottom[1]);
		frame.render_widget(enter, bottom[2]);
	}

	fn add_click_form(&mut self, frame: &mut Frame, popup_block: Block, centered_area: Rect) {
		let popup = popup_block.title(" Add a new click ");

		let inner_area = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
			.split(popup.inner(centered_area));

		frame.render_widget(popup, centered_area);

		let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
			.margin(1)
			.split(inner_area[0]);

		let key_p = Paragraph::new("Set key: ");
		let interval_p = Paragraph::new("Interval (ms): ");

		let bottom = Layout::horizontal([
			Constraint::Fill(1),
			Constraint::Length(12), // TODO: magic numbers
			Constraint::Length(15),
			Constraint::Fill(1),
		])
		.split(inner_area[1]);

		let x = Paragraph::new("[x] Cancel  ")
			.fg(Color::Red)
			.alignment(HorizontalAlignment::Right);
		let enter = Paragraph::new("[Enter] Confirm").fg(Color::Green);

		frame.render_widget(key_p, chunks[0]);
		frame.render_widget(interval_p, chunks[1]);
		frame.render_widget(x, bottom[1]);
		frame.render_widget(enter, bottom[2]);
	}
}
