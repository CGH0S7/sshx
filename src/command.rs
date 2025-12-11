use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::Backend, Terminal};
use std::{
    io,
    process::Command,
};

pub fn run_external_command<B: Backend + std::io::Write>(
    terminal: &mut Terminal<B>,
    program: &str,
    args: &[String],
) -> io::Result<()> {
    // Drop TUI state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    // Run command
    println!("Running {} {}...", program, args.join(" "));
    let status = Command::new(program)
        .args(args)
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                println!("Command exited with status: {}", s);
                println!("Press Enter to continue...");
                let mut line = String::new();
                std::io::stdin().read_line(&mut line)?;
            }
        }
        Err(e) => {
            println!("Failed to execute {}: {}", program, e);
            println!("Press Enter to continue...");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
        }
    }

    // Restore TUI state
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture)?;
    terminal.clear()?;
    Ok(())
}