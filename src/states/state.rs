use piston::input::{RenderArgs, UpdateArgs};

pub trait State {
  fn update(&mut self, args: &UpdateArgs);
  fn render(&mut self, args: &RenderArgs);
}

pub struct StateEngine {
  state: Box<dyn State>,
}

impl StateEngine {
  pub fn new(state: Box<dyn State>) -> Self {
    StateEngine { state }
  }

  pub fn set_state(&mut self, state: Box<dyn State>) {
    self.state = state
  }

  pub fn render(&mut self, args: &RenderArgs) {
    self.state.render(args)
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    self.state.update(args)
  }
}
