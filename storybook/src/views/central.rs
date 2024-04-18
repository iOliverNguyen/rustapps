use crate::{AppState, Assets, Workspace};
use gpui::*;
use gpui_ext::*;
use std::sync::Arc;

pub struct Central {
    app_state: Arc<AppState>,
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
}

impl Central {
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

impl FocusableView for Central {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Central {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(div().size_full().bg(rgb(0x8888aa)))
    }
}
