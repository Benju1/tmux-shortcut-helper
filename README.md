# tmux-shortcut-helper

A lightweight tmux shortcut helper that brings zellij-style modal keybindings to tmux.

No more memorizing complex `Ctrl+b` combinations - just press `Ctrl+p` for pane operations, `Ctrl+t` for tabs, etc.

## Features

- **Zellij-style modal operation**: `Ctrl+key` to enter mode, `Esc` to exit
- **Dynamic status bar**: Shows available shortcuts for current mode
- **No dependencies**: Pure Rust, single binary
- **Fast**: Instant response, minimal resource usage

## Demo

```
# Default status bar
Ctrl +  <p> PANE  <t> TAB  <n> RESIZE  <h> MOVE  <s> SEARCH  <o> SESSION  <q> QUIT

# After pressing Ctrl+p (PANE mode)
[PANE]  <n> Split→  <d> Split↓  <x> Close  <f> Full  <hjkl> Move  <r> Resize  <Esc> Back
```

## Installation

### From crates.io

```bash
cargo install tmux-shortcut-helper
```

### From source

```bash
git clone https://github.com/nyanko3141592/tmux-shortcut-helper
cd tmux-shortcut-helper
cargo install --path .
```

## Setup

Add to your `~/.tmux.conf`:

```bash
# ===========================================
# tmux-shortcut-helper: zellij-style config
# ===========================================

# Status bar
set -g status-right "#(tmux-shortcut-helper --zellij)"
set -g status-right-length 120

# --- PANE mode (Ctrl+p) ---
bind -n C-p set status-right '#(tmux-shortcut-helper --mode-status pane)' \; switch-client -T pane-mode
bind -T pane-mode h select-pane -L \; switch-client -T pane-mode
bind -T pane-mode j select-pane -D \; switch-client -T pane-mode
bind -T pane-mode k select-pane -U \; switch-client -T pane-mode
bind -T pane-mode l select-pane -R \; switch-client -T pane-mode
bind -T pane-mode n split-window -h -c "#{pane_current_path}"
bind -T pane-mode d split-window -v -c "#{pane_current_path}"
bind -T pane-mode x confirm-before -p "Close pane? (y/n)" kill-pane
bind -T pane-mode f resize-pane -Z
bind -T pane-mode Escape set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T pane-mode Enter set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T pane-mode q set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root

# --- TAB mode (Ctrl+t) ---
bind -n C-t set status-right '#(tmux-shortcut-helper --mode-status tab)' \; switch-client -T tab-mode
bind -T tab-mode n new-window -c "#{pane_current_path}"
bind -T tab-mode x confirm-before -p "Close tab? (y/n)" kill-window
bind -T tab-mode r command-prompt -I "#W" "rename-window '%%'"
bind -T tab-mode h previous-window \; switch-client -T tab-mode
bind -T tab-mode l next-window \; switch-client -T tab-mode
bind -T tab-mode 1 select-window -t 1
bind -T tab-mode 2 select-window -t 2
bind -T tab-mode 3 select-window -t 3
bind -T tab-mode 4 select-window -t 4
bind -T tab-mode 5 select-window -t 5
bind -T tab-mode Escape set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T tab-mode Enter set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T tab-mode q set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root

# --- RESIZE mode (Ctrl+n) ---
bind -n C-n set status-right '#(tmux-shortcut-helper --mode-status resize)' \; switch-client -T resize-mode
bind -T resize-mode h resize-pane -L 5 \; switch-client -T resize-mode
bind -T resize-mode j resize-pane -D 5 \; switch-client -T resize-mode
bind -T resize-mode k resize-pane -U 5 \; switch-client -T resize-mode
bind -T resize-mode l resize-pane -R 5 \; switch-client -T resize-mode
bind -T resize-mode Escape set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T resize-mode Enter set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T resize-mode q set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root

# --- MOVE mode (Ctrl+h) ---
bind -n C-h set status-right '#(tmux-shortcut-helper --mode-status move)' \; switch-client -T move-mode
bind -T move-mode h select-pane -L \; switch-client -T move-mode
bind -T move-mode j select-pane -D \; switch-client -T move-mode
bind -T move-mode k select-pane -U \; switch-client -T move-mode
bind -T move-mode l select-pane -R \; switch-client -T move-mode
bind -T move-mode Tab select-pane -t :.+
bind -T move-mode Escape set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T move-mode Enter set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T move-mode q set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root

# --- SESSION mode (Ctrl+o) ---
bind -n C-o set status-right '#(tmux-shortcut-helper --mode-status session)' \; switch-client -T session-mode
bind -T session-mode d detach-client
bind -T session-mode w choose-tree -Zs
bind -T session-mode r command-prompt -I "#S" "rename-session '%%'"
bind -T session-mode Escape set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T session-mode Enter set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root
bind -T session-mode q set status-right '#(tmux-shortcut-helper --zellij)' \; switch-client -T root

# --- SEARCH mode (Ctrl+s) ---
bind -n C-s copy-mode \; send-keys ?

# --- QUIT (Ctrl+q) ---
bind -n C-q confirm-before -p "Detach from session? (y/n)" detach-client
```

Reload config:

```bash
tmux source-file ~/.tmux.conf
```

## Keybindings

| Key | Mode | Description |
|-----|------|-------------|
| `Ctrl+p` | PANE | Pane operations (split, close, move, resize) |
| `Ctrl+t` | TAB | Tab/window operations (new, close, rename, switch) |
| `Ctrl+n` | RESIZE | Resize current pane |
| `Ctrl+h` | MOVE | Move between panes |
| `Ctrl+o` | SESSION | Session operations (detach, list, rename) |
| `Ctrl+s` | SEARCH | Enter copy/search mode |
| `Ctrl+q` | QUIT | Detach from session |
| `Esc` | - | Return to normal mode |

## CLI Options

```bash
tmux-shortcut-helper                    # Default zellij-style status bar
tmux-shortcut-helper --mode-status pane # PANE mode status bar
tmux-shortcut-helper --popup pane       # Popup display for pane shortcuts
tmux-shortcut-helper --no-color         # Disable colors
tmux-shortcut-helper --help             # Show help
```

## License

MIT
