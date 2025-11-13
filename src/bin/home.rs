use alloc::format;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13, FONT_7X13_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::{DrawTarget, Primitive};
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::{Alignment, Text};
use esp_println::println;
use crate::display;
use crate::screen::{InputEvent, Screen};
use crate::state::State;

#[derive(Clone)]
pub struct HomeScreen {
    volume_timer: u64
}

impl HomeScreen {
    pub(crate) fn new() -> Self {
        HomeScreen {
            volume_timer: 0
        }
    }
}

impl Screen for HomeScreen {
    fn draw<D>(&self, state: &State, target: &mut D) where D: DrawTarget<Color = Gray4> {
        if self.volume_timer > 0 {
            display::draw_bar(target, "VOLUME", state.volume() as f32, 0_f32, 100_f32, "%");
            return;
        }

        // Voltage
        Text::with_alignment(
            format!("{} V", state.voltage()).as_str(),
            Point::new(0, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Left
        ).draw(target).ok();

        // Power
        Text::with_alignment(
            format!("{} W", (state.voltage() * state.current()) as i32).as_str(),
            Point::new(48, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Left
        ).draw(target).ok();

        // Antenna
        Text::with_alignment(
            if state.antenna_up() { "UP" } else { "DOWN" },
            Point::new(crate::display::WIDTH - 36, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Right
        ).draw(target).ok();

        // Power State
        Text::with_alignment(
            match state.power_setting() {
                crate::state::PowerSetting::ON => "ON",
                crate::state::PowerSetting::AUTO => "AUTO",
                crate::state::PowerSetting::OFF => "OFF",
            },
            Point::new(display::WIDTH, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Right
        ).draw(target).ok();

        // Track Title
        Text::with_alignment(
            display::truncate(state.track_title(), 34).as_str(),
            Point::new(0, 32),
            MonoTextStyle::new(&FONT_7X13_BOLD, Gray4::new(15)),
            Alignment::Left
        ).draw(target).ok();

        // Track Artist
        Text::with_alignment(display::truncate(state.track_artist(), 40).as_str(),
            Point::new(0, 48),
            MonoTextStyle::new(&FONT_7X13, Gray4::new(15)),
            Alignment::Left
        ).draw(target).ok();

        // Track Progress
        Line::new(Point::new(0, display::HEIGHT), Point::new(display::WIDTH, display::HEIGHT))
            .into_styled(PrimitiveStyle::with_stroke(Gray4::new(15), 1))
            .draw(target).ok();
    }
    fn update(&mut self, prev_state: &State, state: &State, time_passed: u64) -> bool {
        if state.volume() != prev_state.volume() {
            println!("Volume changed to {}", prev_state.volume());
            self.volume_timer = 1000;
            return true;
        }

        let should_update = self.volume_timer > 0 && time_passed > self.volume_timer;
        self.volume_timer = self.volume_timer.saturating_sub(time_passed);


        should_update
    }

    fn handle_event(&self, state: &State, input: InputEvent) {

    }
}