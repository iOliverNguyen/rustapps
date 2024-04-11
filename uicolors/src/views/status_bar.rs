use gpui::*;
use gpui_ext::*;

use super::Workspace;

pub struct StatusBar {
    focus_handle: FocusHandle,
}

impl StatusBar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for StatusBar {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_8()
            .flex_center()
            .bg(rgb(0x887722))
            .child("status bar")
    }
}
