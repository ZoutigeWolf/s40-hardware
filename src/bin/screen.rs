use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Gray4;
use crate::state::State;

pub enum InputEvent {
    EncoderCW,
    EncoderCCW,
    EncoderBT,
}

pub trait Screen {
    fn draw<D>(&self, state: &State, target: &mut D) where D: DrawTarget<Color = Gray4>;

    fn update(&mut self, prev_state: &State, state: &State, time_passed: u64) -> bool;

    fn handle_event(&self, state: &State, input: InputEvent);
}