mod cli;
mod views;

use clap::Parser;
use gpui::*;
use views::*;

actions!(app, [Quit, About]);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let app = App::new();
    app.run(|cx| {
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

        cx.set_menus(vec![
            Menu {
                name: "",
                items: vec![MenuItem::action("Quit", Quit)],
            },
            Menu {
                name: "Help",
                items: vec![MenuItem::action("About", About)],
            },
        ]);
    });

    Ok(())
}
