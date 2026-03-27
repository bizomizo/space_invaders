use std::error::Error;
use std::time::Duration;
use std::sync::mpsc;
use crate::event::{
  Event,
  KeyCode
};
use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    event,
};
use rusty_audio::Audio;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    
    audio.play("startup");

    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();



    //Game Loop
    'gameloop: loop {
      //input
      while event::poll(Duration::default())? {
        if let Event::Key(key_event) = event::read()? {
          match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
              audio.play("lose");
              break 'gameloop;
            }
            _ => {}
          }
        }
      }
    }







    //cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
