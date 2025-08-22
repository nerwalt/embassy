# Bootloader for nRF

The bootloader uses `embassy-boot` to interact with the flash.

# Usage

Flash the bootloader

```
cargo flash --features embassy-nrf/nrf54l15-app-s --release --chip nRF54L15
```
