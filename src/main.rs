extern crate termion;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, cursor, style};

fn main() {
  let stdout = stdout();
  let mut stdout = stdout.lock().into_raw_mode().unwrap();
  let stdin = async_stdin();

  write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
  stdout.flush().unwrap();

  write!(stdout, "{}{}", clear::All, style::Reset);
  write!(stdout, "{}", "@").unwrap();
  stdout.flush().unwrap();
}
