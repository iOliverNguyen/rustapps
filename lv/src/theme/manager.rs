use super::*;

pub struct ThemeManager {
    pub appearance: Appearance,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            appearance: Appearance::Dark,
        }
    }
}
