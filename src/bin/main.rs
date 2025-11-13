#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use alloc::rc::Rc;
use alloc::string::{ ToString};
use core::cell::RefCell;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::clock::{Clock, CpuClock};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::i2c::master as I2C;
use esp_hal::system::software_reset;
use esp_hal::time::Rate;
use esp_hal::uart as UART;
use esp_hal::xtensa_lx::timer::{delay, get_cycle_count};
use esp_hal::{Blocking, Config, main};
use esp_println::{print, println};

mod encoder;
use encoder::Encoder;

mod sh1122;
use sh1122::Sh1122;

mod display;
use display::Display;

mod screen;

mod home;

mod state;
use state::State;
use crate::screen::{InputEvent, Screen};
use crate::state::ActiveScreen;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic: {:?}", info);
    software_reset();
}

esp_app_desc!();

fn millis(config: &Config) -> u64 {
    let cycles = get_cycle_count() as u64;
    cycles * 1000 / (config.cpu_clock().hz() as u64)
}

fn delay_ms(config: &Config, mut ms: u64) {
    let freq = config.cpu_clock().hz() as u64;
    while ms > 0 {
        let chunk = ms.min(17_000);
        let cycles = freq * chunk / 1000;
        delay(cycles as u32);
        ms -= chunk;
    }
}

#[main]
fn main() -> ! {
    esp_alloc::heap_allocator!(size: 92 * 1024); // 92 KB heap

    let system_config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(system_config);

    let mut uart = UART::Uart::new(peripherals.UART0, UART::Config::default())
        .unwrap()
        .with_tx(peripherals.GPIO1)
        .with_rx(peripherals.GPIO3);

    let i2c_config = I2C::Config::default().with_frequency(Rate::from_khz(400));
    let mut i2c = I2C::I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let driver = Sh1122::new(&mut i2c, 0x3C);
    let mut display = Display::new(driver);

    let power_relay_pin = Output::new(
        peripherals.GPIO16,
        Level::High,
        OutputConfig::default(),
    );

    let antenna_relay_pin = Output::new(
        peripherals.GPIO15,
        Level::High,
        OutputConfig::default(),
    );

    let acc_pin = Input::new(
        peripherals.GPIO5,
        InputConfig::default().with_pull(Pull::Up),
    );

    let encoder_0a_pin = Input::new(
        peripherals.GPIO19,
        InputConfig::default().with_pull(Pull::Up),
    );
    let encoder_0b_pin = Input::new(
        peripherals.GPIO18,
        InputConfig::default().with_pull(Pull::Up),
    );
    let encoder_0c_pin = Input::new(
        peripherals.GPIO17,
        InputConfig::default().with_pull(Pull::Up),
    );

    let state = Rc::new(RefCell::new(State::new()));

    let mut encoder_0 = Encoder::new(encoder_0a_pin, encoder_0b_pin, encoder_0c_pin)
        .with_cw_callback({
            let state = Rc::clone(&state);
            move || {
                let s = state.borrow_mut();
                s.set_volume((s.volume() + 2).min(100));
            }
        })
        .with_ccw_callback({
            let state = Rc::clone(&state);
            move || {
                let s = state.borrow_mut();
                s.set_volume(s.volume().saturating_sub(2));
            }
        })
        .with_button_callback({
            move || { /* optional button logic */ }
        });

    loop {
        let time_passed = millis(&system_config);

        encoder_0.update();

        display.update(&state.borrow(), time_passed);

        delay_ms(&system_config, 5);
    }
}
