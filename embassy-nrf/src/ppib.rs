#![macro_use]

//! Programmable Peripheral Interconnect Bridge (PPIB) driver.
//!
//! TODO
//!

use embassy_hal_internal::{impl_peripheral, Peri, PeripheralType};

use crate::pac;
use crate::ppi::AnyConfigurableChannel;

#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// PPIB instance
pub enum PpibInstance {
    /// PPIB_00 instance
    Ppib00,
    /// PPIB_01 instance
    Ppib01,
    /// PPIB_10 instance
    Ppib10,
    /// PPIB_11 instance
    Ppib11,
    /// PPIB_20 instance
    Ppib20,
    /// PPIB_21 instance
    Ppib21,
    /// PPIB_22 instance
    Ppib22,
    /// PPIB_30 instance
    Ppib30,
}

impl From<pac::ppib::Ppib> for PpibInstance {
    fn from(value: pac::ppib::Ppib) -> Self {
        match value {
            pac::PPIB00 => Self::Ppib00,
            pac::PPIB01 => Self::Ppib01,
            pac::PPIB10 => Self::Ppib10,
            pac::PPIB11 => Self::Ppib11,
            pac::PPIB20 => Self::Ppib20,
            pac::PPIB21 => Self::Ppib21,
            pac::PPIB22 => Self::Ppib22,
            pac::PPIB30 => Self::Ppib30,
            _ => panic!("Unsupported PPIB instance"),
        }
    }
}

// Get the correct register block for the provided instance
pub(crate) fn regs(inst: PpibInstance) -> pac::ppib::Ppib {
    match inst {
        PpibInstance::Ppib00 => pac::PPIB00,
        PpibInstance::Ppib01 => pac::PPIB01,
        PpibInstance::Ppib10 => pac::PPIB10,
        PpibInstance::Ppib11 => pac::PPIB11,
        PpibInstance::Ppib20 => pac::PPIB20,
        PpibInstance::Ppib21 => pac::PPIB21,
        PpibInstance::Ppib22 => pac::PPIB22,
        PpibInstance::Ppib30 => pac::PPIB30,
    }
}

pub(crate) trait SealedChannel {}

/// Interface for PPI bridge channels.
#[allow(private_bounds)]
pub trait Channel: SealedChannel + PeripheralType + Sized + 'static {
    /// Returns the number of the channel
    fn number(&self) -> usize;
    /// Returns the instance of the PPIB controller
    fn inst(&self) -> PpibInstance;
}

/// The any channel can represent any static channel at runtime.
/// This can be used to have fewer generic parameters in some places.
pub struct AnyChannel {
    pub(crate) number: u8,
    pub(crate) inst: PpibInstance,
}

impl_peripheral!(AnyChannel);

impl SealedChannel for AnyChannel {}

impl Channel for AnyChannel {
    fn number(&self) -> usize {
        self.number as usize
    }

    fn inst(&self) -> PpibInstance {
        self.inst
    }
}

macro_rules! impl_ppib_channel {
    ($type:ident, $inst:expr, $number:expr) => {
        impl crate::ppib::SealedChannel for peripherals::$type {}
        impl crate::ppib::Channel for peripherals::$type {
            fn number(&self) -> usize {
                $number
            }
            fn inst(&self) -> crate::ppib::PpibInstance {
                $inst.into()
            }
        }
        impl From<peripherals::$type> for crate::ppib::AnyChannel {
            fn from(val: peripherals::$type) -> Self {
                Self {
                    number: crate::ppib::Channel::number(&val) as u8,
                    inst: crate::ppib::Channel::inst(&val),
                }
            }
        }
    };
}

/// PPI bridge driver.
pub struct PpiBridge<'d, C: Channel, D: Channel> {
    // Source PPIB channel
    source: Peri<'d, C>,
    // Source DPPIC channel
    send: AnyConfigurableChannel,
    // Sink PPIB channel
    sink: Peri<'d, D>,
    // Sink DPPIC channel
    receive: AnyConfigurableChannel,
}

impl<'d, C: Channel, D: Channel> PpiBridge<'d, C, D> {
    /// Create a new PPI bridge driver.
    pub fn new<S, R>(source: Peri<'d, C>, send: S, sink: Peri<'d, D>, receive: R) -> Self
    where
        S: Into<AnyConfigurableChannel>,
        R: Into<AnyConfigurableChannel>,
    {
        Self {
            source,
            send: send.into(),
            sink,
            receive: receive.into(),
        }
    }

    /// Enable this bridge.
    pub fn enable(&mut self) {
        // Configure and enable source
        let r = regs(self.source.inst());
        let idx = self.source.number();
        let subscribe = r.subscribe_send(idx);
        if subscribe.read().en() {
            panic!("PPIB send ch already in use");
        }
        subscribe.write(|w| {
            w.set_chidx(self.send.number as u8);
            w.set_en(true);
        });
        r.events_receive(idx).write(|w| *w = 0);

        // Configure and enable sink
        let r = regs(self.sink.inst());
        let idx = self.sink.number();
        let publish = r.publish_receive(idx);
        if publish.read().en() {
            panic!("PPIB receive ch already in use");
        }
        publish.write(|w| {
            w.set_chidx(self.receive.number as u8);
            w.set_en(true);
        });
        r.events_receive(idx).write(|w| *w = 0);
    }

    /// Disable this bridge.
    pub fn disable(&mut self) {
        // Disable source
        let r = regs(self.source.inst());
        let idx = self.source.number();
        r.subscribe_send(idx).write(|w| {
            w.set_en(false);
            w.set_chidx(0);
        });

        // Disable sink
        let r = regs(self.sink.inst());
        let idx = self.sink.number();
        r.publish_receive(idx).write(|w| {
            w.set_en(false);
            w.set_chidx(0);
        });
        r.events_receive(idx).write(|w| *w = 0);
    }
}

impl<C: Channel, D: Channel> Drop for PpiBridge<'_, C, D> {
    fn drop(&mut self) {
        self.disable();
    }
}
