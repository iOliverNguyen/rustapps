use super::*;
use gpui::*;
use gpui_ext::*;

pub struct Central {
    focus_handle: FocusHandle,
    left_panel: View<LeftPanel>,
    right_panel: View<RightPanel>,
}

impl Central {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            left_panel: cx.new_view(|cx| LeftPanel::new(cx)),
            right_panel: cx.new_view(|cx| RightPanel::new(cx)),
        }
    }
}

impl FocusableView for Central {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Central {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x444444))
            .flex()
            .child(self.left_panel.clone())
            .child(div().size_full().flex_center().child("central"))
            .child(self.right_panel.clone())
    }
}
