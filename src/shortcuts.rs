pub struct Shortcut {
    pub key: &'static str,
    pub desc_short: &'static str,
    pub desc_long: &'static str,
}

pub struct Category {
    pub name: &'static str,
    pub key: &'static str,  // trigger key for popup
    pub shortcuts: &'static [Shortcut],
}

pub const TAB: Category = Category {
    name: "TAB",
    key: "W",
    shortcuts: &[
        Shortcut { key: "c", desc_short: "+", desc_long: "新規作成" },
        Shortcut { key: "n", desc_short: "→", desc_long: "次へ" },
        Shortcut { key: "p", desc_short: "←", desc_long: "前へ" },
        Shortcut { key: ",", desc_short: "名", desc_long: "名前変更" },
        Shortcut { key: "&", desc_short: "×", desc_long: "閉じる" },
        Shortcut { key: "w", desc_short: "一覧", desc_long: "一覧表示" },
        Shortcut { key: "0-9", desc_short: "#", desc_long: "番号で選択" },
    ],
};

pub const PANE: Category = Category {
    name: "PANE",
    key: "Q",
    shortcuts: &[
        Shortcut { key: "%", desc_short: "┃", desc_long: "縦に分割" },
        Shortcut { key: "\"", desc_short: "━", desc_long: "横に分割" },
        Shortcut { key: "x", desc_short: "×", desc_long: "閉じる" },
        Shortcut { key: "z", desc_short: "⤢", desc_long: "全画面切替" },
        Shortcut { key: "o", desc_short: "↻", desc_long: "次のペインへ" },
        Shortcut { key: "hjkl", desc_short: "移動", desc_long: "ペイン間移動" },
        Shortcut { key: "HJKL", desc_short: "サイズ", desc_long: "サイズ変更" },
    ],
};

pub const SESSION: Category = Category {
    name: "SESS",
    key: "S",
    shortcuts: &[
        Shortcut { key: "d", desc_short: "離", desc_long: "デタッチ" },
        Shortcut { key: "s", desc_short: "一覧", desc_long: "セッション一覧" },
        Shortcut { key: "$", desc_short: "名", desc_long: "名前変更" },
        Shortcut { key: "(", desc_short: "←", desc_long: "前のセッション" },
        Shortcut { key: ")", desc_short: "→", desc_long: "次のセッション" },
    ],
};

pub const COPY: Category = Category {
    name: "COPY",
    key: "[",
    shortcuts: &[
        Shortcut { key: "[", desc_short: "開始", desc_long: "コピーモード開始" },
        Shortcut { key: "]", desc_short: "貼付", desc_long: "ペースト" },
        Shortcut { key: "Space", desc_short: "選択", desc_long: "選択開始" },
        Shortcut { key: "Enter", desc_short: "コピー", desc_long: "コピー" },
        Shortcut { key: "q", desc_short: "終了", desc_long: "コピーモード終了" },
    ],
};

pub const QUICK: Category = Category {
    name: "QUICK",
    key: "",
    shortcuts: &[
        Shortcut { key: "d", desc_short: "Detach", desc_long: "セッションから離脱" },
        Shortcut { key: "?", desc_short: "Help", desc_long: "ヘルプ表示" },
    ],
};

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Default,
    Tab,
    Pane,
    Session,
    Copy,
}

impl Mode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" | "all" => Some(Mode::Default),
            "tab" | "window" | "win" | "t" | "w" => Some(Mode::Tab),
            "pane" | "p" => Some(Mode::Pane),
            "session" | "sess" | "s" => Some(Mode::Session),
            "copy" | "c" => Some(Mode::Copy),
            _ => None,
        }
    }

    pub fn get_category(&self) -> &'static Category {
        match self {
            Mode::Default => &TAB,
            Mode::Tab => &TAB,
            Mode::Pane => &PANE,
            Mode::Session => &SESSION,
            Mode::Copy => &COPY,
        }
    }
}
