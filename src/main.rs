mod formatter;
mod shortcuts;

use formatter::Formatter;
use shortcuts::Mode;
use std::env;
use std::io::{self, Read};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!(
        r#"tmux-shortcut-helper v{}
Zellij-style shortcut helper for tmux

USAGE:
    tmux-shortcut-helper [OPTIONS]

OPTIONS:
    --zellij             Zellij-style status bar (default)
    --popup <MODE>       Show popup for mode (tab/pane/session/copy)
    --no-color           Disable color output
    -h, --help           Show this help message
    -v, --version        Show version

ZELLIJ-STYLE KEYBINDINGS:
    Ctrl+p    PANE mode (split, close, fullscreen)
    Ctrl+t    TAB mode (new, close, rename, navigate)
    Ctrl+n    RESIZE mode (resize panes)
    Ctrl+h    MOVE mode (move between panes)
    Ctrl+s    SEARCH mode (search in scrollback)
    Ctrl+o    SESSION mode (detach, list, rename)
    Ctrl+q    QUIT (detach from session)

    In any mode, press Esc/Enter/q to return to normal.

SETUP (~/.tmux.conf):
    set -g status-right '#(tmux-shortcut-helper --zellij)'
    set -g status-right-length 120

    # See README for full keybinding setup
"#,
        VERSION
    );
}

fn print_version() {
    println!("tmux-shortcut-helper v{}", VERSION);
}

struct Args {
    use_color: bool,
    zellij_mode: bool,
    popup_mode: Option<Mode>,
    show_help: bool,
    show_version: bool,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut args = env::args().skip(1);
        let mut result = Args {
            use_color: true,
            zellij_mode: true,  // default to zellij mode
            popup_mode: None,
            show_help: false,
            show_version: false,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => result.show_help = true,
                "-v" | "--version" => result.show_version = true,
                "--no-color" => result.use_color = false,
                "--zellij" => result.zellij_mode = true,
                "--popup" => {
                    let mode_str = args.next().ok_or("--popup requires a value")?;
                    result.popup_mode = Some(Mode::from_str(&mode_str)
                        .ok_or_else(|| format!("Unknown mode: {}", mode_str))?);
                }
                other => {
                    return Err(format!("Unknown option: {}", other));
                }
            }
        }

        Ok(result)
    }
}

fn wait_for_key() {
    let mut buffer = [0u8; 1];
    let _ = io::stdin().read(&mut buffer);
}

fn main() {
    let args = match Args::parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Run 'tmux-shortcut-helper --help' for usage");
            std::process::exit(1);
        }
    };

    if args.show_help {
        print_help();
        return;
    }

    if args.show_version {
        print_version();
        return;
    }

    let formatter = Formatter::new(args.use_color, "");

    if let Some(popup_mode) = args.popup_mode {
        println!("{}", formatter.format_popup(popup_mode));
        wait_for_key();
    } else {
        println!("{}", formatter.format_zellij());
    }
}
