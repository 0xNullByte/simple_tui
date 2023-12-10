use super::{AsWidget, Rect, Widget};
use crate::{functions::Alignment, Console};

pub struct Lable {
    /// Widget id
    id: Option<usize>,

    /// Text to display
    pub text: String,

    /// Text Alignment
    alignment: Alignment,

    /// Wrap around.
    wrap: bool,

    /// Console controler
    pub console: Console,

    /// Widget shape.
    pub shape: Rect,
}

impl Lable {
    /// Construct new lable
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: None,
            text: text.into(),
            alignment: Alignment::Left,
            wrap: false,
            console: Console::new(),
            shape: Rect::default(),
        }
    }

    pub fn id(&self) -> Option<usize> {
        self.id
    }
    pub fn set_id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    /// Check if Widget's wrap around.
    pub fn is_wrap(&self) -> bool {
        self.wrap
    }

    /// Wrap around
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
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
            self.render_text(&rect);
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

impl AsWidget for Lable {
    fn as_widget(self) -> Option<Widget> {
        Some(Widget::Lable(self))
    }
}
