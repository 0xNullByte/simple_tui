use simple_tui::functions::*;

fn add_one(w: &mut Widget) -> &mut Widget {
    if let Widget::Lable(l) = w {
        let n = &l.text["Counter: ".len()..].parse::<usize>().unwrap();
        l.text = "Counter: ".to_string() + &(n + 1).to_string();
    }
    w
}

fn main() {
    vbox(widgets![
        lable("Counter: 0")
            .set_id(1337)
            .align(Alignment::Center)
            .wrap(),
            
        button("Click me!")
            .set_rid(1337)
            .on_click(|w| add_one(w))
    ])
    .start();
}
