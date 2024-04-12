use crate::*;
use gpui::*;
use image::{Bgra, ImageBuffer};
use std::sync::Arc;

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

    fn gen_image_data(&self, bounds: Bounds<Pixels>) -> ImageData {
        let Size { width, height } = bounds.size;
        ColorScale::Hue.gen_image(f32::from(width), f32::from(height))
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
        let image_data = self.slider_view.update(cx, |view, cx| {
            let image_data = match view.bounds {
                None => Some(Arc::new(self.gen_image_data(bounds))),
                Some(b) if b.size != bounds.size => Some(Arc::new(self.gen_image_data(bounds))),
                _ => view.image_data.clone(),
            };
            view.bounds = Some(bounds);
            view.image_data = image_data.clone();
            image_data
        });
        image_data
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

enum ColorScale {
    Hue,
    Saturation(Hsla),
    Lightness(Hsla),
}

impl ColorScale {
    fn gen_image(self, w: f32, h: f32) -> ImageData {
        fn cv(f: f32) -> u8 {
            (f * 256.).clamp(0., 255.) as u8
        }

        let (w, h) = (w.trunc(), h.trunc());
        let buffer: ImageBuffer<Bgra<u8>, Vec<u8>> = match self {
            ColorScale::Hue => ImageBuffer::from_fn(w as u32, h as u32, |x, y| {
                let color = hsla(x as f32 / w, 1., 0.5, 1.);
                let Rgba { r, g, b, a } = color.to_rgb();
                Bgra([cv(b), cv(g), cv(r), 255])
            }),
            ColorScale::Lightness(color) => todo!(),
            ColorScale::Saturation(color) => todo!(),
        };
        ImageData::new(buffer)
    }
}
