use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal,
	crossterm::{
		ExecutableCommand,
		event::{
			self, EnableFocusChange, Event, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
			PushKeyboardEnhancementFlags,
		},
	},
};
use std::{
	io::stdout,
	thread::sleep,
	time::{Duration, Instant},
};

mod actions;
mod appstate;
mod handlers;
mod key_parser;
mod os_input;
mod ui;

use appstate::AppState;

fn main() -> Result<()> {
	let mut app = AppState::default();
	app.list_state.borrow_mut().select_first();
	color_eyre::install()?;

	let terminal = ratatui::init();

	let _ = stdout().execute(EnableFocusChange);

	let flags = KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
		| KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES;
	let _ = stdout().execute(PushKeyboardEnhancementFlags(flags));

	let result = run(terminal, &mut app);

	let _ = stdout().execute(PopKeyboardEnhancementFlags);
	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal, app: &mut AppState) -> Result<()> {
	loop {
		terminal.draw(|t| crate::ui::render(t, &app))?;

		match event::read()? {
			Event::Key(key) if key.kind == event::KeyEventKind::Press => {
				let (lock, _cvar) = &*app.focus_signal;
				let is_focused = *lock.lock().unwrap();
				let debounce = Duration::from_millis(200);

				if !is_focused || app.focus_regained_time.elapsed() < debounce {
					sleep(debounce);
					continue;
				}

				if app.handle_key(key) {
					break;
				}
			}
			Event::FocusLost => {
				let (lock, cvar) = &*app.focus_signal;
				let mut focused = lock.lock().unwrap();
				*focused = false;
				cvar.notify_all();
			}
			Event::FocusGained => {
				let (lock, cvar) = &*app.focus_signal;
				let mut focused = lock.lock().unwrap();
				*focused = true;
				app.focus_regained_time = Instant::now();
				cvar.notify_all();
			}
			_ => {}
		}
	}

	Ok(())
}
