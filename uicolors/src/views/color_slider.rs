use crate::*;
use gpui::*;
use gpui_ext::*;
use image::{Bgra, ImageBuffer};
use std::sync::Arc;

pub enum ColorScale {
    Hue,
    Saturation,
    Lightness,
}

impl ColorScale {
    fn range(self) -> (f32, f32) {
        match self {
            ColorScale::Hue => (0., 360.),
            ColorScale::Saturation => (0., 100.),
            ColorScale::Lightness => (0., 100.),
        }
    }

    fn gen_image(&self, w: f32, h: f32, padding: f32, color: Hsla) -> ImageData {
        fn cv(f: f32) -> u8 {
            (f * 256.).clamp(0., 255.) as u8
        }

        let (rw, rh) = (w - padding * 2., h - padding * 2.);
        let (x0, x1) = (padding as u32, (w - padding) as u32);
        let buffer: ImageBuffer<Bgra<u8>, Vec<u8>> = match self {
            ColorScale::Hue => ImageBuffer::from_fn(w as u32, h as u32, |x, y| {
                let hue = if x <= x0 || x >= x1 {
                    0.
                } else {
                    x as f32 / rw
                };
                let color = hsla(hue, 1., 0.5, 1.);
                let Rgba { r, g, b, a } = color.to_rgb();
                Bgra([cv(b), cv(g), cv(r), 255])
            }),
            ColorScale::Lightness => ImageBuffer::from_fn(w as u32, h as u32, |x, y| {
                let lightness = if x <= x0 {
                    0.
                } else if x >= x1 {
                    1.
                } else {
                    x as f32 / rw
                };
                let color = hsla(color.h, color.s, lightness, 1.);
                let Rgba { r, g, b, a } = color.to_rgb();
                Bgra([cv(b), cv(g), cv(r), 255])
            }),
            ColorScale::Saturation => ImageBuffer::from_fn(w as u32, h as u32, |x, y| {
                let saturation = if x <= x0 {
                    0.
                } else if x >= x1 {
                    1.
                } else {
                    x as f32 / rw
                };
                let color = hsla(color.h, saturation, color.l, 1.);
                let Rgba { r, g, b, a } = color.to_rgb();
                Bgra([cv(b), cv(g), cv(r), 255])
            }),
        };
        ImageData::new(buffer)
    }
}

pub struct ColorSlider {
    pub color: Hsla,
    pub scale: ColorScale,
    pub value: f32,
    pub padding: f32,
    pub bounds: Option<Bounds<Pixels>>,
    pub image_data: Option<Arc<ImageData>>,

    focus_handle: FocusHandle,
}

impl ColorSlider {
    pub fn new<T: 'static>(cx: &mut ViewContext<T>, scale: ColorScale, color: Hsla) -> View<Self> {
        cx.new_view(|cx| Self {
            color,
            scale,
            value: 0.,
            padding: 10.,
            bounds: None,
            image_data: None,
            focus_handle: cx.focus_handle(),
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
        let size = 20.;
        let border = 3.;
        let focused = self.focus_handle.is_focused(cx);

        div()
            .w_full()
            .h(px(size))
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
                    .left(px(-1.))
                    .w(px(size))
                    .h(px(size))
                    .rounded(px(size / 2.))
                    .bg(rgb(0xdddddd))
                    .child(
                        div()
                            .absolute()
                            .top(px(border))
                            .left(px(border))
                            .w(px(size - border * 2.))
                            .h(px(size - border * 2.))
                            .rounded(px(size / 2. - border))
                            .bg(rgb(0x888888)),
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
        self.interactivity.occlude_mouse();
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
                    let Size { width, height } = bounds.size;
                    view.scale.gen_image(
                        f32::from(width),
                        f32::from(height),
                        view.padding,
                        view.color,
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
            cx.paint_image(bounds, Corners::all(px(0.)), data.clone(), false)
                .log_err();
        }
    }
}
