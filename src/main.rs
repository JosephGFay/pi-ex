extern crate rust_gpiozero;
use rust_gpiozero::*;

fn main() {
    println!("Starting blink software..");
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);

// blink the LED
// on_time: 2 seconds and off_time: 3 seconds


    led.blink(0.2, 0.3);
    println!("Blink ran");
}
