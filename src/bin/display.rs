use alloc::format;
use alloc::string::{String};
use embedded_graphics::{Drawable};
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_6X12, FONT_6X9, FONT_7X13, FONT_7X13_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Gray4};
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::{Rectangle, PrimitiveStyle, Line};
use embedded_graphics::text::{Alignment, Text};
use esp_hal::Blocking;

use crate::sh1122::Sh1122;
use crate::state::State;

const WIDTH: i32 = 255;
const HEIGHT: i32 = 63;

pub struct Display<'a> {
    driver: Sh1122<'a, Blocking>,
    last_state: Option<State>,
}

impl<'a> Display<'a> {
    pub fn new(mut driver: Sh1122<'a, Blocking>) -> Self {
        driver.init().unwrap();
        driver.clear();
        driver.flush().unwrap();

        Display {
            driver,
            last_state: None,
        }
    }

    pub fn update(&mut self, state: &State) {
        if let Some(last_state) = &self.last_state && last_state == state {
            return;
        }

        self.draw_home(state);
        self.driver.flush().unwrap();

        self.last_state = Some((*state).clone());
    }

    fn draw_volume(&mut self, state: &State) {
        self.draw_bar("VOLUME", 50_f32, 0_f32, 100_f32, "%");
    }

    fn draw_bass(&mut self, state: &State) {
        self.draw_bar("BASS", 6_f32, -12_f32, 12_f32, " dB");
    }

    fn draw_home(&mut self, state: &State) {
        // Voltage
        Text::with_alignment(
            format!("{} V", state.voltage()).as_str(),
            Point::new(0, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Left
        ).draw(&mut self.driver).unwrap();

        // Power
        Text::with_alignment(
            format!("{} W", (state.voltage() * state.current()) as i32).as_str(),
            Point::new(48, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Left
        ).draw(&mut self.driver).unwrap();

        // Antenna
        Text::with_alignment(
            if state.antenna_up() { "UP" } else { "DOWN" },
            Point::new(WIDTH - 36, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Right
        ).draw(&mut self.driver).unwrap();

        // Power State
        Text::with_alignment(
            match state.power_setting() {
                crate::state::PowerSetting::ON => "ON",
                crate::state::PowerSetting::AUTO => "AUTO",
                crate::state::PowerSetting::OFF => "OFF",
            },
            Point::new(WIDTH, 9),
            MonoTextStyle::new(&FONT_6X10, Gray4::new(15)),
            Alignment::Right
        ).draw(&mut self.driver).unwrap();

        // Track Title
        Text::with_alignment(
            truncate(state.track_title(), 34).as_str(),
            Point::new(0, 32),
            MonoTextStyle::new(&FONT_7X13_BOLD, Gray4::new(15)),
            Alignment::Left
        ).draw(&mut self.driver).unwrap();

        // Track Artist
        Text::with_alignment(
            truncate(state.track_artist(), 40).as_str(),
            Point::new(0, 48),
            MonoTextStyle::new(&FONT_7X13, Gray4::new(15)),
            Alignment::Left
        ).draw(&mut self.driver).unwrap();

        // Track Progress
        Line::new(Point::new(0, HEIGHT), Point::new(WIDTH, HEIGHT))
            .into_styled(PrimitiveStyle::with_stroke(Gray4::new(15), 1))
            .draw(&mut self.driver).unwrap();
    }

    fn draw_bar(&mut self, label: &str, value: f32, min: f32, max: f32, suffix: &str)
    {
        let width = 256 - 9;
        let height = 38;
        let v = ((value - min) / (max - min)).clamp(0.0, 1.0);
        let bar_width = (width as f32) * v;

        Text::with_alignment(
            label,
            Point::new(4, 13),
            MonoTextStyle::new(&FONT_7X13_BOLD, Gray4::new(15)),
            Alignment::Left
        ).draw(&mut self.driver).unwrap();

        Text::with_alignment(
            format!("{}{}", value, suffix).as_str(),
            Point::new(WIDTH - 4, 13),
            MonoTextStyle::new(&FONT_7X13_BOLD, Gray4::new(15)),
            Alignment::Right
        ).draw(&mut self.driver).unwrap();

        Rectangle::new(Point::new(4, 20), Size::new(width, height))
            .into_styled(PrimitiveStyle::with_fill(Gray4::new(4)))
            .draw(&mut self.driver).unwrap();

        Rectangle::new(Point::new(4, 20), Size::new(bar_width as u32, height))
            .into_styled(PrimitiveStyle::with_fill(Gray4::new(15)))
            .draw(&mut self.driver).unwrap();
    }
}

fn truncate(s: String, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let truncated: String = s.chars().take(max_len).collect();
        truncated + "..."
    } else {
        s
    }
}