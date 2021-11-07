use std::time::Duration;

use anyhow::Result;
use dht11::Dht11;
use rppal::gpio::{Gpio, Mode};
use rppal::hal::Delay;

const DATA_PIN: u8 = 4;

fn main() -> Result<()> {
    let pin = Gpio::new()?.get(DATA_PIN)?.into_io(Mode::Output);
    let mut delay = Delay::new();
    let mut dht11 = Dht11::new(pin);

    loop {
        let measurement = match dht11.perform_measurement(&mut delay) {
            Ok(m) => m,
            Err(e) => {
                println!("Error getting measurement: {:?}", e);
                continue;
            }
        };

        println!(
            "Temperature: {}degF, humidity: {}%",
            measurement.temperature as f32 / 10.0 * 1.8 + 32.0,
            measurement.humidity as f32 / 10.0
        );
        std::thread::sleep(Duration::from_secs(5));
    }
}
