use crate::*;
use gpui::*;
use gpui_ext::*;
use tracing::instrument::WithSubscriber;

pub struct ColorInputView {
    c: Model<Hsla>,
    color: Model<ColorFormat>,
    palette: Model<ColorPalette>,
    hue_slider: View<ColorSlider>,
    saturation_slider: View<ColorSlider>,
    lightness_slider: View<ColorSlider>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ColorInputView {
    pub fn new(
        cx: &mut ViewContext<Self>,
        color: Model<ColorFormat>,
        palette: Model<ColorPalette>,
    ) -> Self {
        let c = cx.new_model(|cx| color.read(cx).canonicalize());
        let _subscriptions = vec![
            cx.observe(&color, Self::handle_color_change),
            cx.subscribe(&c, Self::handle_color_slider_event),
        ];

        Self {
            c: c.clone(),
            color,
            palette,
            focus_handle: cx.focus_handle(),
            hue_slider: ColorSlider::new(cx, ColorScale::Hue, c.clone()),
            saturation_slider: ColorSlider::new(cx, ColorScale::Saturation, c.clone()),
            lightness_slider: ColorSlider::new(cx, ColorScale::Lightness, c.clone()),
            _subscriptions,
        }
    }

    fn handle_color_change(&mut self, color: Model<ColorFormat>, cx: &mut ViewContext<Self>) {
        let new_color = color.read(cx).canonicalize();
        self.c.update(cx, |c, cx| {
            *c = new_color;
            cx.notify();
        });
    }

    fn handle_color_slider_event(
        &mut self,
        _: Model<Hsla>,
        event: &ColorSliderEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ColorSliderEvent::ColorChanged(new_color) => {
                self.color.update(cx, |color, cx| {
                    *color = ColorFormat::from(*new_color);
                    cx.notify();
                });
                self.palette.update(cx, |palette, cx| {
                    *palette = ColorPalette::from(*new_color);
                    cx.notify();
                })
            }
        };
        cx.refresh()
    }
}

impl FocusableView for ColorInputView {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorInputView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let color = self.color.read(cx);
        let (h, s, l) = color.split_hsl();
        let palette = self.palette.read(cx);
        let mut fade_label_color = palette.at_darkness(300);
        fade_label_color.fade_out(0.5);

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
                                    .text_color(palette.at_darkness(300))
                                    .rounded(px(24.))
                                    .w(px(700.))
                                    .h(px(48.))
                                    .bg(palette.at_darkness(900))
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .gap(px(20.))
                                    .child(
                                        div()
                                            .h_full()
                                            .border_t_width(px(4.))
                                            .border_l_width(px(32.))
                                            .flex()
                                            .items_center()
                                            .justify_start()
                                            .child(color.to_rgb().to_string().to_uppercase()),
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
                                            .child(
                                                div()
                                                    .border_r_width(px(4.))
                                                    .text_color(fade_label_color)
                                                    .child("H"),
                                            )
                                            .child(format!("{:<3}", h))
                                            .child(
                                                div()
                                                    .border_l_width(px(16.))
                                                    .border_r_width(px(4.))
                                                    .text_color(fade_label_color)
                                                    .child("S"),
                                            )
                                            .child(format!("{:<3}", s))
                                            .child(
                                                div()
                                                    .border_l_width(px(16.))
                                                    .border_r_width(px(4.))
                                                    .text_color(fade_label_color)
                                                    .child("L"),
                                            )
                                            .child(format!("{:<3}", l)),
                                    ),
                            ),
                    )
                    .child(
                        div().size_full().flex_center().child(
                            div()
                                .rounded_full()
                                .w(px(100.))
                                .h(px(100.))
                                .bg(color.canonicalize()),
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
