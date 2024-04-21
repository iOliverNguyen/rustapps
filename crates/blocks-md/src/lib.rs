use std::default;

use blocks::{ButtonStyles, ThemeSource, ThemeVariant};

// material design theme
#[derive(Clone)]
pub struct MdTheme {}

#[derive(Default, Clone)]
pub enum MdVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Tertiary,
}

impl MdTheme {
    pub fn new() -> Self {
        Self {}
    }
}

impl ThemeSource for MdTheme {
    type VARIANT = MdVariant;

    fn variant(&self, variant: MdVariant) -> impl ThemeVariant {
        MdThemeVariant {}
    }
}

#[derive(Clone)]
pub struct MdThemeVariant {}

impl ThemeVariant for MdThemeVariant {
    fn apply_button_styles(
        &self,
        variant: blocks::ButtonVariant,
        states: blocks::ButtonStates,
    ) -> blocks::ButtonStyles {
        ButtonStyles::default()
    }
}
