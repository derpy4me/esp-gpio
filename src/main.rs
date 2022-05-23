#![allow(unused_imports)]
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude;
use esp_idf_hal::gpio::Input;
use esp_idf_hal::gpio::Output;
use esp_idf_hal::gpio::Pins;
use esp_idf_hal::gpio::{Gpio1, Gpio15, Gpio2, Gpio22, Gpio25, Gpio26, Gpio3, Gpio32, Gpio33};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys as _;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct EspPins {
    center: Gpio25<Output>,
    top_center: Gpio3<Output>,
    bottom_center: Gpio32<Output>,
    bottom_left: Gpio33<Output>,
    bottom_right: Gpio22<Output>,
    top_left: Gpio26<Output>,
    top_right: Gpio1<Output>,
    led_pin2: Gpio2<Output>,
    button_input1: Gpio15<Input>,
}

impl EspPins {
    fn new() -> EspPins {
        let peripherals = Peripherals::take().unwrap();
        let pins = peripherals.pins;
        EspPins {
            bottom_center: pins.gpio32.into_output().unwrap(),
            bottom_left: pins.gpio33.into_output().unwrap(),
            center: pins.gpio25.into_output().unwrap(),
            top_left: pins.gpio26.into_output().unwrap(),
            bottom_right: pins.gpio22.into_output().unwrap(),
            top_right: pins.gpio1.into_output().unwrap(),
            top_center: pins.gpio3.into_output().unwrap(),
            led_pin2: pins.gpio2.into_output().unwrap(),
            button_input1: pins.gpio15.into_input().unwrap(),
        }
    }

    fn clear_display(&mut self) {
        self.center.set_low().unwrap();
        self.top_center.set_low().unwrap();
        self.bottom_center.set_low().unwrap();
        self.bottom_left.set_low().unwrap();
        self.bottom_right.set_low().unwrap();
        self.top_left.set_low().unwrap();
        self.top_right.set_low().unwrap();
    }

    fn display_zero(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_left.set_high().unwrap();
        self.top_right.set_high().unwrap();
        self.bottom_left.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
        self.bottom_center.set_high().unwrap();
    }

    fn display_one(&mut self) {
        self.top_right.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
    }

    fn display_two(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_right.set_high().unwrap();
        self.center.set_high().unwrap();
        self.bottom_left.set_high().unwrap();
        self.bottom_center.set_high().unwrap();
    }

    fn display_three(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_right.set_high().unwrap();
        self.center.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
        self.bottom_center.set_high().unwrap();
    }

    fn display_four(&mut self) {
        self.top_left.set_high().unwrap();
        self.top_right.set_high().unwrap();
        self.center.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
    }

    fn display_five(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_left.set_high().unwrap();
        self.center.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
        self.bottom_center.set_high().unwrap();
    }

    fn display_six(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_left.set_high().unwrap();
        self.center.set_high().unwrap();
        self.bottom_left.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
        self.bottom_center.set_high().unwrap();
    }

    fn display_seven(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_right.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
    }

    fn display_eight(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_left.set_high().unwrap();
        self.top_right.set_high().unwrap();
        self.center.set_high().unwrap();
        self.bottom_left.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
        self.bottom_center.set_high().unwrap();
    }

    fn display_nine(&mut self) {
        self.top_center.set_high().unwrap();
        self.top_left.set_high().unwrap();
        self.top_right.set_high().unwrap();
        self.bottom_right.set_high().unwrap();
        self.center.set_high().unwrap();
    }
}
fn main() {
    println!("Hello World");
    let mut count: u8 = 0;
    println!("{}", count);
    let mut esp_pins = EspPins::new();
    // loop {
    //     println!("{:?}", pin22.is_high());
    //     pin2.set_high().unwrap();

    //     std::thread::sleep(std::time::Duration::from_secs(1));

    //     pin2.set_low().unwrap();

    //     println!("hello");

    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }

    loop {
        if esp_pins.button_input1.is_low().unwrap() {
            esp_pins.clear_display();
            esp_pins.led_pin2.set_high().unwrap();
            sleep(Duration::from_millis(500));
            count += 1;
        }
        match count {
            1 => esp_pins.display_one(),
            2 => esp_pins.display_two(),
            3 => esp_pins.display_three(),
            4 => esp_pins.display_four(),
            5 => esp_pins.display_five(),
            6 => esp_pins.display_six(),
            7 => esp_pins.display_seven(),
            8 => esp_pins.display_eight(),
            9 => esp_pins.display_nine(),
            _ => {
                count = 0;
                esp_pins.display_zero();
            }
        }

        esp_pins.led_pin2.set_low().unwrap();
    }
}
