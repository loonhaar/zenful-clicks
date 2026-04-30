use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal,
	Frame,
	crossterm::event::{self, Event}, widgets::{Paragraph, Widget}
};

fn main() -> Result<()> {
	color_eyre::install()?;

	let terminal = ratatui::init();
	let result = run(terminal);

	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
	loop {
		// Rendering
		terminal.draw(render)?;

		// Input handling
		if let Event::Key(key) = event::read()? {
			match key.code {
				event::KeyCode::Char('q') => break,
				event::KeyCode::Esc 	  => break,
				_ => {}
			}
		}
	}

	Ok(())
}

fn render(frame: &mut Frame) {
	Paragraph::new("Hi! :3").render(frame.area(), frame.buffer_mut());
}
