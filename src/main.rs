use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

mod states;

use crate::states::game::Game;
use crate::states::menu::Menu;
use crate::states::state::StateEngine;

fn main() {
  // Change this to OpenGL::V2_1 if not working.
  let opengl = OpenGL::V3_2;

  // Create an Glutin window.
  let mut window: Window = WindowSettings::new("spinning-square", [800, 600])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create a new game and run it.
  let mut cur_state = StateEngine::new(Box::new(Menu::new(GlGraphics::new(opengl))));

  // State manager
  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      cur_state.render(&args);
    }

    if let Some(args) = e.update_args() {
      cur_state.update(&args);
    }

    if let Some(Button::Keyboard(_key)) = e.press_args() {
      cur_state.set_state(Box::new(Game::new(GlGraphics::new(opengl))))
    };
  }
}
