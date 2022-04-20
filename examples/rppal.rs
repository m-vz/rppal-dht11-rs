#![deny(unsafe_code)]

use rppal::{
    gpio::{Gpio, Mode},
    hal::Delay,
};
use rppal_dht11::{Dht11, Measurement};

const DHT11_PIN: u8 = 17;

fn main() -> ! {
    let pin = Gpio::new()
        .unwrap()
        .get(DHT11_PIN)
        .unwrap()
        .into_io(Mode::Output);
    let mut dht11 = Dht11::new(pin);
    let mut delay = Delay::new();

    loop {
        match dht11.perform_measurement_with_retries(&mut delay, 20) {
            Ok(Measurement {
                temperature,
                humidity,
            }) => {
                let (temperature, humidity) = (temperature as f64 / 10.0, humidity as f64 / 10.0);
                println!("Temp: {temperature:.1}C Hum: {humidity:.1}");
            }
            Err(e) => eprintln!("Failed to perform measurement: {e:?}"),
        }
    }
}
