use super::*;
use gpui::*;
use gpui_ext::*;

pub struct Central {
    focus_handle: FocusHandle,

    color_input: View<ColorInput>,
    color_palette: View<ColorPalette>,
}

impl Central {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),

            color_input: cx.new_view(|cx| ColorInput::new(cx)),
            color_palette: cx.new_view(|cx| ColorPalette::new(cx)),
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
            .flex_col()
            .child(
                div()
                    .h(rems(10.))
                    .w_full()
                    .flex_center()
                    .child("Select a color or press spacebar for random one."),
            )
            .child(div().flex_center().child(self.color_input.clone()))
            .child(div().size_full().flex_center().child("central"))
    }
}
