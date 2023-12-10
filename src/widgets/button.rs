use super::{AsWidget, Rect, Widget};
use crate::{Alignment, Console};

pub struct Button {
    /// Widget id
    id: Option<usize>,

    /// Text to display
    pub text: String,

    /// Text Alignment
    alignment: Alignment,

    /// Wrap around.
    wrap: bool,

    /// Related id.
    pub rid: Option<usize>,

    /// Callback function.
    pub callback: Option<Box<dyn Fn(&mut Widget) -> &mut Widget>>,

    /// Console controler
    pub console: Console,

    /// Widget shape.
    pub shape: Rect,
}

impl Button {
    /// * Construct new button.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: None,
            rid: None,
            callback: None,
            text: text.into(),
            wrap: true,
            alignment: Alignment::Center,
            console: Console::new(),
            shape: Rect::default(),
        }
    }

    /// * Storing the callback and taking action after getting clicked.
    ///
    /// Example:
    ///
    /// ```no_run
    ///     widgets![
    ///         lable("before")
    ///             .set_id(1337),
    ///         button("click me!")
    ///             .set_rid(1337)     // set related id to 1337 => `lable("before")`
    ///             .on_click(|w: &mut Widget>| {
    ///                 if let Widget::Lable(l) = w { l.text = "after".into() };
    ///                 w
    ///              }),
    ///         ]
    /// ```
    pub fn on_click(mut self, f: impl Fn(&mut Widget) -> &mut Widget + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }

    /// Widget id
    pub fn id(&self) -> Option<usize> {
        self.id
    }

    /// Set Widget id
    pub fn set_id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    /// * Set related id for any Widget inside the same widgets array. \
    pub fn set_rid(mut self, rid: usize) -> Self {
        self.rid = Some(rid);
        self
    }

    /// Check if Widget's wrap around.
    pub fn is_wrap(&self) -> bool {
        self.wrap
    }

    /// Set alignment for the text.
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Render text with consideration the alignment
    fn render_text(&mut self, rect: &Rect) {
        match self.alignment {
            Alignment::Left => self.console.draw((rect.x + 1.0, rect.y), &self.text),
            Alignment::Center => self.console.draw(
                (
                    rect.x + 1.0 + ((rect.w * 0.5) - (self.text.len() as f32 * 0.5)),
                    rect.y,
                ),
                &self.text,
            ),
            Alignment::Right => self.console.draw(
                (rect.x - 1.0 + (rect.w - self.text.len() as f32), rect.y),
                &self.text,
            ),
        }
    }

    /// Render the widget on console.
    pub fn render(&mut self, rect: Rect) {
        if !self.wrap {
            self.console.draw((rect.x, rect.y), &self.text);
            return;
        }

        // wrap around
        let top = "┌".to_string() + &"─".repeat(rect.w as usize - 2) + "┐";
        let donw = "└".to_string() + &"─".repeat(rect.w as usize - 2) + "┘";
        self.console.draw((rect.x, rect.y - 1.0), &top);
        self.console.draw((rect.x, rect.y), "│");
        self.render_text(&rect);
        self.console.draw((rect.x + rect.w - 1.0, rect.y), "│");
        self.console.draw((rect.x, rect.y + 1.0), &donw);

        // required.
        self.shape = rect;
    }
}

impl AsWidget for Button {
    fn as_widget(self) -> Option<Widget> {
        Some(Widget::Button(self))
    }
}
