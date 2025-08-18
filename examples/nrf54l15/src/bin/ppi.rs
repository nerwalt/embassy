#![no_std]
#![no_main]

use core::future::pending;

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::gpiote::{self, InputChannel, InputChannelPolarity};
use embassy_nrf::ppi::Ppi;
use gpiote::{OutputChannel, OutputChannelPolarity};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    info!("Starting");

    // Connect a button in the PERI PD to an LED in the PERI PD via PPI. Becaause both are in
    // the same domain, a PPI bridge (PPIB) is not required.

    // Button 0 (P1_13) and GPIOTE20 are in the PERI PD
    let button0 = InputChannel::new(
        p.GPIOTE20_CH0,
        Input::new(p.P1_13, Pull::Up),
        InputChannelPolarity::HiToLo,
    );

    // LED 1 (P1_10) and GPIOTE20 are in the PERI PD
    let led1 = OutputChannel::new(
        p.GPIOTE20_CH3,
        Output::new(p.P1_10, Level::Low, OutputDrive::Standard),
        OutputChannelPolarity::Toggle,
    );

    let mut ppi = Ppi::new_one_to_one(p.PPI20_CH0, button0.event_in(), led1.task_out());
    ppi.enable();

    info!("Press button 0 to toggle LED 1");

    // Block forever so the above drivers don't get dropped
    pending::<()>().await;
}
