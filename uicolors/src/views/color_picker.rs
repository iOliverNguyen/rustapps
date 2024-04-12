use super::*;
use gpui::*;
use gpui_ext::*;

pub struct ColorPicker {
    focus_handle: FocusHandle,
    color_slider: View<ColorSlider>,
}

impl ColorPicker {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            color_slider: ColorSlider::new(cx),
        }
    }
}

impl FocusableView for ColorPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorPicker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(38.))
            .flex_center()
            .bg(rgb(0xffffff))
            .child(self.color_slider.clone())
    }
}
