# sshx

`sshx` is a simple, terminal-based SSH connection manager written in Rust. It provides a TUI (Text User Interface) to manage your SSH connections, allowing you to add, edit, delete, and connect to servers quickly.

## Features

- **TUI Interface**: built with `ratatui` for a smooth terminal experience.
- **Manage Servers**: Add, edit, and delete server configurations easily.
- **One-Key Connection**: Connect to your saved servers via standard `ssh` with a single keypress.
- **Mosh Support**: Launch `mosh` sessions directly from the interface.
- **Key Management**: Quickly copy your public key to a server using `ssh-copy-id`.
- **Persistent Storage**: Server configurations are saved in JSON format in your user configuration directory.

## Installation

### Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install) (cargo)
- `ssh` client
- `mosh` (optional, for mosh support)
- `ssh-copy-id` (optional, for key copying)

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/CGH0S7/sshx.git
   cd sshx
   ```

2. Build:
   ```bash
   cargo build --release
   ```

## Usage

Start the application by running `sshx` (if installed) or `cargo run`.

### Key Bindings

**Navigation & General:**
- `j` / `Down` / `Tab`: Select next server
- `k` / `Up` / `Shift+Tab`: Select previous server
- `q` / `Esc` / `Ctrl+c` / `Ctrl + d`: Quit application

**Actions:**
- `Enter`: Connect to selected server via `ssh`
- `m`: Connect to selected server via `mosh`
- `n`: Add a new server
- `N` (Shift+n): Run `ssh-copy-id` for the selected server
- `i`: Edit the selected server
- `d`: Delete the selected server

**Input Mode (Adding/Editing):**
- `Tab` / `Down`: Move to next field
- `Shift+Tab` / `Up`: Move to previous field
- `Enter`: Save and close
- `Esc`: Cancel and return to list

## Configuration

Server configurations are stored in `~/.config/sshx/servers.json` (on Linux) or the equivalent configuration directory for your OS.

## License

This project is licensed under the terms defined in the `Cargo.toml` file.
