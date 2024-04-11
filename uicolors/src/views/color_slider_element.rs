use gpui::*;

pub struct ColorSliderElement {
    focus_handle: FocusHandle,
    focused: bool,
    interactivity: Interactivity,
}

impl StatefulInteractiveElement for ColorSliderElement {}

impl InteractiveElement for ColorSliderElement {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl IntoElement for ColorSliderElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl ColorSliderElement {
    pub fn new(focus_handle: FocusHandle, focused: bool) -> Self {
        Self {
            focus_handle,
            focused,
            interactivity: Default::default(),
        }
    }
}

impl Element for ColorSliderElement {
    type BeforeLayout = ();
    type AfterLayout = ();

    /// Before an element can be painted, we need to know where it's going to be and how big it is.
    /// Use this method to request a layout from Taffy and initialize the element's state.
    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        self.interactivity.occlude_mouse();
        let layout_id = self.interactivity.before_layout(cx, |mut style, cx| {
            style.size.width = relative(1.).into();
            style.size.height = relative(1.).into();
            let layout_id = cx.request_layout(&style, None);

            layout_id
        });
        (layout_id, ())
    }

    /// After laying out an element, we need to commit its bounds to the current frame for hitbox
    /// purposes. The state argument is the same state that was returned from [`Element::before_layout()`].
    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Self::AfterLayout {
        ()
    }

    /// Once layout has been completed, this method will be called to paint the element to the screen.
    /// The state argument is the same state that was returned from [`Element::before_layout()`].
    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        after_layout: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        cx.paint_quad(fill(bounds, rgb(0xff0000)));
    }
}
