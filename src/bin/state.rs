use alloc::string::{String, ToString};
use core::cell::{Cell, RefCell};
use crate::home::HomeScreen;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PowerSetting {
    ON,
    AUTO,
    OFF
}

#[derive(Clone)]
pub enum ActiveScreen {
    Home(HomeScreen),
}


#[derive(Clone)]
pub struct State {
    accessory_power: Cell<bool>,
    power_setting: Cell<PowerSetting>,
    antenna_up: Cell<bool>,
    voltage: Cell<f32>,
    current: Cell<f32>,
    track_title: RefCell<String>,
    track_artist: RefCell<String>,
    volume: Cell<u32>,
    current_screen: RefCell<ActiveScreen>,
}

impl State {
    pub fn new() -> Self {
        State {
            accessory_power: Cell::new(true),
            power_setting: Cell::new(PowerSetting::AUTO),
            antenna_up: Cell::new(true),
            voltage: Cell::new(13.2),
            current: Cell::new(2.6),
            track_title: RefCell::new("Plastic Beach (feat. Mick Jones and Paul Simonon)".to_string()),
            track_artist: RefCell::new("Gorillaz".to_string()),
            volume: Cell::new(50),
            current_screen: RefCell::new(ActiveScreen::Home(HomeScreen::new())),
        }
    }

    pub fn accessory_power(&self) -> bool {
        self.accessory_power.get()
    }

    pub fn set_accessory_power(&self, value: bool) {
        self.accessory_power.set(value);
    }

    pub fn power_setting(&self) -> PowerSetting {
        self.power_setting.get()
    }

    pub fn set_power_setting(&self, value: PowerSetting) {
        self.power_setting.set(value);
    }

    pub fn antenna_up(&self) -> bool {
        self.antenna_up.get()
    }

    pub fn set_antenna_up(&self, value: bool) {
        self.antenna_up.set(value);
    }

    pub fn voltage(&self) -> f32 {
        self.voltage.get()
    }

    pub fn set_voltage(&self, value: f32) {
        self.voltage.set(value);
    }

    pub fn current(&self) -> f32 {
        self.current.get()
    }

    pub fn set_current(&self, value: f32) {
        self.current.set(value);
    }

    pub fn track_title(&self) -> String {
        self.track_title.borrow().clone()
    }

    pub fn set_track_title(&self, value: &str) {
        *self.track_title.borrow_mut() = String::from(value);
    }

    pub fn track_artist(&self) -> String {
        self.track_artist.borrow().clone()
    }

    pub fn set_track_artist(&self, value: &str) {
        *self.track_artist.borrow_mut() = String::from(value);
    }

    pub fn volume(&self) -> u32 {
        self.volume.get()
    }

    pub fn set_volume(&self, value: u32) {
        self.volume.set(value);
    }

    pub fn current_screen(&self) -> core::cell::RefMut<'_, ActiveScreen> {
        self.current_screen.borrow_mut()
    }

    pub fn set_current_screen(&self, screen: ActiveScreen) {
        *self.current_screen.borrow_mut() = screen;
    }

    pub fn matches(&self, other: &State) -> bool {
        self.accessory_power.get() == other.accessory_power.get() &&
        self.power_setting.get() == other.power_setting.get() &&
        self.antenna_up.get() == other.antenna_up.get() &&
        self.voltage.get() == other.voltage.get() &&
        self.current.get() == other.current.get() &&
        self.track_title.borrow().as_str() == other.track_title.borrow().as_str() &&
        self.track_artist.borrow().as_str() == other.track_artist.borrow().as_str() &&
        core::mem::discriminant(&*self.current_screen.borrow()) == core::mem::discriminant(&*other.current_screen.borrow())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.matches(other)
    }
}