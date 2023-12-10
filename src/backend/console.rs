use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::Print,
    terminal::Clear,
};
use std::io::{stdout, Stdout, Write};
pub struct Console {
    pub stdout: Stdout,
}
impl Console {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn draw(&mut self, pos: (f32, f32), text: impl Into<String>) {
        crossterm::queue!(
            self.stdout,
            MoveTo(pos.0.floor() as u16, pos.1.floor() as u16)
        )
        .unwrap();
        self.write(text.into());
        self.flush();
    }

    pub fn write(&mut self, text: String) {
        crossterm::queue!(self.stdout, Print(text)).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
        crossterm::execute!(self.stdout, Hide).unwrap();
    }

    pub fn max_xy(&mut self) -> (f32, f32) {
        let s = crossterm::terminal::size().unwrap();
        (s.0 as f32, s.1 as f32)
    }

    pub fn clear(&mut self) {
        crossterm::execute!(
            self.stdout,
            Clear(crossterm::terminal::ClearType::All),
            Show
        )
        .unwrap();
    }
}
