mod app;
mod server;
mod ui;
mod command;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    error::Error,
    io,
};

use app::{App, InputMode, AddingState, EditingState};
use command::run_external_command;
use ui::ui;
use server::Server;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;

    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend + std::io::Write>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        // Hide cursor in normal mode, show in input/edit mode
        match app.input_mode {
            InputMode::Normal | InputMode::ConfirmDelete(_) => terminal.hide_cursor()?,
            InputMode::Adding(_) | InputMode::Editing(_) => terminal.show_cursor()?,
        }
        
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match &mut app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                    KeyCode::Down | KeyCode::Tab | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::BackTab | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('n') => {
                        app.input_mode = InputMode::Adding(AddingState::new());
                    }
                    KeyCode::Char('c') => {
                        // SSH Copy ID 
                        if let Some(idx) = app.state.selected() {
                            let server = &app.servers[idx];
                            let args = server.to_copy_id_args();
                            run_external_command(terminal, "ssh-copy-id", &args)?;
                        }
                    }
                    KeyCode::Char('d') => {
                        // 进入删除确认模式
                        if let Some(idx) = app.state.selected() {
                            app.input_mode = InputMode::ConfirmDelete(idx);
                        }
                    }
                    KeyCode::Char('i') => {
                        // Edit server
                        if let Some(idx) = app.state.selected() {
                            let server = &app.servers[idx].clone();
                            app.input_mode = InputMode::Editing(EditingState::new(server, idx));
                        }
                    }
                    KeyCode::Char('m') => {
                        // Mosh
                        if let Some(idx) = app.state.selected() {
                            let server = &app.servers[idx];
                            let args = server.to_mosh_args();
                            run_external_command(terminal, "mosh", &args)?;
                        }
                    }
                    KeyCode::Char('s') => {
                        // SFTP
                        if let Some(idx) = app.state.selected() {
                            let server = &app.servers[idx];
                            let args = server.to_sftp_args();
                            run_external_command(terminal, "sftp", &args)?;
                        }
                    }
                    KeyCode::Enter => {
                        // SSH
                        if let Some(idx) = app.state.selected() {
                            let server = &app.servers[idx];
                            let args = server.to_ssh_args();
                            run_external_command(terminal, "ssh", &args)?;
                        }
                    }
                    _ => {}
                },
                InputMode::Editing(state) => match key.code {
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Char(c) => {
                        match state.field_idx {
                            0 => state.name.push(c),
                            1 => state.user.push(c),
                            2 => state.host.push(c),
                            3 => state.port.push(c),
                            4 => state.jump_host.push(c),
                            _ => {}
                        }
                    }
                    KeyCode::Backspace => {
                        match state.field_idx {
                            0 => { state.name.pop(); },
                            1 => { state.user.pop(); },
                            2 => { state.host.pop(); },
                            3 => { state.port.pop(); },
                            4 => { state.jump_host.pop(); },
                            _ => {}
                        }
                    }
                    KeyCode::Tab | KeyCode::Down => {
                        if state.field_idx < 4 {
                            state.field_idx += 1;
                        } else {
                            state.field_idx = 0;
                        }
                    }
                    KeyCode::BackTab | KeyCode::Up => {
                        if state.field_idx > 0 {
                            state.field_idx -= 1;
                        } else {
                            state.field_idx = 4;
                        }
                    }
                    KeyCode::Enter => {
                        // Validate and save
                        if !state.name.is_empty() && !state.host.is_empty() {
                            let updated_server = Server {
                                name: state.name.clone(),
                                user: if state.user.is_empty() { "root".to_string() } else { state.user.clone() },
                                host: state.host.clone(),
                                port: if state.port.is_empty() { "22".to_string() } else { state.port.clone() },
                                jump_host: state.jump_host.clone(),
                            };
                            app.servers[state.server_index] = updated_server;
                            let _ = app.save();
                            app.input_mode = InputMode::Normal;
                        }
                    }
                    _ => {}
                },
                InputMode::Adding(state) => match key.code {
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Char(c) => {
                        match state.field_idx {
                            0 => state.name.push(c),
                            1 => state.user.push(c),
                            2 => state.host.push(c),
                            3 => state.port.push(c),
                            4 => state.jump_host.push(c),
                            _ => {}
                        }
                    }
                    KeyCode::Backspace => {
                        match state.field_idx {
                            0 => { state.name.pop(); },
                            1 => { state.user.pop(); },
                            2 => { state.host.pop(); },
                            3 => { state.port.pop(); },
                            4 => { state.jump_host.pop(); },
                            _ => {}
                        }
                    }
                    KeyCode::Tab | KeyCode::Down => {
                        if state.field_idx < 4 {
                            state.field_idx += 1;
                        } else {
                            state.field_idx = 0;
                        }
                    }
                    KeyCode::BackTab | KeyCode::Up => {
                        if state.field_idx > 0 {
                            state.field_idx -= 1;
                        } else {
                            state.field_idx = 4;
                        }
                    }
                    KeyCode::Enter => {
                        // Validate and save
                        if !state.name.is_empty() && !state.host.is_empty() {
                            let new_server = Server {
                                name: state.name.clone(),
                                user: if state.user.is_empty() { "root".to_string() } else { state.user.clone() },
                                host: state.host.clone(),
                                port: if state.port.is_empty() { "22".to_string() } else { state.port.clone() },
                                jump_host: state.jump_host.clone(),
                            };
                            app.servers.push(new_server);
                            let _ = app.save();
                            app.state.select(Some(app.servers.len() - 1));
                            app.input_mode = InputMode::Normal;
                        }
                    }
                    _ => {}
                },
                InputMode::ConfirmDelete(idx) => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        let idx = *idx;
                        app.input_mode = InputMode::Normal;
                        if idx < app.servers.len() {
                            app.servers.remove(idx);
                            if app.servers.is_empty() {
                                app.state.select(None);
                            } else if idx >= app.servers.len() {
                                app.state.select(Some(app.servers.len() - 1));
                            }
                            let _ = app.save();
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}
