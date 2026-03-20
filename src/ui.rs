use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear, ListState, Wrap},
    Frame,
};

use crate::app::{App, InputMode, BroadcastPhase};
use crate::server::Server;

pub fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Determine help text based on current mode
    let help_text = match &app.input_mode {
        InputMode::Normal => "Enter: SSH | s: SFTP | m: Mosh | p: Broadcast | n: New | e: Profile | c: Copy ID | i: Edit | d: Delete | q: Quit",
        InputMode::Adding(_) => "Enter: Save | Esc: Cancel | Tab: Next Field",
        InputMode::Editing(_) => "Enter: Save | Esc: Cancel | Tab: Next Field",
        InputMode::ConfirmDelete(_) => "y: Confirm Delete | n/Esc: Cancel",
        InputMode::ShowMessage(_) => "Press Enter, Esc or Space to close",
        InputMode::BroadcastCommand(s) => match s.phase {
            BroadcastPhase::EnterCommand => "Enter: Next | Esc: Cancel",
            BroadcastPhase::SelectServers => "Space: Toggle | j/k: Move | Enter: Execute | Esc: Cancel",
        },
        InputMode::SelectingProfile => "Enter: Load | n: New Profile | Esc: Cancel",
        InputMode::CreatingProfile(_) => "Enter: Create | Esc: Cancel",
    };

    // Calculate needed height for help text based on width
    // Inner width is size.width - 2 (borders)
    let inner_width = size.width.saturating_sub(2);
    let help_lines = if inner_width > 0 {
        // Rough estimate of lines needed: total length / inner width + 1
        (help_text.len() as u16 + inner_width - 1) / inner_width
    } else {
        1
    };
    let help_height = help_lines + 2; // +2 for borders

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(help_height)].as_ref())
        .split(size);

    let items: Vec<ListItem> = app
        .servers
        .iter()
        .map(|s| {
            let content = format!("{} ({}) - {}:{}", s.name, s.user, s.host, s.port);
            ListItem::new(content).style(Style::default())
        })
        .collect();

    let title = format!(" SSHX - Servers [{}] ", app.current_profile);
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    // Help text with wrapping
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title(" Help "))
        .wrap(Wrap { trim: true });
    f.render_widget(help, chunks[1]);

    // Popup for Adding Server
    if let InputMode::Adding(state) = &app.input_mode {
        render_connection_form(
            f,
            " Add New Connection ",
            state.field_idx,
            &[&state.name, &state.user, &state.host, &state.port, &state.jump_host],
        );
    }

    // Popup for Editing Server
    if let InputMode::Editing(state) = &app.input_mode {
        render_connection_form(
            f,
            " Edit Connection ",
            state.field_idx,
            &[&state.name, &state.user, &state.host, &state.port, &state.jump_host],
        );
    }

    // Popup for Delete Confirmation
    if let InputMode::ConfirmDelete(idx) = &app.input_mode {
        if *idx < app.servers.len() {
            let server_name = &app.servers[*idx].name;
            render_confirm_dialog(f, server_name);
        }
    }

    // Popup for Message
    if let InputMode::ShowMessage(msg) = &app.input_mode {
        render_message_dialog(f, msg);
    }

    // Popup for Broadcast Command
    if let InputMode::BroadcastCommand(state) = &app.input_mode {
        match state.phase {
            BroadcastPhase::EnterCommand => {
                render_broadcast_command_input(f, &state.command);
            }
            BroadcastPhase::SelectServers => {
                render_broadcast_server_select(f, &app.servers, &state.selected, state.cursor);
            }
        }
    }

    // Popup for Profile Selection
    if let InputMode::SelectingProfile = &app.input_mode {
        render_profile_selection(f, &app.profiles, &mut app.profile_state, &app.current_profile);
    }

    // Popup for Profile Creation
    if let InputMode::CreatingProfile(name) = &app.input_mode {
        render_profile_creation(f, name);
    }
}

