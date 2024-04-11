use gpui::*;

use super::*;

pub struct Workspace {
    focus_handle: FocusHandle,

    title_bar: View<ColorPicker>,
    status_bar: View<StatusBar>,
    left_panel: View<LeftPanel>,
    central: View<Central>,
}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            title_bar: cx.new_view(|cx| ColorPicker::new(cx)),
            status_bar: cx.new_view(|cx| StatusBar::new(cx)),
            left_panel: cx.new_view(|cx| LeftPanel::new(cx)),
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
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_row()
                    .child(self.left_panel.clone())
                    .child(self.central.clone()),
            )
            .child(self.status_bar.clone())
    }
}
