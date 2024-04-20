use blocks::{button, Button};
use gpui::*;
use gpui_ext::*;

pub struct ButtonStory {
    btn_basic: View<Button>,
    btn_filled: View<Button>,
    // btn_outline: View<Button>,
    // btn_icon_basic: View<Button>,
    // btn_icon_filled: View<Button>,
    // btn_icon_outline: View<Button>,
    focus_handle: FocusHandle,
}

impl ButtonStory {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            btn_basic: button().build(cx),
            btn_filled: button().build(cx),
            // btn_outline: button().build(cx),
            // btn_icon_basic: button().build(cx),
            // btn_icon_filled: button().build(cx),
            // btn_icon_outline: button().build(cx),
            focus_handle: cx.focus_handle(),
        }
    }

    fn row(&self, cx: &mut ViewContext<Self>, text: SharedString) -> Div {
        div().flex_center().flex_row().gap(px(10.)).child(text)
    }

    fn cell(
        &self,
        cx: &mut ViewContext<Self>,
        button: View<Button>,
        text: SharedString,
    ) -> impl IntoElement {
        div()
            .w_full()
            .flex_center()
            .flex_col()
            .child(div().w_full().flex_center().child(button))
            .child(div().w_full().flex_center().child(text))
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
            self.row(cx, "Basic".into()).children(vec![
                self.cell(cx, self.btn_basic.clone(), "Basic".into()),
                self.cell(cx, self.btn_basic.clone(), "Primary".into()),
                self.cell(cx, self.btn_basic.clone(), "Accent".into()),
                self.cell(cx, self.btn_basic.clone(), "Error".into()),
                self.cell(cx, self.btn_basic.clone(), "Disabled".into()),
            ]),
            self.row(cx, "Filled".into()).children(vec![
                self.cell(cx, self.btn_filled.clone(), "Basic".into()),
                self.cell(cx, self.btn_filled.clone(), "Primary".into()),
                self.cell(cx, self.btn_filled.clone(), "Accent".into()),
                self.cell(cx, self.btn_filled.clone(), "Error".into()),
                self.cell(cx, self.btn_filled.clone(), "Disabled".into()),
            ]),
        ])
    }
}
