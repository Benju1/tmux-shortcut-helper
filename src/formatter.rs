use crate::shortcuts::Mode;

pub struct Formatter {
    pub use_color: bool,
}

impl Formatter {
    pub fn new(use_color: bool, _prefix: &str) -> Self {
        Self { use_color }
    }

    /// Zellij-style: Ctrl + <key> MODE format
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

    /// Zellij-style status bar
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
            self.format_mode_button("h", "MOVE"),
            self.format_mode_button("s", "SEARCH"),
            self.format_mode_button("o", "SESSION"),
            self.format_mode_button("q", "QUIT"),
        ];

        format!("{}  {}", prefix, modes.join("  "))
    }

    /// Popup用: 罫線付きの詳細表示
    pub fn format_popup(&self, mode: Mode) -> String {
        let category = mode.get_category();
        let width = 30;

        let mut lines = Vec::new();

        // Top border with title
        let title = format!(" {} ", category.name);
        let padding = width - 2 - title.len();
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;
        lines.push(format!("┌{}{}{}┐", "─".repeat(left_pad), title, "─".repeat(right_pad)));

        // Empty line
        lines.push(format!("│{}│", " ".repeat(width - 2)));

        // Shortcuts
        for shortcut in category.shortcuts {
            let key_display = format!("  {}  ", shortcut.key);
            let desc = shortcut.desc_long;
            let content = format!("{}{}", key_display, desc);
            let padding = width - 2 - Self::display_width(&content);
            lines.push(format!("│{}{}│", content, " ".repeat(padding.max(0))));
        }

        // Empty line
        lines.push(format!("│{}│", " ".repeat(width - 2)));

        // Footer
        let footer = "  Press q to close";
        let padding = width - 2 - footer.len();
        lines.push(format!("│{}{}│", footer, " ".repeat(padding)));

        // Bottom border
        lines.push(format!("└{}┘", "─".repeat(width - 2)));

        lines.join("\n")
    }

    /// 表示幅を計算（日本語文字は2幅）
    fn display_width(s: &str) -> usize {
        s.chars().map(|c| {
            if c.is_ascii() { 1 } else { 2 }
        }).sum()
    }

    /// 従来のformat関数
    pub fn format(&self, mode: Mode) -> String {
        match mode {
            Mode::Default => self.format_zellij(),
            _ => self.format_popup(mode),
        }
    }
}
