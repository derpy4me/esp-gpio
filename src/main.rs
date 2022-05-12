#![allow(unused_imports)]
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys as _;

fn main() {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut pin2 = pins.gpio2.into_output().unwrap();
    pin2.set_high().unwrap();

    println!("hello");
}
