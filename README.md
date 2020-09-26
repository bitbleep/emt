# emt
![](https://github.com/bitbleep/emt/workflows/Build/badge.svg)

Embedded test runner experiment in Rust.

## Example
The included example is written for the nRF52832-DK.

```sh
# flash the firmware using cargo embed
cd example
cargo embed --target thumbv7em-none-eabihf

# now run the example tests using emt
cd ..
cargo run -p emt
```
