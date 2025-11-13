use alloc::format;
use alloc::string::String;
use embedded_graphics::mono_font::ascii::FONT_7X13_BOLD;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Text};
use esp_hal::Blocking;
use crate::sh1122::Sh1122;
use crate::state::{ActiveScreen, State};
use crate::screen::Screen;

pub const WIDTH: i32 = 255;
pub const HEIGHT: i32 = 63;

pub struct Display<'a> {
    driver: Sh1122<'a, Blocking>,
    last_state: Option<State>,
    last_display_update: u64,
}

impl<'a> Display<'a> {
    pub fn new(mut driver: Sh1122<'a, Blocking>) -> Self {
        driver.init().unwrap();
        driver.clear();
        driver.flush().unwrap();

        Display {
            driver,
            last_state: None,
            last_display_update: 0,
        }
    }

    pub fn update(&mut self, state: &State, time_passed: u64) {
        if self.last_state.is_none() {
            self.last_state = Some(state.clone());
            self.draw_update(&state);
            return;
        }

        let last_state = self.last_state.as_ref().unwrap();

        let should_update = match &mut *state.current_screen() {
            ActiveScreen::Home(screen) => screen.update(last_state, &state, time_passed),
        };

        if time_passed.wrapping_sub(self.last_display_update) > 200 {
            if last_state != state || should_update {
                self.draw_update(&state);
            }

            self.last_display_update = time_passed;
        }

        self.last_state = Some(state.clone());
    }

    pub fn draw_update(&mut self, state: &State) {
        self.driver.clear();
        
        match &*state.current_screen() {
            ActiveScreen::Home(screen) => screen.draw(&state, &mut self.driver),
        }

        self.driver.flush().unwrap();
    }
}

pub fn truncate(s: String, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let truncated: String = s.chars().take(max_len).collect();
        truncated + "..."
    } else {
        s
    }
}

pub fn draw_bar<D>(target: &mut D, label: &str, value: f32, min: f32, max: f32, suffix: &str) where D: DrawTarget<Color = Gray4> {
    let width = 256 - 9;
    let height = 38;
    let v = ((value - min) / (max - min)).clamp(0.0, 1.0);
    let bar_width = (width as f32) * v;

    Text::with_alignment(
        label,
        Point::new(4, 13),
        MonoTextStyle::new(&FONT_7X13_BOLD, Gray4::new(15)),
        Alignment::Left
    ).draw(target).ok();

    Text::with_alignment(
        format!("{}{}", value, suffix).as_str(),
        Point::new(WIDTH - 4, 13),
        MonoTextStyle::new(&FONT_7X13_BOLD, Gray4::new(15)),
        Alignment::Right
    ).draw(target).ok();

    Rectangle::new(Point::new(4, 20), Size::new(width, height))
        .into_styled(PrimitiveStyle::with_fill(Gray4::new(4)))
        .draw(target).ok();

    Rectangle::new(Point::new(4, 20), Size::new(bar_width as u32, height))
        .into_styled(PrimitiveStyle::with_fill(Gray4::new(15)))
        .draw(target).ok();
}