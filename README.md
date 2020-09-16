# emt
![](https://github.com/bitbleep/emt/workflows/Build/badge.svg)

Embedded test runner experiment in Rust.

## Example

```
cargo build --target thumbv7em-none-eabihf &
arm-none-eabi-objcopy -I binary target/thumbv7em-none-eabihf/debug/example -O ihex target/thumbv7em-none-eabihf/debug/example.hex &
nrfjprog --family NRF52 --program target/thumbv7em-none-eabihf/debug/example.hex --sectorerase --reset
```
