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
	// toggles: Vec<Toggle>,
	// clicks: Vec<Click>,
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

impl AppState {
	pub fn handle_key(&mut self, code: KeyCode) -> bool {
		match code {
			KeyCode::Esc => true,
			KeyCode::Char('Q') => true,
			KeyCode::Char('T') => {
				self.focus_pane = Pane::Toggles;
				false
			}
			KeyCode::Char('C') => {
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
