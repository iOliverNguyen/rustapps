use super::*;
use gpui::*;

pub struct LeftPanel {
    focus_handle: FocusHandle,
}

impl LeftPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for LeftPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LeftPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(rems(20.))
            .h_full()
            .bg(rgb(0x444488))
            .child("left panel")
    }
}
