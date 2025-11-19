#![no_std]
#![no_main]

use core::future::pending;

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, OutputDrive, Pull};
use embassy_nrf::gpiote::{InputChannel, InputChannelPolarity, OutputChannel, OutputChannelPolarity};
use embassy_nrf::ppi::Ppi;
use embassy_nrf::ppib::PpiBridge;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    info!("Starting");

    // Connect a button in the LP PD to an LED in the PERI PD via PPI. Becaause we are crossing
    // power domains, we need to use PPI bridges. The LP and PERI domains are connected by PPIB22
    // and PPIB30.

    // Button 3 (P0_04) in LP PD
    let button_lp = InputChannel::new(
        p.GPIOTE30_CH0,
        p.P0_04, 
        Pull::Up,
        InputChannelPolarity::HiToLo,
    );

    // Connect the button event to the PPI controller. Don't connect any tasks, becasue we will
    // connect the PPI channel to the PPI bridge to forward the event PERI PD.
    let mut ppi_lp = Ppi::new_many_to_many(p.PPI30_CH0, [button_lp.event_in()], []);
    ppi_lp.enable();

    // LED 3 (P1_14)in PERI PD
    let led_peri = OutputChannel::new(
        p.GPIOTE20_CH0,
        p.P1_14, 
        Level::Low, 
        OutputDrive::Standard,
        OutputChannelPolarity::Toggle,
    );

    // Connect the LED task to the PPI controller. Don't connect an event, becasue we will connect
    // the PPI channel to the PPI bridge to receive an event from the LP PD.
    let mut ppi_peri = Ppi::new_many_to_many(p.PPI20_CH0, [], [led_peri.task_out()]);
    ppi_peri.enable();

    // Setup the PPI bridge
    let mut ppib = PpiBridge::new(p.PPIB30_CH0, ppi_lp.channel(), p.PPIB22_CH0, ppi_peri.channel());
    ppib.enable();

    info!("Press button 3 to toggle LED 3");

    pending::<()>().await;
}
