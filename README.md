# rppal-dht11-rs

[![crates.io badge](https://img.shields.io/crates/v/rppal-dht11.svg)](https://crates.io/crates/rppal-dht11)
[![docs.rs badge](https://docs.rs/rppal-dht11/badge.svg)](https://docs.rs/rppal-dht11)

Raspberry Pi Rust driver for the DHT11 temperature and humidity sensor, compatible with the [rppal](https://docs.golemparts.com/rppal/0.13.1/rppal/gpio/struct.IoPin.html#) GPIO library `IoPin` type.

Based on [this driver](https://github.com/plorefice/dht11-rs), but modified to work on my Raspberry Pi 4B.

## Requirements

- Rust 1.43+

## Usage

Include library as a dependency in your Cargo.toml

```toml
[dependencies]
rppal-dht11 = "0.3.1"
```

```rust
use rppal_dht11::Dht11;
use rppal::{gpio::Gpio, hal::Delay};

let pin = Gpio::new()?.get(DHT11_PIN)?.into_output_low();
// Create an instance of the DHT11 device
let mut dht11 = Dht11::new(pin);
let mut delay = Delay::new();

// Perform a sensor reading
let measurement = dht11.perform_measurement(&mut delay).unwrap();
println!("{:?}", measurement);
```

## License

Copyright © 2020 Pietro Lorefice

Copyright © 2022 Xavientois

Dual licensed under your choice of either of:

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
