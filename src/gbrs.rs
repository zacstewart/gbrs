#![feature(globs)]
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

fn draw_mmu(mmu: MMU, texture: Texture, gl: &mut Gl) {
  for i in range(0u16, 0xffff) {
    let x = (i % 256) as u32;
    let y = (i / 256) as u32;
    let p = mmu.read_byte(i);

    let r = (p & 0xe0);
    let g = (p & 0x1c) << 3;
    let b = (p & 0x7) << 5;
    //image.put_pixel(x, y, image::Rgba(r, g, b, 255));
  }

  //texture.update(&image);

  let c = Context::abs(256.0, 256.0);
  c.rgb(1.0, 1.0, 1.0).draw(gl);
  c.image(&texture).draw(gl);
}

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

  match File::open(&Path::new("data/Tetris.gb")).read_to_end() {
    Ok(contents) => {
      let program = contents.as_slice();
      let mut mmu: MMU = MMU::new();
      mmu.load_rom(program);
      let mut cpu: CPU = CPU::new(mmu);

      println!("Loaded ROM and beginning emulation");
      loop {
        let instruction = cpu.take_byte();
        cpu.execute(instruction);
      }
    },
    _ => fail!("Failed to read ROM.")
  }
}
