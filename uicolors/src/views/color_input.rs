use crate::*;
use gpui::*;
use gpui_ext::*;

pub struct ColorInput {
    focus_handle: FocusHandle,

    color_picker: View<ColorPicker>,
}

impl ColorInput {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            color_picker: cx.new_view(|cx| ColorPicker::new(cx)),
        }
    }
}

impl FocusableView for ColorInput {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorInput {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(px(600.))
            .h(px(100.))
            .flex_center()
            .flex_col()
            .bg(rgb(0x222222))
            .child("COLOR")
            .child(self.color_picker.clone())
    }
}
