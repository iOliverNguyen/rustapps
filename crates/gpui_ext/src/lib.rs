use gpui::*;

impl<T: Styled> StyledExtension for T {}
impl<T: IntoElement> ElementExtension for T {}

pub trait StyledExtension: Styled {
    fn flex_center(self) -> Self {
        self.flex().justify_center().items_center()
    }
}

trait ElementExtension: IntoElement {
    fn apply(self, f: impl FnOnce(Self) -> Self) -> Self {
        f(self)
    }
}
