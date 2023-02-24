extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    println!("Starting blink software..");
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);

    loop{
        // Make the led switch on
        led.on();

        // Let the LED stay on for one second
        sleep(Duration::from_secs(1));

        // Make the led switch off
        led.off();

        // Let the LED stay off for one second
        sleep(Duration::from_secs(1));
        println!("Blinked");

    }
}

