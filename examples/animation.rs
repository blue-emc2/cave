use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug)]
struct Player {
  position: Point,
  sprite: Rect,
  speed: i32,
  direction: Direction,
  current_frame: i32,
}

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
  let (frame_width, frame_height) = player.sprite.size();
  let current_frame = Rect::new(
    player.sprite.x() + frame_width as i32 * player.current_frame,
    player.sprite.y() + frame_height as i32 * direction_spritesheet_row(player.direction),
    frame_width,
    frame_height,
  );

  let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);

  // screen_positionとspriteの幅と高さを渡すだけで、spriteの中心を計算して指定した座標に描画してくれる
  let screen_rect = Rect::from_center(
    screen_position,
    player.sprite.width(),
    player.sprite.height(),
  );
  canvas.copy(texture, current_frame, screen_rect).unwrap();

  canvas.present();

  Ok(())
}

fn update_player(player: &mut Player) {
  use self::Direction::*;
  match player.direction {
    Left => {
      player.position = player.position.offset(-player.speed, 0);
    }
    Right => {
      player.position = player.position.offset(player.speed, 0);
    }
    Up => {
      player.position = player.position.offset(0, -player.speed);
    }
    Down => {
      player.position = player.position.offset(0, player.speed);
    }
  }

  // 移動中のみアニメーションする
  if player.speed != 0 {
    // 3より大きくならないようにフレームを調整する
    player.current_frame = (player.current_frame + 1) % 3;
  }
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
  let texture = texture_creator.load_texture("assets/seeker.png").unwrap();

  let mut player = Player {
    position: Point::new(0, 0),
    sprite: Rect::new(0, 0, 32, 32),
    speed: 0,
    direction: Direction::Right,
    current_frame: 0,
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

    update_player(&mut player);

    render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player).unwrap();

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
  }
}
