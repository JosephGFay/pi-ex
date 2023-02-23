use rust_gpiozero::*;

fn main() {
    println!("Starting blind software..");
    // Create a new LED attached to Pin 17
    let mut led = LED::new(13);

// blink the LED
// on_time: 2 seconds and off_time: 3 seconds


    led.blink(2.0,3.0);
    println!("Blinked");
}
