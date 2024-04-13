use crate::*;
use clap::builder::styling::Color;
use gpui::*;
use gpui_ext::*;
use tracing_subscriber::fmt::init;

pub struct ColorInputView {
    color: Hsla,
    hue_slider: View<ColorSlider>,
    saturation_slider: View<ColorSlider>,
    lightness_slider: View<ColorSlider>,
    focus_handle: FocusHandle,
}

impl ColorInputView {
    pub fn new(cx: &mut ViewContext<Self>, color: Hsla) -> Self {
        Self {
            color,
            focus_handle: cx.focus_handle(),
            hue_slider: ColorSlider::new(cx, ColorScale::Hue, color),
            saturation_slider: ColorSlider::new(cx, ColorScale::Saturation, color),
            lightness_slider: ColorSlider::new(cx, ColorScale::Lightness, color),
        }
    }
}

impl FocusableView for ColorInputView {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorInputView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(px(500.))
            .flex_center()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .h(px(100.))
                    .child(
                        div()
                            .absolute()
                            .top(px(0.))
                            .size_full()
                            .flex_center()
                            .child(
                                div()
                                    .font("Monaspace Xenon")
                                    .rounded(px(24.))
                                    .w(px(400.))
                                    .h(px(48.))
                                    .bg(rgb(0x008888))
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .gap(px(20.))
                                    .child(
                                        div()
                                            .h_full()
                                            .border_t_width(px(4.))
                                            .border_l_width(px(20.))
                                            .flex()
                                            .items_center()
                                            .justify_start()
                                            .child("#abcdef"),
                                    )
                                    .child(
                                        div()
                                            .h_full()
                                            .border_t_width(px(4.))
                                            .border_r_width(px(14.))
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_end()
                                            .child(div().text_color(rgba(0x00000033)).child("H"))
                                            .child("123")
                                            .child(
                                                div()
                                                    .border_l_width(px(4.))
                                                    .text_color(rgba(0x00000033))
                                                    .child("S"),
                                            )
                                            .child("100")
                                            .child(
                                                div()
                                                    .border_l_width(px(4.))
                                                    .text_color(rgba(0x00000033))
                                                    .child("L"),
                                            )
                                            .child("100"),
                                    ),
                            ),
                    )
                    .child(
                        div().size_full().flex_center().child(
                            div()
                                .rounded_full()
                                .w(px(100.))
                                .h(px(100.))
                                .bg(rgb(0x880088)),
                        ),
                    ),
            )
            .child("COLOR")
            .child(self.hue_slider.clone())
            .child(div().h(px(20.)))
            .child(self.saturation_slider.clone())
            .child(div().h(px(20.)))
            .child(self.lightness_slider.clone())
    }
}
