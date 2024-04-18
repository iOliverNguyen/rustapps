use super::*;
use crate::*;
use gpui::*;
use std::sync::Arc;

pub struct Workspace {
    app_state: Arc<AppState>,
    title_bar: View<TitleBar>,
    left_panel: View<LeftPanel>,
    central: View<Central>,
    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let app_state = Arc::new(AppState {});
        let weak_handle = cx.view().downgrade();
        let workspace = Self {
            app_state: app_state.clone(),
            title_bar: cx.new_view(|cx| TitleBar::new(cx)),
            left_panel: cx
                .new_view(|cx| LeftPanel::new(cx, weak_handle.clone(), app_state.clone())),
            central: cx.new_view(|cx| Central::new(cx, weak_handle.clone(), app_state.clone())),
            focus_handle: cx.focus_handle(),
        };

        workspace
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
            .key_context("Workspace")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(rgba(0x000000))
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
    }
}
