# tmux-shortcut-helper

A lightweight tmux shortcut helper inspired by zellij's friendly UI.

Shows category buttons in the status bar, with detailed popups for each category.

## Status Bar

```
[^b]  <W> TAB  <Q> PANE  <S> SESS  <d> Detach  <?> Help
```

## Popup (e.g., Ctrl+b → Shift+Q)

```
┌─────────── PANE ───────────┐
│                            │
│  %  縦に分割               │
│  "  横に分割               │
│  x  閉じる                 │
│  z  全画面切替             │
│  o  次のペインへ           │
│  hjkl  ペイン間移動        │
│  HJKL  サイズ変更          │
│                            │
│  Press q to close          │
└────────────────────────────┘
```

## Installation

```bash
cargo install --path .
```

## Setup

Add to `~/.tmux.conf`:

```bash
# Status bar
set -g status-right "#(tmux-shortcut-helper)"
set -g status-right-length 80

# Popup keybindings
bind W display-popup -E -w 35 -h 14 "tmux-shortcut-helper --popup tab"
bind Q display-popup -E -w 35 -h 14 "tmux-shortcut-helper --popup pane"
bind S display-popup -E -w 35 -h 12 "tmux-shortcut-helper --popup session"
```

Reload config:

```bash
tmux source-file ~/.tmux.conf
```

## Usage

### Status Bar Keys

| Key | Action |
|-----|--------|
| `Ctrl+b` → `Shift+W` | Show TAB popup |
| `Ctrl+b` → `Shift+Q` | Show PANE popup |
| `Ctrl+b` → `Shift+S` | Show SESSION popup |
| `Ctrl+b` → `d` | Detach from session |
| `Ctrl+b` → `?` | Show tmux help |

### CLI Options

```bash
tmux-shortcut-helper                    # Status bar output
tmux-shortcut-helper --popup pane       # Popup for pane operations
tmux-shortcut-helper --popup tab        # Popup for tab operations
tmux-shortcut-helper --popup session    # Popup for session operations
tmux-shortcut-helper --prefix "^a"      # Custom prefix (for Ctrl+a users)
tmux-shortcut-helper --no-color         # Disable colors
tmux-shortcut-helper --help             # Show help
```

## Shortcut Reference

### TAB (Window)

| Key | Description |
|-----|-------------|
| `c` | New tab |
| `n` | Next tab |
| `p` | Previous tab |
| `,` | Rename |
| `&` | Close |
| `w` | List all |
| `0-9` | Select by number |

### PANE

| Key | Description |
|-----|-------------|
| `%` | Vertical split |
| `"` | Horizontal split |
| `x` | Close |
| `z` | Toggle fullscreen |
| `o` | Next pane |
| `hjkl` | Navigate |
| `HJKL` | Resize |

### SESSION

| Key | Description |
|-----|-------------|
| `d` | Detach |
| `s` | List sessions |
| `$` | Rename |
| `(` `)` | Previous/Next session |

### COPY MODE

| Key | Description |
|-----|-------------|
| `[` | Enter copy mode |
| `]` | Paste |
| `Space` | Start selection |
| `Enter` | Copy |
| `q` | Quit |

## License

MIT
