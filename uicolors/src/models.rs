use gpui::{Hsla, Rgba};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorFormat {
    Rgb(u8, u8, u8),  // 0..255, 0..255, 0..255
    Hsl(u16, u8, u8), // 0..360, 0..100, 0..100
}

impl ColorFormat {
    pub fn to_string(&self) -> String {
        match self {
            ColorFormat::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
            ColorFormat::Hsl(h, s, l) => format!("{},{},{}", h, s, l),
        }
    }

    pub fn parse(s: &str) -> Option<ColorFormat> {
        if s.starts_with("#") {
            // parse from hex
            let hex = s.trim_start_matches("#");
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(ColorFormat::Rgb(r, g, b).canonicalize());
        } else {
            // parse from hsl
            let parts: Vec<&str> = s.split(",").collect();
            let h: u16 = parts[0].parse().ok()?;
            let s: u8 = parts[1].parse().ok()?;
            let l: u8 = parts[2].parse().ok()?;
            return Some(ColorFormat::Hsl(h, s, l).canonicalize());
        }
    }

    pub fn canonicalize(self) -> ColorFormat {
        match self {
            ColorFormat::Rgb(r, g, b) => ColorFormat::Rgb(r.min(255), g.min(255), b.min(255)),
            ColorFormat::Hsl(h, s, l) => {
                let h = if h < 0 || h > 360 { h % 360 } else { h };
                ColorFormat::Hsl(h, s.min(100), l.min(100))
            }
        }
    }

    pub fn normalize(&self) -> Hsla {
        match self.canonicalize() {
            ColorFormat::Rgb(r, g, b) => {
                let r = r as f32 / 255.;
                let g = g as f32 / 255.;
                let b = b as f32 / 255.;
                Rgba { r, g, b, a: 1. }.into()
            }
            ColorFormat::Hsl(h, s, l) => {
                let h = h as f32 / 360.;
                let s = s as f32 / 100.;
                let l = l as f32 / 100.;
                Hsla { h, s, l, a: 1. }
            }
        }
    }

    pub fn to_rgb(self) -> ColorFormat {
        match self.canonicalize() {
            ColorFormat::Rgb(_, _, _) => self,
            ColorFormat::Hsl(h, s, l) => {
                let h = (h as f32 / 360. * 255.) as u8;
                let s = (s as f32 / 100. * 255.) as u8;
                let l = (l as f32 / 100. * 255.) as u8;
                ColorFormat::Rgb(h, s, l)
            }
        }
    }

    pub fn to_hsl(self) -> ColorFormat {
        match self.canonicalize() {
            ColorFormat::Hsl(_, _, _) => self,
            ColorFormat::Rgb(_, _, _) => {
                let Hsla { h, l, s, a: _ } = self.normalize();
                ColorFormat::Hsl((h * 360.) as u16, (s * 100.) as u8, (l * 100.) as u8)
            }
        }
    }
}

impl From<&str> for ColorFormat {
    fn from(s: &str) -> Self {
        ColorFormat::parse(s).unwrap()
    }
}

// implement Display for ColorItem
impl std::fmt::Display for ColorFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Serialize for ColorFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ColorFormat {
    fn deserialize<D>(deserializer: D) -> Result<ColorFormat, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ColorFormat::parse(&s).ok_or_else(|| serde::de::Error::custom("invalid color format"))
    }
}

// convert 0 50 100 .. 900 950 1000 to 0..1
pub fn lightness(l: usize) -> f32 {
    l.min(1000) as f32 / 1000.
}

#[derive(Clone, Copy)]
pub struct ColorPalette {
    pub h: f32,
    pub s: f32,
    pub hue_shift: f32,
}

impl ColorPalette {
    pub fn new(h: f32, s: f32) -> Self {
        Self {
            h,
            s,
            hue_shift: 0.,
        }
    }

    pub fn from(c: Hsla) -> Self {
        Self {
            h: c.h,
            s: c.s,
            hue_shift: 0.,
        }
    }

    pub fn hue_shift(mut self, hue_shift: f32) -> Self {
        self.hue_shift = hue_shift;
        self
    }

    pub fn at(self, l: f32) -> Hsla {
        let Self { h, s, hue_shift } = self;
        let l = l.clamp(0., 1.);
        let h = (h + (l - 0.5) * hue_shift).fract();
        let h = if h < 0. { h + 1. } else { h };
        Hsla { h, s, l, a: 1. }
    }

    pub fn colors(self) -> Vec<Hsla> {
        vec![
            self.at(lightness(50)),
            self.at(lightness(100)),
            self.at(lightness(200)),
            self.at(lightness(300)),
            self.at(lightness(400)),
            self.at(lightness(500)),
            self.at(lightness(600)),
            self.at(lightness(700)),
            self.at(lightness(800)),
            self.at(lightness(900)),
            self.at(lightness(950)),
        ]
    }
}

#[derive(Serialize, Deserialize)]
pub struct ColorItem {
    pub color: ColorFormat,
    pub name: Option<String>,
    pub favorite: bool,
}

impl ColorItem {
    pub fn new(color: ColorFormat, name: Option<String>) -> Self {
        Self {
            color,
            name: name.into(),
            favorite: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ColorLibrary {
    pub items: Vec<ColorItem>,
}

impl Default for ColorLibrary {
    fn default() -> Self {
        fn item(hex: &str, name: &str) -> ColorItem {
            ColorItem::new(hex.into(), Some(name.to_owned()))
        }

        Self {
            items: vec![
                item("#f43f5e", "Rose"),
                item("#ec4899", "Pink"),
                item("#d946ef", "Fuchsia"),
                item("#a855f7", "Purple"),
                item("#8b5cf6", "Violet"),
                item("#6366f1", "Indigo"),
                item("#3b82f6", "Blue"),
                item("#0ea5e9", "Sky"),
                item("#06b6d4", "Cyan"),
                item("#14b8a6", "Teal"),
                item("#10b981", "Emerald"),
                item("#22c55e", "Green"),
                item("#84cc16", "Lime"),
                item("#eab308", "Yellow"),
                item("#f59e0b", "Amber"),
                item("#f97316", "Orange"),
                item("#ef4444", "Red"),
                item("#78716c", "Stone"),
                item("#737373", "Neutral"),
            ],
        }
    }
}

impl ColorLibrary {
    pub fn position(&self, color: ColorFormat) -> Option<usize> {
        let (rgb, hsl) = (color.to_rgb(), color.to_hsl());
        self.items
            .iter()
            .position(|x| x.color == rgb || x.color == hsl)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            ColorFormat::parse("#ee00ff"),
            Some(()).map(|_| ColorFormat::Rgb(238, 0, 255))
        );
    }
    #[test]
    fn canonicalize() {
        let c0 = ColorFormat::Hsl(400, 120, 120);
        let c1 = c0.canonicalize();
        assert_eq!(c0, ColorFormat::Hsl(400, 120, 120));
        assert_eq!(c1, ColorFormat::Hsl(40, 100, 100));
    }
}
