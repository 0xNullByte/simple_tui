use crate::widgets::*;

pub enum Widget {
    Vbox(Vbox),
    Hbox(Hbox),
    Button(Button),
    Lable(Lable),
    None,
}

impl Widget {
    /// * Render the Widget on the terminal.
    pub fn render(&mut self, rect: Rect) {
        match self {
            Widget::Button(b) => b.render(rect),
            Widget::Lable(l) => l.render(rect),
            Widget::Vbox(vb) => vb.render(rect),
            Widget::Hbox(hb) => hb.render(rect),
            _ => {
                panic!("Can not render Widget. Because Widget doesn't have `.render(&mut self, rect: Rect)` method")
            }
        }
    }

    /// * Shape of the Widget.
    pub fn shape(&self) -> &Rect {
        match self {
            Widget::Button(b) => &b.shape,
            Widget::Lable(l) => &l.shape,
            Widget::Vbox(vb) => &vb.shape,
            Widget::Hbox(hb) => &hb.shape,
            _ => {
                panic!(
                    "Can not find shape for this Widget. Because Widget doesn't have shape field."
                )
            }
        }
    }

    pub fn id(&self) -> Option<usize> {
        match self {
            Widget::Vbox(vb) => vb.id(),
            Widget::Hbox(rb) => rb.id(),
            Widget::Button(b) => b.id(),
            Widget::Lable(l) => l.id(),
            _ => {
                panic!("Can not find Widget id.")
            }
        }
    }

    /// * replace `Widget::Variant(T)` => `Widget::None`
    /// * return previous `Widget::Variant(T)`
    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Self::None)
    }
}
