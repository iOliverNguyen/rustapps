use crate::VsCodeTheme;
use anyhow::Result;
use gpui::{Hsla, Rgba};
use std::fs::File;
use theme::BaseColors;
use tracing::{error, info};

pub struct Convertor {
    pub vscode_theme: VsCodeTheme,
}

impl Convertor {
    pub fn new(input_path: &str) -> Result<Self> {
        let vscode_theme = Self::load_vscode_theme(input_path)?;
        Ok(Self { vscode_theme })
    }

    fn load_vscode_theme(input_path: &str) -> Result<VsCodeTheme> {
        let input_file = File::open(input_path).or_else(|err| {
            error!("Failed to open input file: {}", err);
            Err(err)
        })?;

        let vscode_theme: VsCodeTheme =
            serde_json_lenient::from_reader(input_file).or_else(|err| {
                error!("Failed to parse vscode theme: {}", err);
                Err(err)
            })?;

        info!(
            message = "Successfully parsed vscode theme",
            name = vscode_theme.name
        );
        Ok(vscode_theme)
    }

    fn to_base(&self) -> BaseColors {
        let x = &self.vscode_theme.colors;
        BaseColors {
            background: color(x.editor.background),
            foreground: color(x.editor.background),
            border: color(x.editor_group.border),
        }
    }

    fn to_terminal() -> TerminalColors {
        TerminalColors {
            background: todo!(),
            border: todo!(),
            foreground: todo!(),
            ansi_black: todo!(),
            ansi_blue: todo!(),
            ansi_bright_black: todo!(),
            ansi_bright_blue: todo!(),
            ansi_bright_cyan: todo!(),
            ansi_bright_green: todo!(),
            ansi_bright_magenta: todo!(),
            ansi_bright_red: todo!(),
            ansi_bright_white: todo!(),
            ansi_bright_yellow: todo!(),
            ansi_cyan: todo!(),
            ansi_green: todo!(),
            ansi_magenta: todo!(),
            ansi_red: todo!(),
            ansi_white: todo!(),
            ansi_yellow: todo!(),
            selection_background: todo!(),
            selection_foreground: todo!(),
            inactive_selection_background: todo!(),
            find_match_background: todo!(),
            find_match_border: todo!(),
            find_match_highlight_background: todo!(),
            find_match_highlight_border: todo!(),
            hover_highlight_background: todo!(),
            drop_background: todo!(),
            tab_active_border: todo!(),
        }
    }
}

fn color(s: Option<String>) -> Hsla {
    let a: Rgba = s.into();
    a.into()
}
