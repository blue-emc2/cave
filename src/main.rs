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

  write!(stdout, "{}", clear::All).unwrap();
  write!(stdout, "{}", cursor::Hide).unwrap();
  stdout.flush().unwrap();

  write!(stdout, "{} x={}, y={}", cursor::Goto(1, 10), 1, 1).unwrap();
  stdout.flush().unwrap();

  let mut keys = stdin.keys();
  let mut x: isize = 1;
  let mut y: isize = 1;
  let mut pos = (x, y);

  loop {
    // Debug
    write!(stdout, "{} x={}, y={}", cursor::Goto(1, 10), pos.0, pos.1).unwrap();
    // stdout.flush().unwrap();

    let b = keys.next().unwrap().unwrap();

    if Char('q') == b {
      return;
    }

    let accel = match b {
      Up => (0, -1),
      Down => (0, 1),
      Left => (-1, 0),
      Right => (1, 0),
      _ => (0, 0),
    };

    x += accel.0;
    y += accel.1;

    if 1 > x {
      x = 1;
    } else if 1 > y {
      y = 1;
    }

    pos.0 = x;
    pos.1 = y;

    write!(
      stdout,
      "{}{}",
      "@",
      cursor::Goto(pos.0 as u16, pos.1 as u16)
    )
    .unwrap();
    stdout.flush().unwrap();
  }
}
