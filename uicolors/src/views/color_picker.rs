use gpui::*;
use gpui_ext::*;

pub struct ColorPicker {
    focus_handle: FocusHandle,
}

impl ColorPicker {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for ColorPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorPicker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(38.))
            .flex_center()
            .bg(rgb(0x222222))
            .child("color picker")
    }
}
