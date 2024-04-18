use crate::{AppState, Assets, Workspace};
use gpui::*;
use gpui_ext::*;
use std::sync::Arc;

pub struct LeftPanel {
    app_state: Arc<AppState>,
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
}

impl LeftPanel {
    pub fn new(
        cx: &mut ViewContext<Self>,
        workspace: WeakView<Workspace>,
        app_state: Arc<AppState>,
    ) -> Self {
        Self {
            app_state,
            workspace,
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
            .h_full()
            .w(px(200.))
            .child(div().size_full().bg(rgb(0x888888)))
    }
}
