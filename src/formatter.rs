use crate::shortcuts::Mode;

pub struct Formatter {
    pub use_color: bool,
}

impl Formatter {
    pub fn new(use_color: bool, _prefix: &str) -> Self {
        Self { use_color }
    }

    fn format_mode_button(&self, key: &str, name: &str) -> String {
        if self.use_color {
            format!(
                "#[fg=red,bold]<{}>#[default] #[fg=white]{}#[default]",
                key, name
            )
        } else {
            format!("<{}> {}", key, name)
        }
    }

    fn format_key(&self, key: &str, desc: &str) -> String {
        if self.use_color {
            format!(
                "#[fg=yellow,bold]<{}>#[default] #[fg=white]{}#[default]",
                key, desc
            )
        } else {
            format!("<{}> {}", key, desc)
        }
    }

    fn format_mode_header(&self, name: &str) -> String {
        if self.use_color {
            format!("#[bg=green,fg=black,bold] {} #[default]", name)
        } else {
            format!("[{}]", name)
        }
    }

    /// Default: Zellij-style status bar
    pub fn format_zellij(&self) -> String {
        let prefix = if self.use_color {
            "#[bg=colour238,fg=green,bold] Ctrl + #[default]"
        } else {
            "Ctrl +"
        };

        let modes = [
            self.format_mode_button("p", "PANE"),
            self.format_mode_button("t", "TAB"),
            self.format_mode_button("n", "RESIZE"),
            self.format_mode_button("b", "MOVE"),
            self.format_mode_button("s", "SEARCH"),
            self.format_mode_button("o", "SESSION"),
            self.format_mode_button("q", "QUIT"),
        ];

        format!("{}  {}", prefix, modes.join("  "))
    }

    /// PANE mode status bar
    pub fn format_pane_mode(&self) -> String {
        let header = self.format_mode_header("PANE");
        let keys = [
            self.format_key("n", "Split→"),
            self.format_key("d", "Split↓"),
            self.format_key("x", "Close"),
            self.format_key("f", "Full"),
            self.format_key("hjkl", "Move"),
            self.format_key("r", "Resize"),
            self.format_key("Esc", "Back"),
        ];
        format!("{}  {}", header, keys.join("  "))
    }

    /// TAB mode status bar
    pub fn format_tab_mode(&self) -> String {
        let header = self.format_mode_header("TAB");
        let keys = [
            self.format_key("n", "New"),
            self.format_key("x", "Close"),
            self.format_key("r", "Rename"),
            self.format_key("h/l", "←/→"),
            self.format_key("1-9", "Select"),
            self.format_key("Esc", "Back"),
        ];
        format!("{}  {}", header, keys.join("  "))
    }

    /// RESIZE mode status bar
    pub fn format_resize_mode(&self) -> String {
        let header = self.format_mode_header("RESIZE");
        let keys = [
            self.format_key("h", "←"),
            self.format_key("j", "↓"),
            self.format_key("k", "↑"),
            self.format_key("l", "→"),
            self.format_key("=", "Equal"),
            self.format_key("Esc", "Back"),
        ];
        format!("{}  {}", header, keys.join("  "))
    }

    /// MOVE mode status bar
    pub fn format_move_mode(&self) -> String {
        let header = self.format_mode_header("MOVE");
        let keys = [
            self.format_key("h", "←"),
            self.format_key("j", "↓"),
            self.format_key("k", "↑"),
            self.format_key("l", "→"),
            self.format_key("Tab", "Next"),
            self.format_key("Esc", "Back"),
        ];
        format!("{}  {}", header, keys.join("  "))
    }

    /// SESSION mode status bar
    pub fn format_session_mode(&self) -> String {
        let header = self.format_mode_header("SESSION");
        let keys = [
            self.format_key("d", "Detach"),
            self.format_key("w", "List"),
            self.format_key("r", "Rename"),
            self.format_key("Esc", "Back"),
        ];
        format!("{}  {}", header, keys.join("  "))
    }

    /// Mode-specific status bar
    pub fn format_mode_status(&self, mode: &str) -> String {
        match mode {
            "pane" => self.format_pane_mode(),
            "tab" => self.format_tab_mode(),
            "resize" => self.format_resize_mode(),
            "move" => self.format_move_mode(),
            "session" => self.format_session_mode(),
            _ => self.format_zellij(),
        }
    }

    /// Popup用: 罫線付きの詳細表示
    pub fn format_popup(&self, mode: Mode) -> String {
        let category = mode.get_category();
        let width = 30;

        let mut lines = Vec::new();

        let title = format!(" {} ", category.name);
        let padding = width - 2 - title.len();
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;
        lines.push(format!("┌{}{}{}┐", "─".repeat(left_pad), title, "─".repeat(right_pad)));

        lines.push(format!("│{}│", " ".repeat(width - 2)));

        for shortcut in category.shortcuts {
            let key_display = format!("  {}  ", shortcut.key);
            let desc = shortcut.desc_long;
            let content = format!("{}{}", key_display, desc);
            let padding = width - 2 - Self::display_width(&content);
            lines.push(format!("│{}{}│", content, " ".repeat(padding.max(0))));
        }

        lines.push(format!("│{}│", " ".repeat(width - 2)));

        let footer = "  Press q to close";
        let padding = width - 2 - footer.len();
        lines.push(format!("│{}{}│", footer, " ".repeat(padding)));

        lines.push(format!("└{}┘", "─".repeat(width - 2)));

        lines.join("\n")
    }

    fn display_width(s: &str) -> usize {
        s.chars().map(|c| {
            if c.is_ascii() { 1 } else { 2 }
        }).sum()
    }
}
