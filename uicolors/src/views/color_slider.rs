use crate::*;
use gpui::*;
use gpui_ext::*;
use std::sync::Arc;

pub struct ColorSlider {
    focus_handle: FocusHandle,

    pub bounds: Option<Bounds<Pixels>>,
    pub image_data: Option<Arc<ImageData>>,
}

impl ColorSlider {
    pub fn new<T: 'static>(cx: &mut ViewContext<T>) -> View<Self> {
        cx.new_view(|cx| Self {
            focus_handle: cx.focus_handle(),
            bounds: None,
            image_data: None,
        })
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
            .child(ColorSliderElement::new(cx.view()))
    }
}
