use ratatui::crossterm::event::KeyCode;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Pane {
	#[default]
	Toggles,
	Clicks,
}

#[derive(Debug, Default)]
pub struct AppState {
	pub focus_pane: Pane,
	pub show_add_form: bool,
	pub toggles: Vec<Toggle>,
	pub clicks: Vec<Click>,
	//pub list_state: ListState,
}

#[derive(Debug)]
pub struct Toggle {
	pub key: String,
	pub active: bool,
}

#[derive(Debug)]
pub struct Click {
	pub key: String,
	pub interval: u32,
	pub active: bool,
}

impl AppState {
	pub fn handle_key(&mut self, code: KeyCode) -> bool {
		match code {
			KeyCode::Esc => true,
			KeyCode::Char('Q') => true,
			KeyCode::Char('t') | KeyCode::Char('h') => {
				self.focus_pane = Pane::Toggles;
				false
			}
			KeyCode::Char('c') | KeyCode::Char('l') => {
				self.focus_pane = Pane::Clicks;
				false
			}
			KeyCode::Char('n') => {
				self.show_add_form = true;
				false
			}
			KeyCode::Char('x') => {
				self.show_add_form = false;
				false
			}
			_ => false,
		}
	}
}
