use gpui::*;

impl<T: Styled> StyledExtension for T {}
impl<T: IntoElement> ElementExtension for T {}

pub trait StyledExtension: Styled {
    fn flex_center(self) -> Self {
        self.flex().justify_center().items_center()
    }

    fn border_size<T>(self, t: T, r: T, b: T, l: T) -> Self
    where
        T: Clone + Into<AbsoluteLength>,
    {
        self.border_t_width(t)
            .border_r_width(r)
            .border_b_width(b)
            .border_l_width(l)
    }

    fn border_x_size<T>(self, l: T, r: T) -> Self
    where
        T: Clone + Into<AbsoluteLength>,
    {
        self.border_l_width(l).border_r_width(r)
    }

    fn border_y_size<T>(self, t: T, b: T) -> Self
    where
        T: Clone + Into<AbsoluteLength>,
    {
        self.border_t_width(t).border_b_width(b)
    }
}

trait ElementExtension: IntoElement {
    fn apply(self, f: impl FnOnce(Self) -> Self) -> Self {
        f(self)
    }
}
