use std::io::Stdout;

use crossterm::{QueueableCommand, style::SetBackgroundColor, terminal::{Clear, ClearType}};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
  if force {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
  }
}