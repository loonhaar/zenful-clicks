use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use ratatui::{crossterm::event::KeyEvent, widgets::ListState};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum FormField {
	#[default]
	Keys,
	Interval,
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Pane {
	#[default]
	Toggles,
	Clicks,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Toggle {
	pub keys: String,
	pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Click {
	pub key: String,
	pub interval: u32,
	pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
	pub active_pane: Pane,
	pub toggles: Vec<Toggle>,
	pub clicks: Vec<Click>,
}

impl AppConfig {
	fn get_config_path() -> Option<PathBuf> {
		let mut path = dirs::config_dir()?;
		path.push("zenful-clicks");
		path.push("config.json");
		Some(path)
	}

	pub fn load() -> Option<Self> {
		let path = Self::get_config_path()?;

		if !path.exists() {
			return None;
		}

		let mut file = File::open(path).ok()?;
		let mut contents = String::new();
		file.read_to_string(&mut contents).ok()?;

		let mut config: AppConfig = serde_json::from_str(&contents).ok()?;

		for toggle in &mut config.toggles {
			toggle.active = false;
		}

		for click in &mut config.clicks {
			click.active = false;
		}

		Some(config)
	}

	pub fn save(
		active_pane: Pane,
		toggles: &[Toggle],
		clicks: &[Click],
	) -> Result<(), Box<dyn std::error::Error>> {
		let path = Self::get_config_path().ok_or_else(|| {
			std::io::Error::new(ErrorKind::NotFound, "Could not find config directory")
		})?;

		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent)?;
		}

		let config = AppConfig {
			active_pane,
			toggles: toggles.to_vec(),
			clicks: clicks.to_vec(),
		};

		let serialized = serde_json::to_string_pretty(&config)?;
		let mut file = File::create(path)?;
		file.write_all(serialized.as_bytes())?;
		Ok(())
	}
}

#[derive(Debug)]
pub struct AppState {
	pub active_pane: Pane,
	pub focus_signal: Arc<(Mutex<bool>, Condvar)>,
	pub focus_regained_time: Instant,
	pub show_add_form: bool,
	pub show_delete_confirm: bool,
	pub show_help: bool,
	pub help_scroll: u16,
	pub form_field: FormField,
	pub form_keys: String,
	pub form_interval: String,
	pub toggles: Vec<Toggle>,
	pub clicks: Vec<Click>,
	pub list_state: RefCell<ListState>,
	pub toggle_controller: crate::os_input::OsInputManager,
	pub click_controller: crate::os_input::OsInputManager,
}

impl Default for AppState {
	fn default() -> Self {
		let saved_config = AppConfig::load();

		let (active_pane, toggles, clicks, show_help) = match saved_config {
			Some(config) => (config.active_pane, config.toggles, config.clicks, false),
			None => (Pane::Toggles, Vec::new(), Vec::new(), true),
		};

		Self {
			active_pane,
			focus_signal: Arc::new((Mutex::new(true), Condvar::new())),
			focus_regained_time: Instant::now(),
			show_add_form: false,
			show_delete_confirm: false,
			show_help,
			help_scroll: 0,
			form_field: FormField::Keys,
			form_keys: String::new(),
			form_interval: String::new(),
			toggles,
			clicks,
			list_state: RefCell::new(ListState::default()),
			toggle_controller: crate::os_input::OsInputManager::new(),
			click_controller: crate::os_input::OsInputManager::new(),
		}
	}
}

impl AppState {
	pub fn handle_key(&mut self, event: KeyEvent) -> bool {
		crate::handlers::handle_key(self, event)
	}

	pub fn save_config(&self) {
		if let Err(e) = AppConfig::save(self.active_pane, &self.toggles, &self.clicks) {
			eprintln!("Failsed to save config: {}", e)
		}
	}
}

impl Drop for AppState {
	fn drop(&mut self) {
		self.save_config();
		self.toggle_controller.shutdown();
		self.click_controller.shutdown();
		thread::sleep(Duration::from_millis(100));
	}
}
