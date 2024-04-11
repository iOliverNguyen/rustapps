use super::*;
use gpui::*;

pub struct RightPanel {
    focus_handle: FocusHandle,
}

impl RightPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for RightPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RightPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(rems(20.))
            .h_full()
            .bg(rgb(0x448844))
            .child("Left panel")
    }
}
