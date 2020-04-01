// Windowsのコマンドプロンプトウィンドウを抑止
// - debug   ウィンドウを出す
// - release ウィンドウを抑止する
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;


use sdl2::rect::{Point, Rect};

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &Texture,
) -> Result<(), String> {

    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.clear();

    let (width, height) = canvas.output_size().expect("get output size failed");

    // (Point::new(x, y), width, height)
    let screen_rect = Rect::from_center(Point::new(300, 400), 656, 913);

    canvas.copy(&textures, None, screen_rect).expect("canvas copy failed");
    canvas.present();

    // canvas.copy(&textures, None, None).expect("render failed");
    // canvas.present();

    Ok(())
}



pub fn main() {
    let sdl_context = sdl2::init().expect("init failed");
    let video_subsystem = sdl_context.video().expect("get video subsystem failed");

    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("image init failed");

    // let window = video_subsystem.window("rust-sdl2 image demo", 748, 1000)
    let window = video_subsystem.window("rust-sdl2 image demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("failed to create window");

    let mut canvas = window.into_canvas()
        .build()
        .expect("failed to convert canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let image_texture = texture_creator.load_texture("assets/my_image.png")
        .expect("failed to load image");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // draw
        render(&mut canvas, &image_texture);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
