use gpui::Hsla;

pub struct ThemeColors {
    pub base: BaseColors,
    pub button: ButtonColors,
    pub editor: EditorColors,
    pub input: EditorColors,
    pub side_bar: SideBarColors,
    pub status_bar: StatusBarColors,
    pub tab: TabColors,
    pub terminal: TerminalColors,
    pub title_bar: TitleBarColors,
}

pub struct BaseColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
}

pub struct ButtonColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
    pub active_background: Hsla,
    pub active_foreground: Hsla,
    pub active_border: Hsla,
}

pub struct InputColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
    pub active_border: Hsla,
    pub selection_background: Hsla,
    pub selection_foreground: Hsla,
    pub placeholder: Hsla,
    pub cursor: Hsla,
}

pub struct EditorColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
    pub selection_background: Hsla,
    pub selection_foreground: Hsla,
}

pub struct TabColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
    pub active_background: Hsla,
    pub active_foreground: Hsla,
    pub active_border: Hsla,
}

pub struct TitleBarColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub active_background: Hsla,
    pub active_foreground: Hsla,
    pub border: Hsla,
}

pub struct SideBarColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
    pub selection_background: Hsla,
    pub selection_foreground: Hsla,
    pub title_background: Hsla,
    pub title_foreground: Hsla,
}

pub struct StatusBarColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
}

pub struct TerminalColors {
    pub ansi_black: Hsla,
    pub ansi_red: Hsla,
    pub ansi_green: Hsla,
    pub ansi_yellow: Hsla,
    pub ansi_blue: Hsla,
    pub ansi_magenta: Hsla,
    pub ansi_cyan: Hsla,
    pub ansi_white: Hsla,
    pub ansi_bright_black: Hsla,
    pub ansi_bright_red: Hsla,
    pub ansi_bright_green: Hsla,
    pub ansi_bright_yellow: Hsla,
    pub ansi_bright_blue: Hsla,
    pub ansi_bright_magenta: Hsla,
    pub ansi_bright_cyan: Hsla,
    pub ansi_bright_white: Hsla,
    pub background: Hsla,
    pub foreground: Hsla,
    pub selection_background: Hsla,
}
