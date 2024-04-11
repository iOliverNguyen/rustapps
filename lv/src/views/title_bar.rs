use gpui::*;

use crate::gpui_ext::*;

pub struct TitleBar {
    focus_handle: FocusHandle,
}

impl TitleBar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for TitleBar {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TitleBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_10()
            .flex_center()
            .bg(rgb(0x222222))
            .child("title bar")
    }
}
