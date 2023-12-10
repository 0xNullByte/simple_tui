mod utility;
pub use utility::*;

mod widget;
pub use widget::*;

mod vbox;
pub use vbox::*;

mod hbox;
pub use hbox::*;

mod button;
pub use button::*;

mod lable;
pub use lable::*;

pub trait AsWidget {
    /// * Convert `T` =>  `Option<Widget>`
    fn as_widget(self) -> Option<Widget>;
}

/// * Start drawing Widgets on console.
/// * Handle Mouse, Resize and Keystroke.
/// * Trigger callbacks.
pub trait EventHandler {
    /// #####  Start rendering the Widgets on screen. \[ *Infinity loop* \]
    fn start(&mut self);

    ///* Handle inputs.
    fn handle_event(&mut self);

    ///* Trigger the callback `on_click`.
    fn trigger_callback(&mut self, clicked_widget: Option<Widget>) -> Option<Widget>;
}
