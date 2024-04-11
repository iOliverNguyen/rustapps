use gpui::*;

use super::*;

pub struct Workspace {
    focus_handle: FocusHandle,

    title_bar: View<TitleBar>,
    status_bar: View<StatusBar>,
    central: View<Central>,
}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            title_bar: cx.new_view(|cx| TitleBar::new(cx)),
            status_bar: cx.new_view(|cx| StatusBar::new(cx)),
            central: cx.new_view(|cx| Central::new(cx)),
        }
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("app")
            .size_full()
            .bg(rgb(0x888888))
            .flex()
            .flex_col()
            .child(self.title_bar.clone())
            .child(self.central.clone())
            .child(self.status_bar.clone())
    }
}
