use super::*;
use crate::{ColorFormat, ColorLibrary, ColorPalette};
use gpui::*;

pub struct Workspace {
    library: Model<ColorLibrary>,
    active_color: Model<ColorFormat>,
    active_palette: Model<ColorPalette>,

    title_bar: View<TitleBar>,
    status_bar: View<StatusBar>,
    left_panel: View<LeftPanel>,
    central: View<Central>,
    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let color = ColorFormat::from("#84cc16");

        Self {
            library: cx.new_model(|x| ColorLibrary::default()),
            active_color: cx.new_model(|cx| color),
            active_palette: cx.new_model(|cx| ColorPalette::from(color.normalize())),

            focus_handle: cx.focus_handle(),
            title_bar: cx.new_view(|cx| TitleBar::new(cx)),
            status_bar: cx.new_view(|cx| StatusBar::new(cx)),
            left_panel: cx.new_view(|cx| LeftPanel::new(cx)),
            central: cx.new_view(|cx| Central::new(cx)),
        }
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("app")
            .size_full()
            .bg(rgb(0x888888))
            .flex()
            .flex_col()
            .child(self.title_bar.clone())
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_row()
                    .child(self.left_panel.clone())
                    .child(self.central.clone()),
            )
            .child(self.status_bar.clone())
    }
}
