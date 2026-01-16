use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::Backend, Terminal};
use std::{
    io::{self, Write},
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
                wait_for_enter()?;
            }
        }
        Err(e) => {
            println!("Failed to execute {}: {}", program, e);
            println!("Press Enter to continue...");
            wait_for_enter()?;
        }
    }

    // Restore TUI state
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture)?;
    terminal.clear()?;
    Ok(())
}

/// Wait for Enter key press in a cross-platform way
fn wait_for_enter() -> io::Result<()> {
    io::stdout().flush()?;

    loop {
        if let Event::Key(key) = crossterm::event::read()? {
            if key.code == KeyCode::Enter {
                break;
            }
        }
    }
    Ok(())
}

/// Check if a command is available in the system
pub fn is_command_available(command: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        Command::new("where")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// Windows-specific ssh-copy-id implementation
#[cfg(target_os = "windows")]
fn run_ssh_copy_id_windows(args: &[String]) -> io::Result<std::process::ExitStatus> {
    use std::env;
    use std::path::PathBuf;

    // Get the public key path
    let userprofile = env::var("USERPROFILE")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "USERPROFILE not set"))?;
    let public_key = PathBuf::from(userprofile).join(".ssh").join("id_rsa.pub");

    if !public_key.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("ERROR: failed to open ID file '{}': No such file", public_key.display())
        ));
    }

    // Verify the public key exists and is readable
    let _key_content = std::fs::read_to_string(&public_key)?;

    // Build the SSH command
    let mut ssh_args = Vec::new();
    let mut target = String::new();

    // Parse args to extract target and options
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-p" => {
                if i + 1 < args.len() {
                    ssh_args.push("-p".to_string());
                    ssh_args.push(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-o" => {
                if i + 1 < args.len() {
                    ssh_args.push("-o".to_string());
                    ssh_args.push(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            arg => {
                target = arg.to_string();
                i += 1;
            }
        }
    }

    ssh_args.push(target);
    ssh_args.push("umask 077; test -d .ssh || mkdir .ssh ; cat >> .ssh/authorized_keys || exit 1".to_string());

    // Execute: type public_key | ssh args
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &format!("type \"{}\" | ssh {}", public_key.display(), ssh_args.join(" "))])
            .status()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("cat '{}' | ssh {}", public_key.display(), ssh_args.join(" ")))
            .status()
    };

    output
}

/// Run ssh-copy-id with platform-specific implementation
pub fn run_ssh_copy_id<B: Backend + std::io::Write>(
    terminal: &mut Terminal<B>,
    args: &[String],
) -> io::Result<()> {
    // Drop TUI state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    #[cfg(target_os = "windows")]
    {
        println!("Running ssh-copy-id (Windows implementation)...");
        let status = run_ssh_copy_id_windows(args);

        match status {
            Ok(s) => {
                if !s.success() {
                    println!("Command exited with status: {}", s);
                    println!("Press Enter to continue...");
                    wait_for_enter()?;
                }
            }
            Err(e) => {
                println!("Failed to execute ssh-copy-id: {}", e);
                println!("Press Enter to continue...");
                wait_for_enter()?;
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("Running ssh-copy-id {}...", args.join(" "));
        let status = Command::new("ssh-copy-id")
            .args(args)
            .status();

        match status {
            Ok(s) => {
                if !s.success() {
                    println!("Command exited with status: {}", s);
                    println!("Press Enter to continue...");
                    wait_for_enter()?;
                }
            }
            Err(e) => {
                println!("Failed to execute ssh-copy-id: {}", e);
                println!("Press Enter to continue...");
                wait_for_enter()?;
            }
        }
    }

    // Restore TUI state
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture)?;
    terminal.clear()?;
    Ok(())
}