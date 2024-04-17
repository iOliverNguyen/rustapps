use super::*;
use crate::{AppState, ColorFormat, ColorPalette, MainTab, KEY_SPACE};
use gpui::*;
use std::sync::Arc;

pub struct Workspace {
    app_state: Arc<AppState>,
    title_bar: View<TitleBar>,
    left_panel: View<LeftPanel>,
    central: View<Central>,
    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let color = ColorFormat::random_hsl();

        let app_state = Arc::new(AppState {
            color: cx.new_model(|cx| color),
            palette: cx.new_model(|cx| ColorPalette::from(color.canonicalize())),
            main_tab: cx.new_model(|cx| MainTab::Home),
            library: cx.new_model(|cx| None),
        });
        let weak_handle = cx.view().downgrade();
        let workspace = Self {
            app_state: app_state.clone(),
            title_bar: cx.new_view(|cx| TitleBar::new(cx)),
            left_panel: cx
                .new_view(|cx| LeftPanel::new(cx, weak_handle.clone(), app_state.clone())),
            central: cx.new_view(|cx| Central::new(cx, weak_handle.clone(), app_state.clone())),
            focus_handle: cx.focus_handle(),
        };

        workspace
    }

    fn handle_keydown(&mut self, ev: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        if let KeyDownEvent {
            keystroke,
            is_held: false,
        } = ev
        {
            if keystroke.modifiers == Modifiers::default() {
                if keystroke.key == KEY_SPACE.key {
                    self.new_random_color(cx);
                    cx.refresh();
                }
            }
        }
    }

    fn set_color(&mut self, cx: &mut ViewContext<Self>, new_color: Hsla) {
        let new_palette = ColorPalette::from(new_color);
        self.app_state.color.update(cx, |color, cx| {
            *color = ColorFormat::from(new_color);
            cx.notify();
        });
        self.app_state.palette.update(cx, |palette, cx| {
            *palette = new_palette;
            cx.notify();
        });
    }

    fn new_random_color(&mut self, cx: &mut ViewContext<Self>) {
        let new_color = ColorFormat::random_hsl();
        self.set_color(cx, new_color.canonicalize());
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
            .key_context("Workspace")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_keydown))
            .size_full()
            .bg(rgba(0x000000))
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
    }
}
