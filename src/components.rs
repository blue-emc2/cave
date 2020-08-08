use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

// 現在位置を表すエンティティ
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

// 方向と速さを表すエンティティ
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
  pub speed: i32,
  pub direction: Direction,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
  // レンダリングするスプライトシートを表す番号
  pub spritesheet: usize,
  // レンダリングするスプライトの領域
  pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
  pub current_frame: usize,
  pub up_frames: Vec<Sprite>,
  pub down_frames: Vec<Sprite>,
  pub left_frames: Vec<Sprite>,
  pub right_frames: Vec<Sprite>,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;
