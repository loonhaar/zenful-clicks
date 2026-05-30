use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal,
	crossterm::event::{self, Event},
};

mod app;
mod ui;
mod os_input;

use app::AppState;

fn main() -> Result<()> {
	let mut app = AppState::default();
	app.list_state.borrow_mut().select_first();
	color_eyre::install()?;

	let terminal = ratatui::init();
	let result = run(terminal, &mut app);

	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
	loop {
		terminal.draw(|t| crate::ui::render(t, &app))?;

		match event::read()? {
			Event::Key(key) if key.kind == event::KeyEventKind::Press => {
				if app.handle_key(key.code) {
					break;
				}
			}
			Event::FocusLost => {
				app.pause_all_toggles();
			}
			Event::FocusGained => {
				app.resume_all_toggles();
			}
			_ => {}
		}
	}

	Ok(())
}
