use crate::{inner_vbox_render, inner_vbox_trigger, AsWidget, Console, EventHandler};
use crossterm::{
    event::{
        read, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    terminal::{disable_raw_mode, enable_raw_mode},
};

use super::{Rect, Widget};

pub struct Hbox {
    /// Widget id
    id: Option<usize>,

    /// Widgets
    pub widgets: Vec<Option<Widget>>,

    /// Console controler
    console: Console,

    /// Wrap around.
    wrap: bool,

    /// Widget shape.
    pub shape: Rect,
}

impl Hbox {
    pub fn new(widgets: Vec<Option<Widget>>) -> Self {
        Self {
            id: None,
            widgets,
            console: Console::new(),
            shape: Rect::default(),
            wrap: false,
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
    pub fn wrap() {}

    /// Render the widget on console.
    pub fn render(&mut self, rect: Rect) {
        let n = self.widgets.len();
        let ww = rect.w / n as f32;
        for idx in 0..n {
            let mut padding = 1.0 as f32;
            if let Widget::Vbox(_) | Widget::Hbox(_) = self.widgets[idx].as_ref().unwrap() {
                padding = 0.0;
            }
            self.widgets[idx].as_mut().unwrap().render(Rect::new(
                (rect.x + ww * idx as f32).floor(),
                rect.y + padding,
                ww,
                rect.h,
            ));
        }
    }
}

impl AsWidget for Hbox {
    fn as_widget(self) -> Option<Widget> {
        Some(Widget::Hbox(self))
    }
}

impl EventHandler for Hbox {
    fn start(&mut self) {
        enable_raw_mode().unwrap();
        self.console.clear();
        loop {
            let (mx, my) = self.console.max_xy();
            let rect = Rect::new(0.0, 0.0, mx, my);
            self.render(rect);
            self.handle_event();
        }
    }

    fn handle_event(&mut self) {
        let event = read().unwrap();
        match event {
            Event::Resize(_, _) => self.console.clear(),
            Event::Mouse(MouseEvent {
                kind, column, row, ..
            }) => {
                if kind == MouseEventKind::Down(MouseButton::Left) {
                    // loop over the widgets
                    // check if the Cursor after the click in Widget area
                    // then trigger the callback
                    for wi in 0..self.widgets.len() {
                        let shape = self.widgets[wi].as_ref().unwrap().shape();
                        if shape.x as u16 <= column
                            && shape.w as u16 >= column
                            && shape.y as u16 == row
                        {
                            if let Widget::Button(_) = self.widgets[wi].as_ref().unwrap() {
                                let clicked_widget = self.widgets[wi].take();
                                self.widgets[wi] = self.trigger_callback(clicked_widget);

                                self.console.clear();
                            }
                        }

                        // Same process but for inner Widgets.
                        if let Widget::Hbox(hb) = self.widgets[wi].as_mut().unwrap() {
                            inner_hbox_render(hb, column, row);
                        }

                        // Same process but for inner Widgets.
                        if let Widget::Vbox(vb) = self.widgets[wi].as_mut().unwrap() {
                            inner_vbox_render(vb, column, row);
                        }
                    }
                }
            }
            _ => {}
        }
        if event == Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)) {
            disable_raw_mode().unwrap();
            self.console.clear();
            std::process::exit(0);
        };
    }

    fn trigger_callback(&mut self, clicked_widget: Option<Widget>) -> Option<Widget> {
        if let Widget::Button(btn) = clicked_widget.as_ref().unwrap() {
            match btn.callback {
                Some(ref f) => {
                    let rid = btn
                        .rid
                        .expect("Please make sure you set the rid through .set_rid(rid: usize)");

                    for wi in 0..self.widgets.len() {
                        match self.widgets[wi].as_mut().unwrap() {
                            // Inner Widgets
                            Widget::Vbox(vb) => {
                                if inner_vbox_trigger(vb, f, rid) {
                                    return clicked_widget;
                                }
                            }
                            // Inner Widgets
                            Widget::Hbox(hb) => {
                                if inner_hbox_trigger(hb, f, rid) {
                                    return clicked_widget;
                                }
                            }
                            // Outer Widgets
                            w @ _ => {
                                if let Some(id) = w.id() {
                                    if rid == id {
                                        self.widgets[wi] = Some(
                                            f(self.widgets[wi].take().as_mut().unwrap()).take(),
                                        );
                                        return clicked_widget;
                                    }
                                }
                            }
                        }
                    }
                }

                _ => {}
            }
        }
        clicked_widget
    }
}

pub fn inner_hbox_render(hbox: &mut Hbox, column: u16, row: u16) {
    for wi in 0..hbox.widgets.len() {
        let shape = hbox.widgets[wi].as_ref().unwrap().shape();
        if shape.x as u16 <= column
            && (shape.w + shape.x - 1.0) as u16 >= column
            && (shape.y - 0.0) as u16 == row
        {
            if let Widget::Button(_) = hbox.widgets[wi].as_ref().unwrap() {
                let clicked_widget = hbox.widgets[wi].take();
                hbox.widgets[wi] = hbox.trigger_callback(clicked_widget);
                crate::Console::new().flush();
            }
        }

        if let Widget::Hbox(hb) = hbox.widgets[wi].as_mut().unwrap() {
            inner_hbox_render(hb, column, row);
        }

        if let Widget::Vbox(vb) = hbox.widgets[wi].as_mut().unwrap() {
            inner_vbox_render(vb, column, row);
        }
    }
}

pub fn inner_hbox_trigger(
    hbox: &mut Hbox,
    f: impl Fn(&mut Widget) -> &mut Widget,
    rid: usize,
) -> bool {
    for wi in 0..hbox.widgets.len() {
        if let Some(w) = hbox.widgets[wi].as_ref() {
            if let Some(id) = w.id() {
                if rid == id {
                    hbox.widgets[wi] = Some(f(hbox.widgets[wi].take().as_mut().unwrap()).take());
                    return true;
                }
            }
        }
    }
    false
}
