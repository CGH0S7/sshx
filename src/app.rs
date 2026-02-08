use anyhow::Result;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::server::Server;

#[derive(Serialize, Deserialize, Default)]
struct AppState {
    #[serde(default)]
    last_connected: Option<String>,
}

pub enum InputMode {
    Normal,
    Adding(AddingState),
    Editing(EditingState),
    ConfirmDelete(usize), // 存储要删除的服务器索引
    ShowMessage(String), // 显示提示信息
}

pub struct AddingState {
    pub field_idx: usize,
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: String,
    pub jump_host: String,
}

impl AddingState {
    pub fn new() -> Self {
        Self {
            field_idx: 0,
            name: String::new(),
            user: String::new(),
            host: String::new(),
            port: "22".to_string(),
            jump_host: String::new(),
        }
    }
}

pub struct EditingState {
    pub server_index: usize,
    pub field_idx: usize,
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: String,
    pub jump_host: String,
}

impl EditingState {
    pub fn new(server: &Server, index: usize) -> Self {
        Self {
            server_index: index,
            field_idx: 0,
            name: server.name.clone(),
            user: server.user.clone(),
            host: server.host.clone(),
            port: server.port.clone(),
            jump_host: server.jump_host.clone(),
        }
    }
}

pub struct App {
    pub servers: Vec<Server>,
    pub state: ListState,
    pub input_mode: InputMode,
    pub pending_g: bool,
    config_path: PathBuf,
    state_path: PathBuf,
    last_connected: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_config_dir = config_dir.join("sshx");
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir)?;
        }
        let config_path = app_config_dir.join("servers.json");
        let state_path = app_config_dir.join("state.json");

        let mut servers: Vec<Server> = if config_path.exists() {
            let data = fs::read_to_string(&config_path)?;
            serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        let app_state = Self::load_state(&state_path);
        let last_connected = app_state.last_connected;

        // Reorder: move last-connected server to the top
        if let Some(ref key) = last_connected {
            if let Some(pos) = servers.iter().position(|s| Self::server_key(s) == *key) {
                if pos > 0 {
                    let server = servers.remove(pos);
                    servers.insert(0, server);
                }
            }
        }

        let mut state = ListState::default();
        if !servers.is_empty() {
            state.select(Some(0));
        }

        Ok(Self {
            servers,
            state,
            input_mode: InputMode::Normal,
            pending_g: false,
            config_path,
            state_path,
            last_connected,
        })
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.servers)?;
        fs::write(&self.config_path, data)?;
        Ok(())
    }

    pub fn next(&mut self) {
        if self.servers.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.servers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.servers.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.servers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_first(&mut self) {
        if !self.servers.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        if !self.servers.is_empty() {
            self.state.select(Some(self.servers.len() - 1));
        }
    }

    fn server_key(server: &Server) -> String {
        format!("{}@{}:{}", server.user, server.host, server.port)
    }

    fn load_state(state_path: &PathBuf) -> AppState {
        if state_path.exists() {
            fs::read_to_string(state_path)
                .ok()
                .and_then(|data| serde_json::from_str(&data).ok())
                .unwrap_or_default()
        } else {
            AppState::default()
        }
    }

    fn save_state(&self) -> Result<()> {
        let app_state = AppState {
            last_connected: self.last_connected.clone(),
        };
        let data = serde_json::to_string_pretty(&app_state)?;
        fs::write(&self.state_path, data)?;
        Ok(())
    }

    pub fn set_last_connected(&mut self, server: &Server) {
        let key = Self::server_key(server);
        self.last_connected = Some(key.clone());
        let _ = self.save_state();

        // Reorder: move the connected server to the top
        if let Some(pos) = self.servers.iter().position(|s| Self::server_key(s) == key) {
            if pos > 0 {
                let s = self.servers.remove(pos);
                self.servers.insert(0, s);
                let _ = self.save();
            }
        }
        self.state.select(Some(0));
    }
}