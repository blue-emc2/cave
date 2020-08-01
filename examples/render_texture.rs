use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

#[derive(Debug)]
struct Player {
  position: Point,
  sprite: Rect,
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

  let player = Player {
    position: Point::new(0, 0),
    sprite: Rect::new(0, 0, 32, 32),
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
        _ => {}
      }
    }

    i = (i + 1) % 255;

    render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player).unwrap();

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
