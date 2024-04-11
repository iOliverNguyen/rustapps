use crate::*;
use gpui::*;
use gpui_ext::*;

pub struct ColorSlider {
    focus_handle: FocusHandle,
}

impl ColorSlider {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for ColorSlider {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorSlider {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(cx);

        div()
            .w_full()
            .h(px(10.))
            .child(ColorSliderElement::new(self.focus_handle.clone(), focused))
    }
}
