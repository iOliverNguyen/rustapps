use super::*;
use crate::{AppState, ColorFormat, ResultExt};
use gpui::*;
use gpui_ext::*;
use indexmap::IndexMap;
use std::{sync::Arc, time::Duration};

pub struct Central {
    workspace: WeakView<Workspace>,
    app_state: Arc<AppState>,
    color_input: View<ColorInputView>,
    focus_handle: FocusHandle,
    btn_json_hover: bool,
    show_copied_msg: Model<Option<(usize, SharedString)>>, // (id, msg)
}

impl Central {
    const FADEOUT_DURATION: Duration = Duration::from_millis(1000);

    pub fn new(
        cx: &mut ViewContext<Self>,
        workspace: WeakView<Workspace>,
        app_state: Arc<AppState>,
    ) -> Self {
        let app_state0 = app_state.clone();
        let AppState { color, palette, .. } = app_state0.as_ref();
        Self {
            workspace,
            app_state,
            color_input: cx.new_view(|cx| ColorInputView::new(cx, color.clone(), palette.clone())),
            focus_handle: cx.focus_handle(),
            btn_json_hover: false,
            show_copied_msg: cx.new_model(|cx| None),
        }
    }

    fn set_copied_msg(&self, cx: &mut ViewContext<Self>, s: SharedString) {
        let prev_id = self.show_copied_msg.update(cx, |x, cx| {
            let id = match x {
                Some((id, _)) => *id + 1,
                None => 1,
            };
            *x = Some((id, s));
            id
        });
        cx.spawn(|this, mut cx| async move {
            // wait for a few secs then clear the message
            tokio::time::sleep(Self::FADEOUT_DURATION).await;

            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, cx| {
                    this.show_copied_msg.update(cx, |x, cx| {
                        if let Some((id, _)) = x {
                            if *id == prev_id {
                                *x = None;
                                cx.notify();
                            }
                        }
                    });
                })
                .log_err();
            }
        })
        .detach();
    }
}

impl FocusableView for Central {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

static SCALES: [usize; 11] = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

impl Render for Central {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let palette = self.app_state.palette.read(cx);

        div()
            .size_full()
            .bg(rgb(0x000000))
            .flex_center()
            .flex_col()
            .child(
                div()
                    .h(rems(10.))
                    .w_full()
                    .flex_center()
                    .text_color(palette.at_darkness(400))
                    .child("Select a color or press spacebar for random one."),
            )
            .child(div().flex_center().child(self.color_input.clone()))
            .child(
                div()
                    .size_full()
                    .flex_center()
                    .flex_col()
                    .child(div().h(px(100.)))
                    .child(
                        div()
                            .h(px(100.))
                            .w_full()
                            .flex_center()
                            .flex_row()
                            .gap(px(6.))
                            .children({
                                SCALES.iter().map(|sc| {
                                    let c = palette.at_darkness(*sc);
                                    let text_color = if c.l < 0.5 {
                                        hsla(0., 0.5, 1., 1.)
                                    } else {
                                        hsla(0., 0.5, 0., 1.)
                                    };
                                    div()
                                        .id(*sc)
                                        .w(px(72.))
                                        .h(px(100.))
                                        .rounded(px(8.))
                                        .bg(c)
                                        .flex()
                                        .flex_col()
                                        .justify_end()
                                        .items_center()
                                        .cursor_pointer()
                                        .on_click(cx.listener(move |this, _, cx| {
                                            let code = ColorFormat::from(c).to_rgb().to_string();
                                            this.set_copied_msg(
                                                cx,
                                                SharedString::from(format!(
                                                    "{} copied to clipboard!",
                                                    &code
                                                )),
                                            );
                                            cx.write_to_clipboard(ClipboardItem::new(code));
                                        }))
                                        .child(
                                            div()
                                                .flex_center()
                                                .font("Monaspace Xenon")
                                                .text_size(px(16.))
                                                .text_color(text_color)
                                                .child(format!("{}", sc)),
                                        )
                                        .child(
                                            div()
                                                .flex_center()
                                                .font("Monaspace Xenon")
                                                .text_size(px(12.))
                                                .text_color(text_color)
                                                .child({
                                                    ColorFormat::from(c)
                                                        .to_rgb()
                                                        .to_string()
                                                        .trim_start_matches("#")
                                                        .to_uppercase()
                                                }),
                                        )
                                        .child(div().h(px(10.)))
                                })
                            }),
                    )
                    .child({
                        let text_color = palette.at_darkness(300);
                        let div0 = div()
                            .h(px(100.))
                            .flex_center()
                            .text_size(px(14.))
                            .text_color(text_color);
                        match self.show_copied_msg.read(cx).clone() {
                            Some((id, msg)) => {
                                let msg_div = div().child(msg.clone());
                                let msg_div = msg_div.with_animation(
                                    id,
                                    Animation::new(Self::FADEOUT_DURATION),
                                    move |div, delta| {
                                        let base = 0.8;
                                        let fade_color = if delta < base {
                                            text_color
                                        } else {
                                            let x = (delta - base) / (1. - base);
                                            let mut text_color = text_color.clone();
                                            text_color.fade_out(x);
                                            text_color
                                        };
                                        div.text_color(fade_color)
                                    },
                                );
                                div0.child(msg_div)
                            }
                            _ => div0,
                        }
                    })
                    .child(
                        div()
                            .id("btn-json")
                            .focusable()
                            .cursor_pointer()
                            .on_hover(cx.listener(|this, is_hover, cx| {
                                if this.btn_json_hover != *is_hover {
                                    this.btn_json_hover = *is_hover;
                                    cx.refresh();
                                }
                            }))
                            .on_click(cx.listener(|this, ev, cx| {
                                let palette = this.app_state.palette.read(cx);
                                let map_colors: IndexMap<usize, String> =
                                    IndexMap::from_iter(SCALES.iter().map(|sc| {
                                        let color = palette.at_darkness(*sc);
                                        let code = ColorFormat::from(color).to_rgb().to_string();
                                        (*sc, format!("{}", code))
                                    }));
                                if let Ok(json) = serde_json::to_string_pretty(&map_colors) {
                                    cx.write_to_clipboard(ClipboardItem::new(json));
                                    this.set_copied_msg(
                                        cx,
                                        SharedString::from(
                                            "Color palette get copied to clipboard!",
                                        ),
                                    );
                                };
                            }))
                            .flex_center()
                            .w(px(200.))
                            .h(px(40.))
                            .rounded(px(4.))
                            .bg(if self.btn_json_hover {
                                palette.at_darkness(200)
                            } else {
                                palette.at_darkness(300)
                            })
                            .child("Copy as JSON"),
                    ),
            )
    }
}
