use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal,
	crossterm::event::{self, Event},
};

mod app;
mod ui;

use app::AppState;

fn main() -> Result<()> {
	let mut app = AppState::default();
	color_eyre::install()?;

	let terminal = ratatui::init();
	let result = run(terminal, &mut app);

	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
	loop {
		terminal.draw(|t| crate::ui::render(t, &app))?;

		if let Event::Key(key) = event::read()?
			&& key.kind == event::KeyEventKind::Press
			&& app.handle_key(key.code)
		{
			break;
		}
	}

	Ok(())
}
