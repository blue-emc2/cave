mod animator;
mod components;
mod physics;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

fn direction_spritesheet_row(direction: Direction) -> i32 {
  use self::Direction::*;
  match direction {
    Up => 3,
    Down => 0,
    Left => 1,
    Right => 2,
  }
}

fn render(
  canvas: &mut WindowCanvas,
  color: Color,
  texture: &Texture,
  player: &Player,
) -> Result<(), String> {
  canvas.set_draw_color(color);
  canvas.clear();

  let (width, height) = canvas.output_size().unwrap();

  let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);

  // screen_positionとspriteの幅と高さを渡すだけで、spriteの中心を計算して指定した座標に描画してくれる
  let screen_rect = Rect::from_center(
    screen_position,
    player.sprite.width(),
    player.sprite.height(),
  );
  canvas.copy(texture, player.sprite, screen_rect).unwrap();

  canvas.present();

  Ok(())
}

fn update_player(player: &mut Player) {
  // 移動中のみアニメーションする
  if player.speed != 0 {
    // 3より大きくならないようにフレームを調整する
    player.current_frame = (player.current_frame + 1) % 3;
  }
}

fn character_animation_frames(
  spritesheet: usize,
  top_left_frame: Rect,
  direction: Direction,
) -> Vec<Sprite> {
  let (frame_width, frame_height) = top_left_frame.size();
  let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

  let mut frames = Vec::new();
  for i in 0..3 {
    frames.push(Sprite {
      spritesheet,
      region: Rect::new(
        top_left_frame.x() + frame_width as i32 * i,
        y_offset,
        frame_width,
        frame_height,
      ),
    })
  }

  frames
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

  let window = video_subsystem
    .window("SDL", 640, 480)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();

  let texture_creator = canvas.texture_creator();
  let textures = [texture_creator.load_texture("assets/seeker.png").unwrap()];

  // texturesの先頭はプレイヤーなので0を設定
  let player_spritesheet = 0;
  let player_top_left_frame = Rect::new(0, 0, 32, 32);
  let player_animation = MovementAnimation {
    current_frame: 0,
    up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
    down_frames: character_animation_frames(
      player_spritesheet,
      player_top_left_frame,
      Direction::Down,
    ),
    left_frames: character_animation_frames(
      player_spritesheet,
      player_top_left_frame,
      Direction::Left,
    ),
    right_frames: character_animation_frames(
      player_spritesheet,
      player_top_left_frame,
      Direction::Right,
    ),
  };

  let mut world = World::new();
  world
    .create_entity()
    .with(Position(Point::new(0, 0)))
    .with(Velocity {
      speed: 0,
      direction: Direction::Right,
    })
    .with(player_animation.right_frames[0].clone())
    .with(player_animation)
    .build();

  let mut player = Player {
    position: Point::new(0, 0),
    sprite: Rect::new(0, 0, 32, 32),
    speed: 0,
    direction: Direction::Right,
  };

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i = 0;
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        Event::KeyDown {
          keycode: Some(Keycode::Left),
          repeat: false,
          ..
        } => {
          player.speed = PLAYER_MOVEMENT_SPEED;
          player.direction = Direction::Left;
        }
        Event::KeyDown {
          keycode: Some(Keycode::Right),
          repeat: false,
          ..
        } => {
          player.speed = PLAYER_MOVEMENT_SPEED;
          player.direction = Direction::Right;
        }
        Event::KeyDown {
          keycode: Some(Keycode::Up),
          repeat: false,
          ..
        } => {
          player.speed = PLAYER_MOVEMENT_SPEED;
          player.direction = Direction::Up;
        }
        Event::KeyDown {
          keycode: Some(Keycode::Down),
          repeat: false,
          ..
        } => {
          player.speed = PLAYER_MOVEMENT_SPEED;
          player.direction = Direction::Down;
        }
        Event::KeyUp {
          keycode: Some(Keycode::Left),
          repeat: false,
          ..
        }
        | Event::KeyUp {
          keycode: Some(Keycode::Right),
          repeat: false,
          ..
        }
        | Event::KeyUp {
          keycode: Some(Keycode::Up),
          repeat: false,
          ..
        }
        | Event::KeyUp {
          keycode: Some(Keycode::Down),
          repeat: false,
          ..
        } => {
          player.speed = 0;
        }
        _ => {}
      }
    }

    i = (i + 1) % 255;

    // TODO: Do with specs!

    render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player).unwrap();

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
  }
}
