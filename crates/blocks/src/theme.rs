use gpui::*;
use std::{collections::HashMap, hash::Hash};

use crate::{ButtonStates, ButtonStyles, ButtonVariant};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ThemeId(usize);

pub trait ThemeAccess<T>
where
    T: ThemeSource,
{
    fn theme(&self) -> &ThemeStore<T>;
}

impl<T> ThemeAccess<T> for AppContext
where
    T: ThemeSource,
{
    fn theme(&self) -> &ThemeStore<T> {
        self.global::<ThemeStore<T>>()
    }
}

pub trait ThemeSource: Clone + 'static {
    type VARIANT: Default;

    fn variant(&self, variant: Self::VARIANT) -> impl ThemeVariant;
}

pub trait ThemeVariant: Clone + 'static {
    fn apply_button_styles(&self, variant: ButtonVariant, states: ButtonStates) -> ButtonStyles;
}

#[derive(Clone)]
pub struct ThemeStore<T>
where
    T: ThemeSource,
{
    default_theme: T,
    themes: HashMap<ThemeId, T>,
}

impl<THEME> ThemeStore<THEME>
where
    THEME: ThemeSource,
{
    pub fn new(default_theme: THEME) -> Self {
        Self {
            default_theme,
            themes: HashMap::new(),
        }
    }
    pub fn add_theme(&mut self, id: ThemeId, theme: THEME) {
        if id == ThemeId::default() {
            panic!("cannot add theme with default id");
        }
        self.themes.insert(id, theme);
    }
    pub fn default_theme(&self) -> &THEME {
        &self.default_theme
    }
    pub fn theme_by_id_default(&self, id: ThemeId) -> &THEME {
        if id == ThemeId::default() {
            &self.default_theme
        } else {
            self.themes.get(&id).unwrap_or(&self.default_theme)
        }
    }
    pub fn theme_by_id(&self, id: ThemeId) -> Option<&THEME> {
        if id == ThemeId::default() {
            Some(&self.default_theme)
        } else {
            self.themes.get(&id)
        }
    }
}

impl<T> Global for ThemeStore<T> where T: ThemeSource {}

impl<T> ThemeSource for ThemeStore<T>
where
    T: ThemeSource,
{
    type VARIANT = T::VARIANT;

    fn variant<'a>(&'a self, variant: T::VARIANT) -> impl ThemeVariant {
        self.default_theme.variant(variant)
    }
}
