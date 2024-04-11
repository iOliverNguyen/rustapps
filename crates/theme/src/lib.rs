mod colors;
mod helpers;
mod themes;

pub use colors::*;
pub use themes::*;

pub enum Appearance {
    Light,
    Dark,
}

pub struct Theme {
    pub name: Option<String>,
    pub appearance: Appearance,
    pub colors: ThemeColors,
}
