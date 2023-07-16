rust-osoyoo-car
===============

As a fun learning experiment I recently purchased [a basic Arduino-controlled robot car kit on Amazon](https://www.amazon.com/gp/product/B08DHRLJ1N/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1). This kit comes with full instructions and Arduino source code, which is fine, but I wanted to challenge myself and use this as an opportunity to learn more about Rust and robotics. This repository is for showcasing my implementation of the Osoyoo Car controls with Rust, and I will likely be implementing some fun bonus features since the car comes with multiple peripherals

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
