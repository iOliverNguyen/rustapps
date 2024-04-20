use gpui::Rgba;
use gpui::{rgba, Hsla};
use material_colors::dynamic_color::dynamic_scheme::DynamicScheme;
use material_colors::dynamic_color::material_dynamic_colors::{self, MaterialDynamicColors};
use material_colors::utils::theme;
use material_colors::{Hct, SchemeTonalSpot};

fn main() {
    let source = Hct::from(25.0, 84.0, 50.0);
    let sc = SchemeTonalSpot::new(source, true, None).scheme;
    let argb = MaterialDynamicColors::primary().get_argb(&sc);
    let hct = MaterialDynamicColors::primary().get_hct(&sc);

    sc.primary_palette;
    let rgb0 = Rgba {
        r: (argb.red as f32) / 255.,
        g: (argb.green as f32) / 255.,
        b: (argb.blue as f32) / 255.,
        a: (argb.alpha as f32) / 255.,
    };
    let hsla: Hsla = rgb0.into();
    let rgb1: Rgba = hsla.into();

    let p = sc.primary_palette;

    println!("{} {} {:?} {:?} {:?}", argb, hct, rgb0, hsla, rgb1);
    println!(
        "{} {} {} {} {}",
        p.tone(0),
        p.tone(30),
        p.tone(35),
        p.tone(100),
        p.tone(150),
    );
}
