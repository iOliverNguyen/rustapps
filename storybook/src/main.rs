mod assets;
mod states;
mod views;

use assets::*;
use gpui::*;
use states::*;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use views::*;

actions!(app, [Quit]);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_log();

    let app = App::new();
    let text_sys = app.text_system();

    app.with_assets(assets::Assets).run(|cx| {
        let opts = WindowOptions {
            bounds: Some(Bounds::centered(None, size(px(1200.), px(800.)), cx)),
            titlebar: Some(TitlebarOptions {
                title: Some("UIColors".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(10.), px(10.))),
                ..TitlebarOptions::default()
            }),
            ..WindowOptions::default()
        };

        cx.open_window(opts, |cx| {
            let workspace = cx.new_view(|cx| Workspace::new(cx));
            cx.focus_view(&workspace);
            workspace
        });

        cx.activate(true);
        cx.on_action(|act: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.set_menus(vec![Menu {
            name: "",
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
    });

    Ok(())
}

fn init_log() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
