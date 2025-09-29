//! Global real-time clock (nRF54L GRTC bring-up)

use crate::pac;
use crate::pac::grtc::vals;

/// Start GRTC and enable the SYSCOUNTER if not already running.
#[inline(always)]
pub fn start_grtc() {
    let grtc = pac::GRTC;

    // If the SYSCOUNTER is already running, nothing to do.
    if grtc.mode().read().syscounteren() {
        return;
    }

    // Select the LFCLK source before starting the counter so it stays in sync
    // with whatever LF clock the rest of the system uses.
    grtc.clkcfg().modify(|w| {
        w.set_clksel(vals::Clksel::SYSTEM_LFCLK);
    });

    // Make sure pending sync events are cleared so we can observe the first one.
    grtc.events_rtcomparesync().write(|w| *w = 0);

    // Start the low-frequency timer domain.
    grtc.tasks_start().write(|w| *w = 1);

    // Enable the SYSCOUNTER and keep it active while any CPU is running.
    grtc.mode().modify(|w| {
        w.set_autoen(vals::Autoen::CPU_ACTIVE);
        w.set_syscounteren(true);
    });

    // Request the counter to remain in the active state for all slots we can control.
    for idx in 0..4 {
        grtc.syscounter(idx).active().write(|w| w.set_active(true));
    }

    // Wait until the hardware latches the enable bit.
    while !grtc.mode().read().syscounteren() {
        core::hint::spin_loop();
    }

    // Ensure the counter is readable before returning.
    loop {
        let hi = grtc.syscounter(0).syscounterh().read();
        if hi.busy() == vals::Busy::READY {
            let _ = grtc.syscounter(0).syscounterl().read();
            break;
        }
        core::hint::spin_loop();
    }
}
