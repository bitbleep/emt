# emt
![](https://github.com/bitbleep/emt/workflows/Build/badge.svg)

Embedded test runner experiment in Rust.

## Runtime
The runtime `emt-rt` is a crate that you add to the firmware where you implement your tests. All tests are written using the `fn()` signature and the device is reset before each test ie. all test functions are responsible for setting up the device.

### Tests
Tests can either require human interaction or not. Those that do could for instance be tests where you need to measure a voltage on the board, press a button or similar. The runner can skip tests that require human interaction for fully automatable setups.

## Runner
The runner `emt` is a command-line tool that leverages [probe-rs](https://probe.rs) to interact with the runtime. It can either run tests on a locally connected device or one hosted by another `emt` process.

### Usage
```sh
# run all tests
# using a device connected locally
emt run

# run tests that does not require human interaction
# using a device connected locally
emt run --no-human-interaction

# host a device connected locally to be
# used by a emt runner on another machine
emt host --domain localhost --port 32100

# run all tests
# using a device hosted remotely
emt run --link hosted --domain localhost --port 32100
```

## Example
The included example is written for the nRF52832-DK.

```sh
# flash the firmware using cargo embed
cd example
cargo embed --target thumbv7em-none-eabihf

# now run the example tests using emt
cd ..
cargo run -p emt -- run
```
