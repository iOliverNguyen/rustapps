use gpui::{Context, Model, ModelContext};

use crate::{ColorFormat, ColorLibrary, ColorPalette};

pub enum MainTab {
    Home,
    Browse,
    Favorites,
}

pub struct AppState {
    pub color: Model<ColorFormat>,
    pub palette: Model<ColorPalette>,
    pub main_tab: Model<MainTab>,
    pub library: Model<Option<ColorLibrary>>,
}

pub enum Event {
    InputColor(ColorFormat),
}
