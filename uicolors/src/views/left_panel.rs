use crate::Assets;
use gpui::*;
use gpui_ext::*;

pub struct LeftPanel {
    btn_home: View<TopLevelButton>,
    btn_browse: View<TopLevelButton>,
    btn_favorites: View<TopLevelButton>,
    focus_handle: FocusHandle,
}

impl LeftPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            btn_home: cx.new_view(|cx| TopLevelButton {
                label: "Home".into(),
                icon: Assets::icon("home").into(),
                focus_handle: cx.focus_handle(),
            }),
            btn_browse: cx.new_view(|cx| TopLevelButton {
                label: "Browse".into(),
                icon: Assets::icon("book").into(),
                focus_handle: cx.focus_handle(),
            }),
            btn_favorites: cx.new_view(|cx| TopLevelButton {
                label: "Favorites".into(),
                icon: Assets::icon("book-star").into(),
                focus_handle: cx.focus_handle(),
            }),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for LeftPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LeftPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(rems(10.))
            .h_full()
            .bg(rgb(0x444488))
            .child(
                div()
                    .border_size(rems(1.5), rems(0.3), rems(1.), rems(0.3))
                    .flex_col()
                    .gap(rems(0.1))
                    .bg(rgb(0x8888FF))
                    .child(self.btn_home.clone())
                    .child(self.btn_browse.clone())
                    .child(self.btn_favorites.clone()),
            )
            .child(div().w_full().h(px(1.)).bg(rgb(0x888888)))
            .child(
                div()
                    .text_size(rems(0.8))
                    .border_size(rems(0.3), rems(1.), rems(0.3), rems(1.))
                    .flex()
                    .flex_row()
                    .child(div().child("Library").w_full())
                    .child(div().child("★")),
            )
            .child(div().size_full().bg(rgb(0x888888)))
    }
}

struct TopLevelButton {
    label: SharedString,
    icon: SharedString,
    focus_handle: FocusHandle,
}

impl FocusableView for TopLevelButton {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TopLevelButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .bg(rgb(0x8888FF))
            .border_y_size(rems(0.1), rems(0.1))
            .border_x_size(rems(0.6), rems(0.6))
            .rounded_md()
            .flex()
            .flex_row()
            .hover(|st| st.bg(rgb(0x000000)))
            .text_color(rgb(0xffff88))
            .child(
                div()
                    .size(rems(1.5))
                    .flex_center()
                    .text_color(rgb(0x88ffff))
                    .child(
                        svg()
                            .size(rems(1.1))
                            .text_color(rgb(0xffffff))
                            .path(self.icon.clone()),
                    ),
            )
            .child(self.label.clone())
    }
}
