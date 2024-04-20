use crate::Icon;
use gpui::*;

pub fn button() -> ButtonBuilder {
    ButtonBuilder::new()
}

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
pub enum ButtonType {
    Basic,
    Elevated,
    Filled,
    FilledTonal,
    Outline,
}

pub struct ButtonStyles {
    pub typ: ButtonType,
}

impl Default for ButtonStyles {
    fn default() -> Self {
        Self {
            typ: ButtonType::Basic,
        }
    }
}

pub struct ButtonElements {
    pub container: Div,
    pub text: Option<Div>,
    pub icon: Option<Div>,
}

pub trait ApplyButtonStyles {
    fn apply_button_styles(self, this: &Button, elems: ButtonElements) -> ButtonElements;
}

impl ApplyButtonStyles for ApplyButtonStylesFn {
    fn apply_button_styles(self, this: &Button, elems: ButtonElements) -> ButtonElements {
        self(this, elems)
    }
}

pub type ApplyButtonStylesFn = fn(this: &Button, elems: ButtonElements) -> ButtonElements;

pub struct ButtonBuilder {
    pub attrs: ButtonAttrs,
    pub styles: ButtonStyles,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Self {
            attrs: ButtonAttrs::default(),
            styles: ButtonStyles::default(),
        }
    }

    pub fn text(mut self, text: SharedString) -> Self {
        self.attrs.text = Some(text);
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.attrs.icon = Some(icon);
        self
    }

    pub fn build<T: Render>(self, cx: &mut ViewContext<T>) -> View<Button> {
        cx.new_view(|cx| Button::new(cx, self.attrs, self.styles))
    }
}

pub struct Button {
    pub attrs: Model<ButtonAttrs>,
    pub states: Model<ButtonStates>,
    pub styles: Model<ButtonStyles>,
    pub apply_style: Option<ApplyButtonStylesFn>,
    pub focus_handle: FocusHandle,
}

impl Button {
    pub fn new(cx: &mut ViewContext<Self>, attrs: ButtonAttrs, styles: ButtonStyles) -> Self {
        Self {
            attrs: cx.new_model(|cx| attrs),
            states: cx.new_model(|cx| ButtonStates::default()),
            styles: cx.new_model(|cx| styles),
            apply_style: None,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for Button {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Button {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let container = div().w(px(100.)).h(px(40.)).bg(rgba(0xaaffaaff));
        let elems = ButtonElements {
            container,
            text: None,
            icon: None,
        };
        let elems = match self.apply_style {
            Some(f) => f(self, elems),
            _ => elems,
        };
        let ButtonElements {
            mut container,
            mut text,
            mut icon,
        } = elems;
        container = match text {
            Some(text) => container.child(text),
            _ => container,
        };
        container = match icon {
            Some(icon) => container.child(icon),
            _ => container,
        };
        container
    }
}
