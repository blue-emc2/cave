mod animator;
mod components;
mod keyboard;
mod physics;
mod renderer;

use crate::components::*;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use specs::prelude::*;
use std::time::Duration;

pub enum MovementCommand {
  Stop,
  Move(Direction),
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
  let texture_creator = canvas.texture_creator();

  // Keyboardの依存関係を指定するのがあんまり分かってない
  let mut dispatcher = DispatcherBuilder::new()
    .with(keyboard::Keyboard, "Keyboard", &[])
    .with(physics::Physics, "Physics", &["Keyboard"])
    .with(animator::Animator, "Animator", &["Keyboard"])
    .build();

  let mut world = World::new();
  dispatcher.setup(&mut world.res);
  renderer::SystemData::setup(&mut world.res);

  let movement_command: Option<MovementCommand> = None;
  world.add_resource(movement_command);

  let textures = [texture_creator.load_texture("assets/seeker.png").unwrap()];

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

  world
    .create_entity()
    .with(KeyboardControlled)
    .with(Position(Point::new(0, 0)))
    .with(Velocity {
      speed: 0,
      direction: Direction::Right,
    })
    .with(player_animation.right_frames[0].clone())
    .with(player_animation)
    .build();

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i = 0;
  'running: loop {
    let mut movement_command = None;

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
          movement_command = Some(MovementCommand::Move(Direction::Left));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Right),
          repeat: false,
          ..
        } => {
          movement_command = Some(MovementCommand::Move(Direction::Right));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Up),
          repeat: false,
          ..
        } => {
          movement_command = Some(MovementCommand::Move(Direction::Up));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Down),
          repeat: false,
          ..
        } => {
          movement_command = Some(MovementCommand::Move(Direction::Down));
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
          movement_command = Some(MovementCommand::Stop);
        }
        _ => {}
      }
    }

    *world.write_resource() = movement_command;

    i = (i + 1) % 255;
    dispatcher.dispatch(&mut world.res);
    world.maintain();

    renderer::render(
      &mut canvas,
      Color::RGB(i, 64, 255 - i),
      &textures,
      world.system_data(),
    )
    .unwrap();

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
  }
}
