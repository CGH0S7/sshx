# sshx

`sshx` is a simple, terminal-based SSH connection manager written in Rust. It provides a TUI (Text User Interface) to manage your SSH connections, allowing you to add, edit, delete, and connect to servers quickly.

## Features

- **TUI Interface**: built with `ratatui` for a smooth terminal experience.
- **Manage Servers**: Add, edit, and delete server configurations easily.
- **One-Key Connection**: Connect to your saved servers via standard `ssh` with a single keypress.
- **Jump Host Support**: Connect through a bastion/proxy server using SSH `-J` option.
- **Mosh Support**: Launch `mosh` sessions directly from the interface.
- **SFTP Support**: Open SFTP sessions for file transfers.
- **Key Management**: Quickly copy your public key to a server using `ssh-copy-id`.
- **Delete Confirmation**: Prevent accidental deletion with a confirmation dialog.
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
- `s`: Connect to selected server via `sftp`
- `m`: Connect to selected server via `mosh`
- `n`: Add a new server
- `c`: Run `ssh-copy-id` for the selected server
- `i`: Edit the selected server
- `d`: Delete the selected server (with confirmation)

**Input Mode (Adding/Editing):**
- `Tab` / `Down`: Move to next field
- `Shift+Tab` / `Up`: Move to previous field
- `Enter`: Save and close
- `Esc`: Cancel and return to list

**Delete Confirmation:**
- `y`: Confirm deletion
- `n` / `Esc`: Cancel deletion

## Configuration

Server configurations are stored in `~/.config/sshx/servers.json` (on Linux) or the equivalent configuration directory for your OS.

### Jump Host (Bastion Server)

When adding or editing a server, you can specify a jump host to connect through a bastion server. The format is:

- `user@host` - Jump host with default port (22)
- `user@host:port` - Jump host with custom port

Leave the jump host field empty for direct connections (default).

## License

This project is licensed under the terms defined in the `Cargo.toml` file.
