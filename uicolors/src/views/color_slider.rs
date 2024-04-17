use crate::*;
use gpui::*;
use gpui_ext::*;
use image::{Bgra, ImageBuffer};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSliderEvent {
    ColorChanged(Hsla),
}

impl EventEmitter<ColorSliderEvent> for Hsla {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorScale {
    Hue,
    Saturation,
    Lightness,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MouseClickOrDrag {
    Click,
    Drag,
    Release,
}

impl ColorScale {
    fn range(&self) -> (f32, f32) {
        match self {
            ColorScale::Hue => (0., 360.),
            ColorScale::Saturation => (0., 100.),
            ColorScale::Lightness => (0., 100.),
        }
    }

    fn unit(&self, c: Hsla) -> u16 {
        match self {
            ColorScale::Hue => (c.h.clamp(0., 1.) * 360.) as u16,
            ColorScale::Saturation => (c.s.clamp(0., 1.) * 100.) as u16,
            ColorScale::Lightness => (c.l.clamp(0., 1.) * 100.) as u16,
        }
    }

    fn should_gen_image(&self, c0: Hsla, c1: Hsla) -> bool {
        match self {
            ColorScale::Hue => false,
            ColorScale::Lightness => {
                Self::Hue.unit(c0) != Self::Hue.unit(c1)
                    || Self::Saturation.unit(c0) != Self::Saturation.unit(c1)
            }
            ColorScale::Saturation => {
                Self::Hue.unit(c0) != Self::Hue.unit(c1)
                    || Self::Lightness.unit(c0) != Self::Lightness.unit(c1)
            }
        }
    }

    fn gen_image(&self, w: u32, h: u32, padding: u32, color: Hsla) -> ImageData {
        fn cv(f: f32) -> u8 {
            (f * 256.).clamp(0., 255.) as u8
        }

        let rw = w - padding * 2;
        let (x0, x1) = (padding, (w - padding));
        let buffer: ImageBuffer<Bgra<u8>, Vec<u8>> = match self {
            ColorScale::Hue => ImageBuffer::from_fn(w, h, |x, _y| {
                let hue = if x <= x0 || x >= x1 {
                    0.
                } else {
                    (x - x0) as f32 / rw as f32
                };
                let color = hsla(hue, 1., 0.5, 1.);
                let Rgba { r, g, b, a: _ } = color.to_rgb();
                Bgra([cv(b), cv(g), cv(r), 255])
            }),
            ColorScale::Saturation => ImageBuffer::from_fn(w, h, |x, _y| {
                let saturation = if x <= x0 {
                    0.
                } else if x >= x1 {
                    1.
                } else {
                    // (x - x0) as f32 / rw as f32
                    x as f32 / w as f32
                };
                // let saturation = 0.;
                let color = hsla(color.h, saturation, color.l, 1.);
                let Rgba { r, g, b, a: _ } = color.to_rgb();
                Bgra([cv(b), cv(g), cv(r), 255])
            }),
            ColorScale::Lightness => ImageBuffer::from_fn(w, h, |x, _y| {
                let lightness = if x <= x0 {
                    0.
                } else if x >= x1 {
                    1.
                } else {
                    // (x - x0) as f32 / rw as f32
                    x as f32 / w as f32
                };
                // let lightness = 0.;
                let color = hsla(color.h, color.s, lightness, 1.);
                let Rgba { r, g, b, a: _ } = color.to_rgb();
                Bgra([cv(b), cv(g), cv(r), 255])
            }),
        };
        ImageData::new(buffer)
    }
}

pub struct ColorSlider {
    color: Model<Hsla>,
    prev_color: Hsla,
    scale: ColorScale,
    padding: Pixels,
    thumb_size: Pixels,
    bounds: Option<Bounds<Pixels>>,
    image_data: Option<Arc<ImageData>>,

    dragging: bool,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ColorSlider {
    pub fn new<T: 'static>(
        cx: &mut ViewContext<T>,
        scale: ColorScale,
        color: Model<Hsla>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let _subscriptions = vec![cx.observe(&color, Self::handle_color_change)];

            Self {
                color: color.clone(),
                prev_color: color.read(cx).to_owned(),
                scale,
                padding: px(4.),
                thumb_size: px(20.),
                bounds: None,
                image_data: None,
                dragging: false,
                focus_handle: cx.focus_handle(),
                _subscriptions,
            }
        })
    }

    fn handle_color_change(&mut self, color: Model<Hsla>, cx: &mut ViewContext<Self>) {
        let c = color.read(cx).clone();
        if self.scale.should_gen_image(self.prev_color, c) {
            self.image_data = None;
        }
        self.prev_color = c;
    }

