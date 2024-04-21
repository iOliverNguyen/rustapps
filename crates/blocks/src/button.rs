use crate::Icon;
use crate::ThemeAccess;
use crate::ThemeSource;
use crate::ThemeStore;
use crate::ThemeVariant;
use gpui::{prelude::FluentBuilder, *};
use std::marker::PhantomData;

pub struct ButtonAttrs {
    pub text: Option<SharedString>,
    pub icon: Option<Icon>,
}

impl Default for ButtonAttrs {
    fn default() -> Self {
        Self {
            text: None,
            icon: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ButtonStates {
    pub selected: bool,
    pub disabled: bool,
    pub hovered: bool,
    pub focused: bool,
    pub pressed: bool,
    pub dragged: bool,
}

impl Default for ButtonStates {
    fn default() -> Self {
        Self {
            selected: false,
            disabled: false,
            hovered: false,
            focused: false,
            pressed: false,
            dragged: false,
        }
    }
}

// https://m3.material.io/components/all-buttons
#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Basic,
    Elevated,
    Filled,
    FilledTonal,
    Outline,
}

pub trait GenericButtonStyles {
    fn styles(&self, states: ButtonStates) -> ButtonStyles;
}

#[derive(Default)]
pub struct ButtonStyles {
    pub background: Hsla,
    pub label_color: Hsla,
    pub icon_color: Hsla,
    pub border_color: Hsla,
    //
    // more customization?
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Basic
    }
}

pub struct ButtonElements {
    pub base: Div,
    pub text: Option<Div>,
    pub icon: Option<Div>,
}

#[derive(IntoElement)]
pub struct Button<T: ThemeSource> {
    pub elems: ButtonElements,
    pub attrs: ButtonAttrs,
    pub states: ButtonStates,
    pub variant: ButtonVariant,
    pub styles: ButtonStyles,
    pub theme_variant: T::VARIANT,

    _phantom: PhantomData<T>,
}

impl<T: ThemeSource> Button<T> {
    fn new() -> Self {
        Self {
            elems: ButtonElements {
                base: div(),
                text: None,
                icon: None,
            },
            attrs: ButtonAttrs::default(),
            states: ButtonStates::default(),
            variant: ButtonVariant::default(),
            styles: ButtonStyles::default(),
            theme_variant: T::VARIANT::default(),
            _phantom: PhantomData,
        }
    }

    pub fn new_text(text: impl Into<SharedString>) -> Self {
        let mut button = Self::new();
        button.attrs.text = Some(text.into());
        button
    }
}

impl<T: ThemeSource> RenderOnce for Button<T> {
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
        let theme: &ThemeStore<T> = cx.theme();
        let style = theme.variant(self.theme_variant);

        let styles = style.apply_button_styles(self.variant, self.states);

        let elems = self.elems;
        elems
            .base
            .w(px(100.))
            .h(px(40.))
            .hover(|st| {
                let mut states = self.states;
                states.hovered = true;
                let x = style.apply_button_styles(self.variant, states);
                st.bg(x.background)
            })
            .when_some(self.attrs.text, |base, text| base.child(div().child(text)))
            .when_some(self.attrs.icon, |base, icon| base.child(div()))
    }
}

// how to store state?
// how to customize style?
// how to handle events?
pub struct ToggleButton {}
