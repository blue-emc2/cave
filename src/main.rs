extern crate termion;
use crate::termion::input::TermRead;
use std::io::{stdin, stdout, Write};
use termion::event::Key::{Char, Down, Left, Right, Up};
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

fn main() {
  let stdout = stdout();
  let mut stdout = stdout.lock().into_raw_mode().unwrap();
  let stdin = stdin();
  let stdin = stdin.lock();

  write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
  stdout.flush().unwrap();

  write!(stdout, "{}", "@").unwrap();
  stdout.flush().unwrap();

  let mut keys = stdin.keys();
  let mut x: isize = 1;
  let mut y: isize = 1;
  let mut pos = (x, y);

  loop {
    let b = keys.next().unwrap().unwrap();

    if Char('q') == b {
      return;
    }

    let accel = match b {
      Up => (0, -1),
      Down => (0, 1),
      Left => (-1, 0),
      Right => (0, 1),
      _ => (0, 0),
    };

    x += accel.0;
    y += accel.1;
    pos.0 = x;
    pos.1 = y;

    write!(
      stdout,
      "{}{}{}",
      clear::All,
      "@",
      cursor::Goto(pos.0 as u16, pos.1 as u16)
    )
    .unwrap();

    // Debug
    // write!(stdout, "x={}, y={}", x, y).unwrap();

    stdout.flush().unwrap();
  }
}