    fn handle_mouse_click_or_drag(
        &mut self,
        cx: &mut ViewContext<Self>,
        ev: MouseClickOrDrag,
        pos: Point<Pixels>,
    ) {
        let (padding, thumb_size) = (self.padding, self.thumb_size);
        if let Some(bounds) = self.bounds {
            let (size, origin) = (bounds.size, bounds.origin);
            match ev {
                MouseClickOrDrag::Click => {
                    let dy = (thumb_size - size.height) / 2.;
                    let min_y = origin.y - dy;
                    let max_y = origin.y + size.height + dy;
                    let dx = padding;
                    let min_x = origin.x - dx;
                    let max_x = origin.x + size.width + dx;
                    if !(min_x <= pos.x && pos.x <= max_x && min_y <= pos.y && pos.y <= max_y) {
                        return;
                    }
                    self.dragging = true;
                }
                MouseClickOrDrag::Release => {
                    self.dragging = false;
                    return;
                }
                MouseClickOrDrag::Drag => {
                    if !self.dragging {
                        return;
                    }
                }
            };

            let x = pos.x - bounds.origin.x - padding;
            let w = bounds.size.width - padding * px(2.);
            match self.scale {
                ColorScale::Hue => {
                    let hue = (x / w).clamp(0., 1.);
                    self.color.update(cx, |color, cx| {
                        color.h = hue;
                        cx.notify();
                        cx.emit(ColorSliderEvent::ColorChanged(*color));
                    });
                }
                ColorScale::Saturation => {
                    let saturation = (x / w).clamp(0., 1.);
                    self.color.update(cx, |color, cx| {
                        color.s = saturation;
                        cx.notify();
                        cx.emit(ColorSliderEvent::ColorChanged(*color));
                    });
                }
                ColorScale::Lightness => {
                    let lightness = (x / w).clamp(0., 1.);
                    self.color.update(cx, |color, cx| {
                        color.l = lightness;
                        cx.notify();
                        cx.emit(ColorSliderEvent::ColorChanged(*color));
                    });
                }
            }
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
        let (padding, thumb_size) = (self.padding, self.thumb_size);
        let border = px(3.);
        let focused = self.focus_handle.is_focused(cx);
        let color = self.color.read(cx);
        let value = match self.scale {
            ColorScale::Hue => color.h,
            ColorScale::Saturation => color.s,
            ColorScale::Lightness => color.l,
        };
        let thumb_color = match self.scale {
            ColorScale::Hue => hsla(color.h, 1., 0.5, 1.),
            ColorScale::Saturation => hsla(color.h, color.s, color.l, 1.),
            ColorScale::Lightness => hsla(color.h, color.s, color.l, 1.),
        };
        let thumb_x: Pixels = match self.bounds {
            None => px(0.),
            Some(bounds) => {
                let range = bounds.size.width - padding * px(2.);
                value * range - thumb_size / 2. + padding
            }
        };

        div()
            .w_full()
            .h(thumb_size)
            .flex_center()
            .child(
                div()
                    .w_full()
                    .h(px(10.))
                    .child(ColorSliderElement::new(cx.view())),
            )
            .child(
                div()
                    .absolute()
                    .top(px(0.))
                    .left(thumb_x)
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_size / 2.)
                    .bg(hsla(0., 0., 0.8, 1.))
                    .child(
                        div()
                            .absolute()
                            .top(border)
                            .left(border)
                            .w(thumb_size - border * 2.)
                            .h(thumb_size - border * 2.)
                            .rounded(thumb_size / 2. - border)
                            .bg(thumb_color),
                    ),
            )
    }
}

pub struct ColorSliderElement {
    slider_view: View<ColorSlider>,
    interactivity: Interactivity,
}

impl StatefulInteractiveElement for ColorSliderElement {}

impl InteractiveElement for ColorSliderElement {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl IntoElement for ColorSliderElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl ColorSliderElement {
    pub fn new(slider_view: &View<ColorSlider>) -> Self {
        Self {
            slider_view: slider_view.clone(),
            interactivity: Default::default(),
        }
    }
}

impl Element for ColorSliderElement {
    type BeforeLayout = ();
    type AfterLayout = Option<Arc<ImageData>>;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        // self.interactivity.occlude_mouse();
        let layout_id = self.interactivity.before_layout(cx, |mut style, cx| {
            style.size.width = relative(1.).into();
            style.size.height = relative(1.).into();
            let layout_id = cx.request_layout(&style, None);

            layout_id
        });
        (layout_id, ())
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Self::AfterLayout {
        self.slider_view.update(cx, |view, cx| {
            let dirty = match view.bounds {
                None => true,
                Some(b) => b.size != bounds.size,
            };
            let image_data = if dirty || view.image_data.is_none() {
                Some(Arc::new({
                    let sf = cx.scale_factor();
                    let Size { width, height } = bounds.size;
                    view.scale.gen_image(
                        (width * sf).into(),
                        (height * sf).into(),
                        (view.padding * sf).into(),
                        view.color.read(cx).to_owned(),
                    )
                }))
            } else {
                view.image_data.clone()
            };
            view.bounds = Some(bounds);
            view.image_data = image_data.clone();
            image_data
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        after_layout: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        if let Some(data) = after_layout {
            cx.on_mouse_event({
                let view = self.slider_view.clone();
                move |ev: &MouseDownEvent, phase, cx| {
                    if ev.button == MouseButton::Left {
                        view.update(cx, |view, cx| {
                            view.handle_mouse_click_or_drag(
                                cx,
                                MouseClickOrDrag::Click,
                                ev.position,
                            )
                        })
                    };
                }
            });
            cx.on_mouse_event({
                let view = self.slider_view.clone();
                move |ev: &MouseMoveEvent, phase, cx| {
                    let state = if let Some(MouseButton::Left) = ev.pressed_button {
                        MouseClickOrDrag::Drag
                    } else {
                        MouseClickOrDrag::Release
                    };
                    if view.read(cx).dragging {
                        view.update(cx, |view, cx| {
                            view.handle_mouse_click_or_drag(cx, state, ev.position)
                        });
                    };
                }
            });
            cx.paint_image(bounds, Corners::all(px(0.)), data.clone(), false)
                .log_err();
        }
    }
}
