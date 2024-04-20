use gpui::*;

enum Image {
    Svg(SharedString),
    Img(SharedString),
    ImgData(ImageData),
}

pub struct Icon {
    pub name: SharedString,
    pub source: Image,
    pub color: Option<Hsla>,
}
