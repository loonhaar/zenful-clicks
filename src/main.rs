use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal,
	crossterm::{
		ExecutableCommand,
		event::{self, EnableFocusChange, Event},
	},
};
use std::{io::stdout, sync::atomic::Ordering};

mod app;
mod os_input;
mod ui;

use app::AppState;

fn main() -> Result<()> {
	let mut app = AppState::default();
	app.list_state.borrow_mut().select_first();
	color_eyre::install()?;

	let terminal = ratatui::init();

	let _ = stdout().execute(EnableFocusChange);

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
				{
					let (lock, _) = &*app.focus_signal;
					let mut focused = lock.lock().unwrap();
					*focused = false;
				}

				app.pause_all_toggles();
			}
			Event::FocusGained => {
				{
					let (lock, cvar) = &*app.focus_signal;
					let mut focused = lock.lock().unwrap();
					*focused = true;
					cvar.notify_all();
				}

				app.resume_all_toggles();
			}
			_ => {}
		}
	}

	Ok(())
}
