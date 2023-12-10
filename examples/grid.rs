use simple_tui::functions::*;

fn main() {
    vbox(widgets![
        hbox(widgets![
            lable("Hello 1").align(Alignment::Right).wrap(),
            lable("Hello 2").align(Alignment::Right).wrap(),
            lable("Hello 3").align(Alignment::Left).wrap()
        ]),
        hbox(widgets![
            lable("Hello 4").align(Alignment::Left).wrap(),
            lable("Hello 5").align(Alignment::Center).wrap(),
            lable("Hello 6").align(Alignment::Right).wrap()
        ]),
        hbox(widgets![
            lable("Hello 7").align(Alignment::Right).wrap(),
            lable("Hello 8").align(Alignment::Left).wrap(),
            lable("Hello 9").align(Alignment::Left).wrap()
        ])
    ])
    .start();
}
