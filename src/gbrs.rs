#![feature(globs)]
extern crate serialize;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_game_window;

use std::io::File;
use cpu::CPU;
use mmu::MMU;
use opengl_graphics::{Gl, Texture};
use graphics::*;
use sdl2_game_window::WindowSDL2;
use piston::{
  EventIterator,
  EventSettings,
  WindowSettings,
  Input,
  Update,
  Render
};
use piston::image;
use piston::image::GenericImage;

mod cpu;
mod mmu;

fn main() {
  let opengl = piston::shader_version::opengl::OpenGL_3_2;
  let mut window: WindowSDL2 = WindowSDL2::new(
    opengl,
    WindowSettings {
      title: "GB MMU".to_string(),
      size: [256, 256],
      fullscreen: false,
      exit_on_esc: true,
      samples: 0
    }
  );
  let ref mut gl = Gl::new(opengl);
  let mut image = image::ImageBuf::new(256, 256);
  let mut texture = Texture::from_image(&image);

  gl.viewport(0, 0, 256, 256);

  let game_iter_settings = EventSettings {
    updates_per_second: 8000000, // 8Mhz
    max_frames_per_second: 8000000,
  };

  match File::open(&Path::new("data/Tetris.gb")).read_to_end() {
    Ok(contents) => {
      let program = contents.as_slice();
      let mut mmu: MMU = MMU::new();
      mmu.load_rom(program);
      let mut cpu: CPU = CPU::new(mmu);

      println!("Loaded ROM and beginning emulation");
      for e in EventIterator::new(&mut window, &game_iter_settings) {
        match e {
          Update(args) => {
            let instruction = cpu.take_byte();
            cpu.execute(instruction);
          },
          Render(_) => {

            for i in range(0u16, 0xffff) {
              let x = (i % 256) as u32;
              let y = (i / 256) as u32;
              let p = mmu.read_byte(i);
              if p != 0 {
                println!("({}, {}) = {}", x, y, p);
              }
              image.put_pixel(x, y, image::Rgba(p & 0xFFF00000, p & 0x000fff00, p & 0x000000ff, 255));
              image.put_pixel(x, y, image::Rgba(p, p << 3, p << 6, 255));
            }

            texture.update(&image);
            let c = Context::abs(256.0, 256.0);
            c.rgb(1.0, 1.0, 1.0).draw(gl);
            c.image(&texture).draw(gl);
          }
          _ => {}
        }
      }
    },
    _ => println!("Failed to read ROM")
  }
}
