use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal,
	crossterm::event::{self, Event},
};

mod app;
mod ui;

use app::AppState;

use crate::app::{Click, Toggle};

fn main() -> Result<()> {
	let mut app = AppState::default();
	app.list_state.borrow_mut().select_first();
	// NOTE: Testing, remove the next line and function
	prep(&mut app);
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

fn prep(app: &mut AppState) {
	app.toggles.push(Toggle {
		keys: String::from("Test1"),
		active: false,
	});
	app.toggles.push(Toggle {
		keys: String::from("Test2"),
		active: true,
	});
	app.toggles.push(Toggle {
		keys: String::from("Test3"),
		active: false,
	});

	app.clicks.push(Click {
		keys: String::from("Test3"),
		interval: 11,
		active: true,
	});
	app.clicks.push(Click {
		keys: String::from("Test3"),
		interval: 20,
		active: false,
	});
	app.clicks.push(Click {
		keys: String::from("Test3"),
		interval: 100,
		active: true,
	});
}
