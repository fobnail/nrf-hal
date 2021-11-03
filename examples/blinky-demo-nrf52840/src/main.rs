#![no_main]
#![no_std]

use nrf52840_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::gpio::Level;
use hal::Timer;
use rtt_target::{rprintln, rtt_init_print};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Blinky demo starting");

    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut timer: Timer<hal::pac::TIMER0, hal::timer::Periodic> = Timer::periodic(p.TIMER0);

    // Power on LEDs (active-low)
    let mut green_led = port0.p0_06.into_push_pull_output(Level::Low);
    let mut red_led = port0.p0_08.into_push_pull_output(Level::Low);

    loop {
        // Blink with 1 second intervals
        timer.delay(Timer::<hal::pac::TIMER0, hal::timer::Periodic>::TICKS_PER_SECOND);
        green_led.set_high().unwrap();
        red_led.set_high().unwrap();
        timer.delay(Timer::<hal::pac::TIMER0, hal::timer::Periodic>::TICKS_PER_SECOND);
        green_led.set_low().unwrap();
        red_led.set_low().unwrap();
    }
}
