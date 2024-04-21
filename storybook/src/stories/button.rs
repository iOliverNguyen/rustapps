use blocks::Button;
use blocks_md::*;
use gpui::*;
use gpui_ext::*;

const N: usize = 5;

pub struct ButtonStory {
    // btn_outline: View<Button>,
    // btn_icon_basic: View<Button>,
    // btn_icon_filled: View<Button>,
    // btn_icon_outline: View<Button>,
    focus_handle: FocusHandle,
}

impl ButtonStory {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn render_row(&self, cx: &mut ViewContext<Self>, text: SharedString) -> Div {
        div()
            .w_full()
            .h(px(80.))
            .flex_center()
            .flex_row()
            .gap(px(20.))
            .child(text)
    }

    fn render_cell(
        &self,
        cx: &mut ViewContext<Self>,
        button: Button<MdTheme>,
        text: SharedString,
    ) -> impl IntoElement {
        div()
            .w(px(100.))
            .flex_center()
            .flex_col()
            .child(div().w_full().flex_center().child(button))
    }
}

impl FocusableView for ButtonStory {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ButtonStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().flex_center().flex_col().children(vec![
            self.render_row(cx, "Basic".into()).children(vec![
                self.render_cell(
                    cx,
                    Button::new_text(SharedString::from("HELLO")), // .theme(ThemeVariant::Primary)
                    "Basic".into(),
                ),
                // self.cell(cx, self.btn_basic[1].clone(), "Primary".into()),
                // self.cell(cx, self.btn_basic[2].clone(), "Accent".into()),
                // self.cell(cx, self.btn_basic[3].clone(), "Error".into()),
                // self.cell(cx, self.btn_basic[4].clone(), "Disabled".into()),
            ]),
            self.render_row(cx, "Filled".into()).children(vec![
                div(),
                // self.cell(cx, self.btn_filled[0].clone(), "Basic".into()),
                // self.cell(cx, self.btn_filled[1].clone(), "Primary".into()),
                // self.cell(cx, self.btn_filled[2].clone(), "Accent".into()),
                // self.cell(cx, self.btn_filled[3].clone(), "Error".into()),
                // self.cell(cx, self.btn_filled[4].clone(), "Disabled".into()),
            ]),
        ])
    }
}
