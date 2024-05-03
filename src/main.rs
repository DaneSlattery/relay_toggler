#![no_std]
#![no_main]
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut off_pin = io.pins.gpio36.into_push_pull_output();
    let mut on_pin = io.pins.gpio35.into_push_pull_output();

    let delay = Delay::new(&clocks);

    println!("Start");
    const PULSE_WIDTH: u32 = 200;
    const SWITCH_PERIOD: u32 = 600;
    loop {
        println!("On");
        on_pin.set_low();
        delay.delay_millis(PULSE_WIDTH);
        on_pin.set_high();
        delay.delay_millis(SWITCH_PERIOD);
        println!("Off");
        off_pin.set_low();
        delay.delay_millis(PULSE_WIDTH);
        off_pin.set_high();
        delay.delay_millis(SWITCH_PERIOD);
    }
}
