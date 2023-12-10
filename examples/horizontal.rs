use simple_tui::functions::*;

fn main() {
    hbox(widgets![
        lable("Hello 1").align(Alignment::Left),
        lable("Hello 2").align(Alignment::Center),
        lable("Hello 3").align(Alignment::Right),
        lable("Hello 4").align(Alignment::Left).wrap(),
        lable("Hello 5").align(Alignment::Center).wrap(),
        lable("Hello 6").align(Alignment::Right).wrap()
    ])
    .start();
}
