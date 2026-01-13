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
    --zellij               Default status bar (Ctrl + modes)
    --mode-status <MODE>   Mode-specific status bar (pane/tab/resize/move/session)
    --popup <MODE>         Show popup for mode
    --no-color             Disable color output
    -h, --help             Show this help message
    -v, --version          Show version

ZELLIJ-STYLE KEYBINDINGS:
    Ctrl+p    PANE mode
    Ctrl+t    TAB mode
    Ctrl+n    RESIZE mode
    Ctrl+h    MOVE mode
    Ctrl+s    SEARCH mode
    Ctrl+o    SESSION mode
    Ctrl+q    QUIT

    In any mode, press Esc/Enter/q to return to normal.
"#,
        VERSION
    );
}

fn print_version() {
    println!("tmux-shortcut-helper v{}", VERSION);
}

enum OutputMode {
    Zellij,
    ModeStatus(String),
    Popup(Mode),
}

struct Args {
    use_color: bool,
    output_mode: OutputMode,
    show_help: bool,
    show_version: bool,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut args = env::args().skip(1);
        let mut result = Args {
            use_color: true,
            output_mode: OutputMode::Zellij,
            show_help: false,
            show_version: false,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => result.show_help = true,
                "-v" | "--version" => result.show_version = true,
                "--no-color" => result.use_color = false,
                "--zellij" => result.output_mode = OutputMode::Zellij,
                "--mode-status" => {
                    let mode = args.next().ok_or("--mode-status requires a value")?;
                    result.output_mode = OutputMode::ModeStatus(mode);
                }
                "--popup" => {
                    let mode_str = args.next().ok_or("--popup requires a value")?;
                    let mode = Mode::from_str(&mode_str)
                        .ok_or_else(|| format!("Unknown mode: {}", mode_str))?;
                    result.output_mode = OutputMode::Popup(mode);
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

    match args.output_mode {
        OutputMode::Zellij => {
            println!("{}", formatter.format_zellij());
        }
        OutputMode::ModeStatus(mode) => {
            println!("{}", formatter.format_mode_status(&mode));
        }
        OutputMode::Popup(mode) => {
            println!("{}", formatter.format_popup(mode));
            wait_for_key();
        }
    }
}