fn render_connection_form(f: &mut Frame, title: &str, field_idx: usize, values: &[&String]) {
    let size = f.size();
    // Use fixed height of 22 lines (enough for 5 fields * 3 lines + margins/borders)
    let area = centered_fixed_rect(60, 22, size);

    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let input_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // Name
                Constraint::Length(3), // User
                Constraint::Length(3), // Host
                Constraint::Length(3), // Port
                Constraint::Length(3), // Jump Host
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(area);

    let fields_labels = [
        "Name",
        "User (default: root)",
        "Host/IP",
        "Port (default: 22)",
        "Jump Host (optional, e.g. user@host:port)",
    ];

    for (i, label) in fields_labels.iter().enumerate() {
        let value = values[i];
        let mut style = Style::default();
        if field_idx == i {
            style = style.fg(Color::Yellow);
        }
        let input = Paragraph::new(value.as_str())
            .style(style)
            .block(Block::default().borders(Borders::ALL).title(*label));
        f.render_widget(input, input_layout[i]);

        // Show cursor in the active input field
        if field_idx == i {
            f.set_cursor(
                input_layout[i].x + value.len() as u16 + 1,
                input_layout[i].y + 1,
            );
        }
    }
}

fn centered_fixed_rect(width_percent: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Fill(1),
                Constraint::Length(height),
                Constraint::Fill(1),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Fill(1),
                Constraint::Percentage(width_percent),
                Constraint::Fill(1),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn render_confirm_dialog(f: &mut Frame, server_name: &str) {
    let size = f.size();
    let area = centered_fixed_rect(50, 7, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Confirm Delete ")
        .style(Style::default().fg(Color::Red));
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Length(2)].as_ref())
        .split(area);

    let msg = format!("Delete \"{}\"?", server_name);
    let text = Paragraph::new(msg).style(Style::default().fg(Color::White));
    f.render_widget(text, inner[0]);

    let hint = Paragraph::new("Press 'y' to confirm, 'n' or Esc to cancel")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, inner[1]);
}

fn render_message_dialog(f: &mut Frame, message: &str) {
    let size = f.size();
    let lines: Vec<&str> = message.lines().collect();
    let height = (lines.len() as u16 + 4).min(20);
    let area = centered_fixed_rect(60, height, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Notice ")
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(area);

    let text = Paragraph::new(message).style(Style::default().fg(Color::White));
    f.render_widget(text, inner[0]);

    let hint = Paragraph::new("Press Enter, Esc or Space to close")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, inner[1]);
}

fn render_broadcast_command_input(f: &mut Frame, command: &str) {
    let size = f.size();
    let area = centered_fixed_rect(60, 7, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Broadcast Command - Enter Command ")
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Length(1)].as_ref())
        .split(area);

    let input = Paragraph::new(command)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Command"));
    f.render_widget(input, inner[0]);

    // Show cursor at end of input
    f.set_cursor(inner[0].x + command.len() as u16 + 1, inner[0].y + 1);

    let hint = Paragraph::new("Enter: confirm | Esc: cancel")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, inner[1]);
}

fn render_broadcast_server_select(f: &mut Frame, servers: &[Server], selected: &[bool], cursor: usize) {
    let size = f.size();
    let height = (servers.len() as u16 + 5).min(size.height.saturating_sub(4));
    let area = centered_fixed_rect(60, height, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Broadcast Command - Select Servers ")
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(area);

    let items: Vec<ListItem> = servers
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let check = if selected[i] { "[x]" } else { "[ ]" };
            let content = format!("{} {} ({}) - {}:{}", check, s.name, s.user, s.host, s.port);
            let style = if i == cursor {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if selected[i] {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default());
    f.render_widget(list, inner[0]);

    let hint = Paragraph::new("Space: toggle | j/k: move | Enter: execute | Esc: cancel")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, inner[1]);
}

fn render_profile_selection(f: &mut Frame, profiles: &[String], state: &mut ListState, current: &str) {
    let size = f.size();
    let height = (profiles.len() as u16 + 4).min(size.height.saturating_sub(4));
    let area = centered_fixed_rect(40, height, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select Profile ")
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(area);

    let items: Vec<ListItem> = profiles
        .iter()
        .map(|p| {
            let mut style = Style::default();
            let content = if p == current {
                style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                format!("* {}", p)
            } else {
                p.clone()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, inner[0], state);

    let hint = Paragraph::new("Enter: Load | n: New | Esc: Cancel")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, inner[1]);
}

fn render_profile_creation(f: &mut Frame, name: &str) {
    let size = f.size();
    let area = centered_fixed_rect(40, 7, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" New Profile ")
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Length(1)].as_ref())
        .split(area);

    let input = Paragraph::new(name)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Profile Name"));
    f.render_widget(input, inner[0]);

    // Show cursor
    f.set_cursor(inner[0].x + name.len() as u16 + 1, inner[0].y + 1);

    let hint = Paragraph::new("Enter: Create | Esc: Cancel")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, inner[1]);
}
