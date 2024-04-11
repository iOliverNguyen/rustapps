use gpui::{rgb, Hsla};

use crate::*;
use helpers::*;

pub const light_theme: Theme = Theme {
    name: Some("Light".into()),
    appearance: Appearance::Light,
    colors: ThemeColors {
        base: BaseColors{
            background: todo!(),
            foreground: todo!(),
            border: todo!(),
        },
        button: todo!(),
        editor: todo!(),
        input: todo!(),
        side_bar: todo!(),
        status_bar: todo!(),
        tab: todo!(),
        terminal: todo!(),
        title_bar: todo!(),
    }
}

pub const dark_theme: Theme = Theme{
    name: Some("Dark".into()),
    appearance: Appearance::Dark,
    colors: ThemeColors{
        base: BaseColors{
            background: rgb(0x181818),
            foreground: rgb(0xcccccc),
            border: todo!(),
        },
        button: todo!(),
        editor: todo!(),
        input: todo!(),
        side_bar: todo!(),
        status_bar: todo!(),
        tab: todo!(),
        terminal: todo!(),
        title_bar: todo!(),
    },
}


