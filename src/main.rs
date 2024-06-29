#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};
use esp32s3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};
use esp32s3_hal::entry;


// use two additional modules
use hal::{IO, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    println!("Hello world!");

    // create a delay object
    let mut delay = Delay::new(&clocks);

    // create an io object
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // from the io object, get pin GPIO2 LED
    let mut led = io.pins.gpio2.into_push_pull_output();

    loop {
        // delay for 1000 milli-seconds
        delay.delay_ms(1000u32);   

        // toggle pin led
        led.toggle().unwrap();

        // print out to serial ...
        println!("...");
    }
}