#!/bin/bash

nrfjprog --family NRF54L --recover

# Build 'b'
cargo build --release --bin b --features embassy-nrf/nrf54l15-app-s

# Generate binary for 'b'
cargo objcopy --release --bin b --features embassy-nrf/nrf54l15-app-s --target thumbv8m.main-none-eabihf -- -O binary b.bin

# Flash bootloader
cargo flash --manifest-path ../../bootloader/nrf54/Cargo.toml --features embassy-nrf/nrf54l15-app-s --target thumbv8m.main-none-eabihf --chip nRF54L15

# Flash `a` (which includes b.bin)
cargo flash --release --bin a --features embassy-nrf/nrf54l15-app-s --target thumbv8m.main-none-eabihf --chip nRF54L15
