use alloc::boxed::Box;
use esp_hal::gpio::{Input};

const EDGES_PER_DETENT: i32 = 2;

pub struct Encoder<'d> {
    a_pin: Input<'d>,
    b_pin: Input<'d>,
    c_pin: Input<'d>,
    position: i32,
    last_state: u8,
    last_button_state: bool,
    cw_callback: Option<Box<dyn FnMut()>>,
    ccw_callback: Option<Box<dyn FnMut()>>,
    bt_callback: Option<Box<dyn FnMut()>>,
}

impl<'d> Encoder<'d> {
    pub fn new(a: Input<'d>, b: Input<'d>, c: Input<'d>) -> Self {
        Encoder {
            a_pin: a,
            b_pin: b,
            c_pin: c,
            position: 0,
            last_state: 0b00,
            last_button_state: false,
            cw_callback: None,
            ccw_callback: None,
            bt_callback: None,
        }
    }

    pub fn update(&mut self) {
        let state = self.read_stable();
        let delta = match (self.last_state, state) {
            (0b00, 0b01) | (0b01, 0b11) | (0b11, 0b10) | (0b10, 0b00) => 1,   // CW
            (0b00, 0b10) | (0b10, 0b11) | (0b11, 0b01) | (0b01, 0b00) => -1,  // CCW
            _ => 0,
        };

        self.position += delta;

        if self.position % EDGES_PER_DETENT == 0 && delta != 0 {
            if delta > 0 {
                if let Some(cb) = &mut self.cw_callback { cb(); }
            } else {
                if let Some(cb) = &mut self.ccw_callback { cb(); }
            }
        }

        let pressed = self.c_pin.is_low();

        if !self.last_button_state && pressed {
            if let Some(cb) = &mut self.bt_callback {
                cb();
            }
        }

        self.last_state = state;
        self.last_button_state = pressed;
    }

    fn read_stable(&self) -> u8 {
        let mut last = (self.a_pin.is_high() as u8) << 1 | self.b_pin.is_high() as u8;
        for _ in 0..5 {
            let current = (self.a_pin.is_high() as u8) << 1 | self.b_pin.is_high() as u8;
            if current == last {
                return current;
            }
            last = current;
        }
        last
    }

    pub fn with_cw_callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.cw_callback = Some(Box::new(callback));
        self
    }

    pub fn with_ccw_callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.ccw_callback = Some(Box::new(callback));
        self
    }

    pub fn with_button_callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.bt_callback = Some(Box::new(callback));
        self
    }
}