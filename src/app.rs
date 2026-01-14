use anyhow::Result;
use ratatui::widgets::ListState;
use std::{fs, path::PathBuf};

use crate::server::Server;

pub enum InputMode {
    Normal,
    Adding(AddingState),
    Editing(EditingState),
    ConfirmDelete(usize), // 存储要删除的服务器索引
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
    config_path: PathBuf,
}

impl App {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_config_dir = config_dir.join("sshx");
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir)?;
        }
        let config_path = app_config_dir.join("servers.json");

        let servers = if config_path.exists() {
            let data = fs::read_to_string(&config_path)?;
            serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        let mut state = ListState::default();
        if !servers.is_empty() {
            state.select(Some(0));
        }

        Ok(Self {
            servers,
            state,
            input_mode: InputMode::Normal,
            config_path,
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
}