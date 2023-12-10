mod widgets;
pub use widgets::*;

mod backend;
pub use backend::*;

/// * Call .as_widget() over items
/// * Convert `Vec<T>` => `Vec<Option<Widget>>`
#[macro_export]
macro_rules! widgets {
    [$($item:expr),*] => {
        vec![$($item.as_widget()),*]
    };
}

pub mod functions {
    pub use super::*;

    /// * Construct new Vertiacl Box.
    pub fn vbox(widgets: Vec<Option<Widget>>) -> Vbox {
        Vbox::new(widgets)
    }

    /// * Construct new Horizontal Box.
    pub fn hbox(widgets: Vec<Option<Widget>>) -> Hbox {
        Hbox::new(widgets)
    }

    /// * Construct new Lable.
    pub fn lable(text: &'static str) -> Lable {
        Lable::new(text)
    }

    /// * Construct new Button.
    pub fn button(text: &'static str) -> Button {
        Button::new(text)
    }
}
