# sshx

`sshx` is a simple, terminal-based SSH connection manager written in Rust. It provides a TUI (Text User Interface) to manage your SSH connections, allowing you to add, edit, delete, and connect to servers quickly.

## Features

- **Multiple Profiles**: Organize servers into different scenarios (e.g., Lab, Home, Cloud). Switch between them easily or create new ones.
- **TUI Interface**: Built with `ratatui` for a smooth terminal experience.
- **Manage Servers**: Add, edit, and delete server configurations easily.
- **One-Key Connection**: Connect to your saved servers via `ssh`, `sftp`, or `mosh` with a single keypress.
- **Broadcast Command**: Send the same command to multiple servers at once — pick your targets interactively, then execute.
- **Jump Host Support**: Connect through a bastion/proxy server using SSH `-J` option.
- **Mosh Support**: Launch `mosh` sessions directly from the interface (with availability check).
- **SFTP Support**: Open SFTP sessions for file transfers.
- **Key Management**: Quickly copy your public key to a server using `ssh-copy-id`.
- **Vim-style Navigation**: Use `j`/`k` to move, `gg` to jump to the top, `G` to jump to the bottom.
- **Last-Connected Sorting**: The most recently connected server is automatically moved to the top of the list.
- **Delete Confirmation**: Prevent accidental deletion with a confirmation dialog.
- **Persistent Storage**: Server configurations are saved in JSON format in your user configuration directory.
- **Windows Support**: Works on Windows with a built-in `ssh-copy-id` fallback.

## Installation

### Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install) (cargo)
- `ssh` client
- `mosh` (optional, for mosh support)
- `ssh-copy-id` (optional, for key copying; built-in fallback on Windows)

### Building from Source

```bash
git clone https://github.com/CGH0S7/sshx.git
cd sshx
cargo build --release
```

The binary will be at `target/release/sshx`.

## Usage

Run `sshx` (if installed to PATH) or `cargo run`.

### Key Bindings

**Navigation:**

| Key | Action |
|-----|--------|
| `j` / `Down` / `Tab` | Select next server |
| `k` / `Up` / `Shift+Tab` | Select previous server |
| `gg` | Jump to first server |
| `G` | Jump to last server |
| `q` / `Esc` / `Ctrl+C` / `Ctrl+D` | Quit |

**Actions:**

| Key | Action |
|-----|--------|
| `Enter` | Connect via `ssh` |
| `s` | Connect via `sftp` |
| `m` | Connect via `mosh` |
| `p` | Broadcast command to multiple servers |
| `e` | Open profile selection menu |
| `n` | Add a new server |
| `i` | Edit the selected server |
| `c` | Copy SSH public key (`ssh-copy-id`) |
| `d` | Delete the selected server (with confirmation) |

**Profiles (`e`):**

1. Press `e` to open the profile selection popup.
2. Use `j`/`k` to select an existing profile (indicated by `*` if it is currently active).
3. Press `Enter` to load the selected profile and switch your server list.
4. Press `n` to create a new profile.
5. Press `Esc` to cancel and return to the main server list.

**Adding / Editing a Server:**

| Key | Action |
|-----|--------|
| `Tab` / `Down` | Move to next field |
| `Shift+Tab` / `Up` | Move to previous field |
| `Enter` | Save and close |
| `Esc` | Cancel |

**Broadcast Command (`p`):**

1. Type the command to run, then press `Enter`.
2. Use `j`/`k` to move, `Space` to toggle server selection (highlighted in green when selected).
3. Press `Enter` to execute the command on all selected servers sequentially.
4. Press `Esc` at any step to cancel.

**Delete Confirmation:**

| Key | Action |
|-----|--------|
| `y` | Confirm deletion |
| `n` / `Esc` | Cancel |

## Configuration

Server configurations are stored in your user configuration directory. Each profile is a `.json` file:

- **Linux/macOS**: `~/.config/sshx/*.json`
- **Windows**: `%APPDATA%\sshx\*.json`

By default, servers are stored in `servers.json`. The application state (last connected server, last profile used) is stored in `state.json`.

### Jump Host (Bastion Server)

When adding or editing a server, you can specify a jump host to connect through a bastion server:

- `user@host` — jump host with default port (22)
- `user@host:port` — jump host with custom port

Leave the field empty for direct connections.

> Note: Mosh does not support jump host connections. Use SSH (`Enter`) instead.

## License

This project is licensed under the terms defined in the `Cargo.toml` file.
