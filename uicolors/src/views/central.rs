use super::*;
use crate::AppState;
use gpui::*;
use gpui_ext::*;
use std::sync::Arc;

pub struct Central {
    workspace: WeakView<Workspace>,
    app_state: Arc<AppState>,
    color_input: View<ColorInputView>,
    color_palette: View<ColorPaletteView>,
    focus_handle: FocusHandle,
}

impl Central {
    pub fn new(
        cx: &mut ViewContext<Self>,
        workspace: WeakView<Workspace>,
        app_state: Arc<AppState>,
    ) -> Self {
        let app_state0 = app_state.clone();
        let AppState { color, palette, .. } = app_state0.as_ref();
        Self {
            workspace,
            app_state,
            color_input: cx.new_view(|cx| ColorInputView::new(cx, color.clone(), palette.clone())),
            color_palette: cx.new_view(|cx| ColorPaletteView::new(cx)),
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
        let palette = self.app_state.palette.read(cx);

        div()
            .size_full()
            .bg(rgb(0x000000))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(rems(10.))
                    .w_full()
                    .flex_center()
                    .text_color(palette.at_darkness(400))
                    .child("Select a color or press spacebar for random one."),
            )
            .child(div().flex_center().child(self.color_input.clone()))
            .child(div().size_full().flex_center().child("central"))
    }
}
