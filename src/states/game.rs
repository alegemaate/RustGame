use super::state::State;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

pub struct Game {
  pub gl: GlGraphics,
  rotation: f64,
}

impl Game {
  pub fn new(gl: GlGraphics) -> Self {
    Game { gl, rotation: 0.0 }
  }
}

impl State for Game {
  fn render(&mut self, args: &RenderArgs) {
    use graphics::*;

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    let square = rectangle::square(0.0, 0.0, 50.0);
    let rotation = self.rotation;
    let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

    self.gl.draw(args.viewport(), |c, gl| {
      clear(GREEN, gl);

      let transform = c
        .transform
        .trans(x, y)
        .rot_rad(rotation)
        .trans(-25.0, -25.0);

      rectangle(RED, square, transform, gl);
    });
  }

  fn update(&mut self, args: &UpdateArgs) {
    self.rotation += 12.0 * args.dt;
  }
}
