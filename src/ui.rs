use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};

use crate::app::{App, InputMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    let items: Vec<ListItem> = app
        .servers
        .iter()
        .map(|s| {
            let content = format!("{} ({}) - {}:{}", s.name, s.user, s.host, s.port);
            ListItem::new(content).style(Style::default())
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" SSHX - Servers "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    // Help text
    let help_text = match app.input_mode {
        InputMode::Normal => "Enter: SSH | s: SFTP | m: Mosh | n: New | c: Copy ID | i: Edit | d: Delete | q: Quit",
        InputMode::Adding(_) => "Enter: Save | Esc: Cancel | Tab: Next Field",
        InputMode::Editing(_) => "Enter: Save | Esc: Cancel | Tab: Next Field",
        InputMode::ConfirmDelete(_) => "y: Confirm Delete | n/Esc: Cancel",
        InputMode::ShowMessage(_) => "Press Enter, Esc or Space to close",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title(" Help "));
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
