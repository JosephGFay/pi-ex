use gpio::{GpioIn, GpioOut};

fn main() {
    println!("Starting pi-ex");
    let mut buzzer_pin = gpio::sysfs::SysFsGpioOutput::open(36).unwrap();
    println!("Pin Set");
    buzzer_pin.set_high().expect("Failed to set high");
    println!("Program finished without issue.");
    // COMMMMMMMENTS
}
