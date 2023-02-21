use gpio::{GpioIn, GpioOut};

fn main() {
    let mut buzzer_pin = gpio::sysfs::SysFsGpioOutput::open(36).unwrap();

    buzzer_pin.set_high().expect("Failed to set high");
}
