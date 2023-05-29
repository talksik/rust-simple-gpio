use std::{thread, time::Duration};

use rppal::gpio::{Gpio, Level};

const BUTTON_PIN: u8 = 17;

// with internal pull up resistor, we get a low reading as
// the electrons flow to ground due to the voltage difference and less resistance path.
// so the mcu reads low when the button is pressed.
fn is_button_pressed(pin: &rppal::gpio::InputPin) -> bool {
    pin.is_low()
}

fn is_button_pressed2(pin: &rppal::gpio::InputPin) -> bool {
    match pin.read() {
        Level::Low => true,
        Level::High => false,
    }
}

fn read_loop(pin: &rppal::gpio::InputPin) {
    loop {
        if is_button_pressed(pin) {
            println!("Button is pressed mate!");
        }
    }
}

fn main() -> Result<(), rppal::gpio::Error> {
    println!("Hello world");

    let gpio = Gpio::new()?;
    let pin = gpio.get(BUTTON_PIN)?.into_input_pullup();

    read_loop(&pin);

    Ok(())
}
