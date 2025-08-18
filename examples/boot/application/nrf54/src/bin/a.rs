#![no_std]
#![no_main]
#![macro_use]
// TODO FIXME
#![allow(unused)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use embassy_boot::State;
use embassy_boot_nrf::{FirmwareUpdater, FirmwareUpdaterConfig};
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::nvmc::Nvmc;
use embassy_nrf::wdt::{self, Watchdog};
use embassy_nrf::{peripherals, Peri};
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use panic_reset as _;

#[cfg(feature = "skip-include")]
static APP_B: &[u8] = &[0, 1, 2, 3];
#[cfg(not(feature = "skip-include"))]
static APP_B: &[u8] = include_bytes!("../../b.bin");

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    #[cfg(feature = "defmt")]
    defmt::info!("Running!");

    // nRF54l15 PDK
    // LED 0
    let mut led = Output::new(p.P2_09, Level::Low, OutputDrive::Standard);
    // Buttom 0
    let mut button = Input::new(p.P1_13, Pull::Up);
    // LED 1
    let mut led_reverted = Output::new(p.P1_10, Level::Low, OutputDrive::Standard);

    // The following code block illustrates how to obtain a watchdog that is configured
    // as per the existing watchdog. Ordinarily, we'd use the handle returned to "pet" the
    // watchdog periodically. If we don't, and we're not going to for this example, then
    // the watchdog will cause the device to reset as per its configured timeout in the bootloader.
    // This helps is avoid a situation where new firmware might be bad and block our executor.
    // If firmware is bad in this way then the bootloader will revert to any previous version.
    // let wdt_config = wdt::Config::try_new(&p.WDT0).unwrap();
    // let (_wdt, [_wdt_handle]) = match Watchdog::try_new(p.WDT0, wdt_config) {
    //     Ok(x) => x,
    //     Err(_) => {
    //         // Watchdog already active with the wrong number of handles, waiting for it to timeout...
    //         loop {
    //             cortex_m::asm::wfe();
    //         }
    //     }
    // };
    spawner.spawn(watchdog_task(p.WDT0)).unwrap();

    let nvmc = Nvmc::new(p.RRAMC);
    let nvmc = Mutex::new(BlockingAsync::new(nvmc));

    let config = FirmwareUpdaterConfig::from_linkerfile(&nvmc, &nvmc);
    let mut magic = [0; 16];
    let mut updater = FirmwareUpdater::new(config, &mut magic);
    let state = updater.get_state().await.unwrap();
    if state == State::Revert {
        led_reverted.set_low();
    } else {
        led_reverted.set_high();
    }

    loop {
        led.set_high();
        button.wait_for_any_edge().await;
        if button.is_low() {
            let mut offset = 0;
            for chunk in APP_B.chunks(4096) {
                let mut buf: [u8; 4096] = [0; 4096];
                buf[..chunk.len()].copy_from_slice(chunk);
                updater.write_firmware(offset, &buf).await.unwrap();
                offset += chunk.len();
            }
            updater.mark_updated().await.unwrap();
            led.set_low();
            cortex_m::peripheral::SCB::sys_reset();
        }
    }
}

#[embassy_executor::task]
pub async fn watchdog_task(wdt: Peri<'static, peripherals::WDT0>) {
    #[cfg(feature = "defmt")]
    defmt::trace!("watchdog_task::Launch");

    let wdt_config = wdt::Config::try_new(&wdt).unwrap();

    let period = (wdt_config.timeout_ticks / 2) as u64;

    let (_wdt, [mut wdt_handle]) = match Watchdog::try_new(wdt, wdt_config) {
        Ok(x) => x,
        Err(_) => {
            // Watchdog already active with the wrong number of handles, waiting for it to timeout...
            loop {
                cortex_m::asm::wfe();
            }
        }
    };

    loop {
        Timer::after_ticks(period).await;
        wdt_handle.pet();
    }
}
