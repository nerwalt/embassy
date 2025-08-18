/// Peripheral Access Crate
#[allow(unused_imports)]
#[rustfmt::skip]
pub mod pac {
    pub use nrf_pac::*;

    #[cfg(feature = "_ns")]
    #[doc(no_inline)]
    pub use nrf_pac::{
        FICR_NS as FICR,
        DPPIC00_NS as DPPIC00,
        PPIB00_NS as PPIB00,
        PPIB01_NS as PPIB01,
        AAR00_NS as AAR00,
        CCM00_NS as CCM00,
        ECB00_NS as ECB00,
        SPIM00_NS as SPIM00,
        SPIS00_NS as SPIS00,
        UARTE00_NS as UARTE00,
        VPR00_NS as VPR00,
        P2_NS as P2,
        CTRLAP_NS as CTRLAP,
        TAD_NS as TAD,
        TIMER00_NS as TIMER00,
        DPPIC10_NS as DPPIC10,
        PPIB10_NS as PPIB10,
        PPIB11_NS as PPIB11,
        TIMER10_NS as TIMER10,
        RTC10_NS as RTC10,
        EGU10_NS as EGU10,
        RADIO_NS as RADIO,
        DPPIC20_NS as DPPIC20,
        PPIB20_NS as PPIB20,
        PPIB21_NS as PPIB21,
        PPIB22_NS as PPIB22,
        SPIM20_NS as SPIM20,
        SPIS20_NS as SPIS20,
        TWIM20_NS as TWIM20,
        TWIS20_NS as TWIS20,
        UARTE20_NS as UARTE20,
        SPIM21_NS as SPIM21,
        SPIS21_NS as SPIS21,
        TWIM21_NS as TWIM21,
        TWIS21_NS as TWIS21,
        UARTE21_NS as UARTE21,
        SPIM22_NS as SPIM22,
        SPIS22_NS as SPIS22,
        TWIM22_NS as TWIM22,
        TWIS22_NS as TWIS22,
        UARTE22_NS as UARTE22,
        EGU20_NS as EGU20,
        TIMER20_NS as TIMER20,
        TIMER21_NS as TIMER21,
        TIMER22_NS as TIMER22,
        TIMER23_NS as TIMER23,
        TIMER24_NS as TIMER24,
        MEMCONF_NS as MEMCONF,
        PDM20_NS as PDM20,
        PDM21_NS as PDM21,
        PWM20_NS as PWM20,
        PWM21_NS as PWM21,
        PWM22_NS as PWM22,
        SAADC_NS as SAADC,
        NFCT_NS as NFCT,
        TEMP_NS as TEMP,
        P1_NS as P1,
        GPIOTE20_NS as GPIOTE20,
        I2S20_NS as I2S20,
        QDEC20_NS as QDEC20,
        QDEC21_NS as QDEC21,
        GRTC_NS as GRTC,
        DPPIC30_NS as DPPIC30,
        PPIB30_NS as PPIB30,
        SPIM30_NS as SPIM30,
        SPIS30_NS as SPIS30,
        TWIM30_NS as TWIM30,
        TWIS30_NS as TWIS30,
        UARTE30_NS as UARTE30,
        RTC30_NS as RTC30,
        COMP_NS as COMP,
        LPCOMP_NS as LPCOMP,
        WDT31_NS as WDT31,
        P0_NS as P0,
        GPIOTE30_NS as GPIOTE30,
        CLOCK_NS as CLOCK,
        POWER_NS as POWER,
        RESET_NS as RESET,
        OSCILLATORS_NS as OSCILLATORS,
        REGULATORS_NS as REGULATORS,
        TPIU_NS as TPIU,
        ETM_NS as ETM,
    };

    #[cfg(feature = "_s")]
    #[doc(no_inline)]
    pub use nrf_pac::{
        SICR_S as SICR,
        ICACHEDATA_S as ICACHEDATA,
        ICACHEINFO_S as ICACHEINFO,
        SWI00_S as SWI00,
        SWI01_S as SWI01,
        SWI02_S as SWI02,
        SWI03_S as SWI03,
        SPU00_S as SPU00,
        MPC00_S as MPC00,
        DPPIC00_S as DPPIC00,
        PPIB00_S as PPIB00,
        PPIB01_S as PPIB01,
        KMU_S as KMU,
        AAR00_S as AAR00,
        CCM00_S as CCM00,
        ECB00_S as ECB00,
        CRACEN_S as CRACEN,
        SPIM00_S as SPIM00,
        SPIS00_S as SPIS00,
        UARTE00_S as UARTE00,
        GLITCHDET_S as GLITCHDET,
        RRAMC_S as RRAMC,
        VPR00_S as VPR00,
        P2_S as P2,
        CTRLAP_S as CTRLAP,
        TAD_S as TAD,
        TIMER00_S as TIMER00,
        SPU10_S as SPU10,
        DPPIC10_S as DPPIC10,
        PPIB10_S as PPIB10,
        PPIB11_S as PPIB11,
        TIMER10_S as TIMER10,
        RTC10_S as RTC10,
        EGU10_S as EGU10,
        RADIO_S as RADIO,
        SPU20_S as SPU20,
        DPPIC20_S as DPPIC20,
        PPIB20_S as PPIB20,
        PPIB21_S as PPIB21,
        PPIB22_S as PPIB22,
        SPIM20_S as SPIM20,
        SPIS20_S as SPIS20,
        TWIM20_S as TWIM20,
        TWIS20_S as TWIS20,
        UARTE20_S as UARTE20,
        SPIM21_S as SPIM21,
        SPIS21_S as SPIS21,
        TWIM21_S as TWIM21,
        TWIS21_S as TWIS21,
        UARTE21_S as UARTE21,
        SPIM22_S as SPIM22,
        SPIS22_S as SPIS22,
        TWIM22_S as TWIM22,
        TWIS22_S as TWIS22,
        UARTE22_S as UARTE22,
        EGU20_S as EGU20,
        TIMER20_S as TIMER20,
        TIMER21_S as TIMER21,
        TIMER22_S as TIMER22,
        TIMER23_S as TIMER23,
        TIMER24_S as TIMER24,
        MEMCONF_S as MEMCONF,
        PDM20_S as PDM20,
        PDM21_S as PDM21,
        PWM20_S as PWM20,
        PWM21_S as PWM21,
        PWM22_S as PWM22,
        SAADC_S as SAADC,
        NFCT_S as NFCT,
        TEMP_S as TEMP,
        P1_S as P1,
        GPIOTE20_S as GPIOTE20,
        TAMPC_S as TAMPC,
        I2S20_S as I2S20,
        QDEC20_S as QDEC20,
        QDEC21_S as QDEC21,
        GRTC_S as GRTC,
        SPU30_S as SPU30,
        DPPIC30_S as DPPIC30,
        PPIB30_S as PPIB30,
        SPIM30_S as SPIM30,
        SPIS30_S as SPIS30,
        TWIM30_S as TWIM30,
        TWIS30_S as TWIS30,
        UARTE30_S as UARTE30,
        RTC30_S as RTC30,
        COMP_S as COMP,
        LPCOMP_S as LPCOMP,
        WDT30_S as WDT30,
        WDT31_S as WDT31,
        P0_S as P0,
        GPIOTE30_S as GPIOTE30,
        CLOCK_S as CLOCK,
        POWER_S as POWER,
        RESET_S as RESET,
        OSCILLATORS_S as OSCILLATORS,
        REGULATORS_S as REGULATORS,
        CRACENCORE_S as CRACENCORE,
        CPUC_S as CPUC,
        ICACHE_S as ICACHE,
    };
}

/// The maximum buffer size that the EasyDMA can send/recv in one operation.
pub const EASY_DMA_SIZE: usize = (1 << 16) - 1;
pub const FORCE_COPY_BUFFER_SIZE: usize = 1024;

// 1.5 MB NVM
#[allow(unused)]
pub const FLASH_SIZE: usize = 1536 * 1024;

embassy_hal_internal::peripherals! {
    // GPIOTE
    GPIOTE20_CH0,
    GPIOTE20_CH1,
    GPIOTE20_CH2,
    GPIOTE20_CH3,
    GPIOTE20_CH4,
    GPIOTE20_CH5,
    GPIOTE20_CH6,
    GPIOTE20_CH7,

    GPIOTE30_CH0,
    GPIOTE30_CH1,
    GPIOTE30_CH2,
    GPIOTE30_CH3,

    // GPIO port P0
    P0_00,
    P0_01,
    P0_02,
    P0_03,
    P0_04,
    P0_05,
    P0_06,

    // GPIO port P1
    P1_00,
    P1_01,
    P1_02,
    P1_03,
    P1_04,
    P1_05,
    P1_06,
    P1_07,
    P1_08,
    P1_09,
    P1_10,
    P1_11,
    P1_12,
    P1_13,
    P1_14,
    P1_15,
    P1_16,

    // GPIO port P2
    P2_00,
    P2_01,
    P2_02,
    P2_03,
    P2_04,
    P2_05,
    P2_06,
    P2_07,
    P2_08,
    P2_09,
    P2_10,

    // PPI CHs
    PPI00_CH0,
    PPI00_CH1,
    PPI00_CH2,
    PPI00_CH3,
    PPI00_CH4,
    PPI00_CH5,
    PPI00_CH6,
    PPI00_CH7,

    PPI10_CH0,
    PPI10_CH1,
    PPI10_CH2,
    PPI10_CH3,
    PPI10_CH4,
    PPI10_CH5,
    PPI10_CH6,
    PPI10_CH7,
    PPI10_CH8,
    PPI10_CH9,
    PPI10_CH10,
    PPI10_CH11,
    PPI10_CH12,
    PPI10_CH13,
    PPI10_CH14,
    PPI10_CH15,
    PPI10_CH16,
    PPI10_CH17,
    PPI10_CH18,
    PPI10_CH19,
    PPI10_CH20,
    PPI10_CH21,
    PPI10_CH22,
    PPI10_CH23,

    PPI20_CH0,
    PPI20_CH1,
    PPI20_CH2,
    PPI20_CH3,
    PPI20_CH4,
    PPI20_CH5,
    PPI20_CH6,
    PPI20_CH7,
    PPI20_CH8,
    PPI20_CH9,
    PPI20_CH10,
    PPI20_CH11,
    PPI20_CH12,
    PPI20_CH13,
    PPI20_CH14,
    PPI20_CH15,

    PPI30_CH0,
    PPI30_CH1,
    PPI30_CH2,
    PPI30_CH3,

    // PPI GROUPs
    PPI00_GROUP0,
    PPI00_GROUP1,

    PPI10_GROUP0,
    PPI10_GROUP1,
    PPI10_GROUP2,
    PPI10_GROUP3,
    PPI10_GROUP4,
    PPI10_GROUP5,

    PPI20_GROUP0,
    PPI20_GROUP1,
    PPI20_GROUP2,
    PPI20_GROUP3,
    PPI20_GROUP4,
    PPI20_GROUP5,

    PPI30_GROUP0,
    PPI30_GROUP1,

    // PPI BRIDGEs
    // PPIB00 channels
    PPIB00_CH0,
    PPIB00_CH1,
    PPIB00_CH2,
    PPIB00_CH3,
    PPIB00_CH4,
    PPIB00_CH5,
    PPIB00_CH6,
    PPIB00_CH7,
    PPIB00_CH8,
    PPIB00_CH9,
    PPIB00_CH10,
    PPIB00_CH11,
    PPIB00_CH12,
    PPIB00_CH13,
    PPIB00_CH14,
    PPIB00_CH15,
    PPIB00_CH16,
    PPIB00_CH17,
    PPIB00_CH18,
    PPIB00_CH19,
    PPIB00_CH20,
    PPIB00_CH21,
    PPIB00_CH22,
    PPIB00_CH23,
    PPIB00_CH24,
    PPIB00_CH25,
    PPIB00_CH26,
    PPIB00_CH27,
    PPIB00_CH28,
    PPIB00_CH29,
    PPIB00_CH30,
    PPIB00_CH31,

    // PPIB01 channels
    PPIB01_CH0,
    PPIB01_CH1,
    PPIB01_CH2,
    PPIB01_CH3,
    PPIB01_CH4,
    PPIB01_CH5,
    PPIB01_CH6,
    PPIB01_CH7,
    PPIB01_CH8,
    PPIB01_CH9,
    PPIB01_CH10,
    PPIB01_CH11,
    PPIB01_CH12,
    PPIB01_CH13,
    PPIB01_CH14,
    PPIB01_CH15,
    PPIB01_CH16,
    PPIB01_CH17,
    PPIB01_CH18,
    PPIB01_CH19,
    PPIB01_CH20,
    PPIB01_CH21,
    PPIB01_CH22,
    PPIB01_CH23,
    PPIB01_CH24,
    PPIB01_CH25,
    PPIB01_CH26,
    PPIB01_CH27,
    PPIB01_CH28,
    PPIB01_CH29,
    PPIB01_CH30,
    PPIB01_CH31,

    // PPIB10 channels
    PPIB10_CH0,
    PPIB10_CH1,
    PPIB10_CH2,
    PPIB10_CH3,
    PPIB10_CH4,
    PPIB10_CH5,
    PPIB10_CH6,
    PPIB10_CH7,
    PPIB10_CH8,
    PPIB10_CH9,
    PPIB10_CH10,
    PPIB10_CH11,
    PPIB10_CH12,
    PPIB10_CH13,
    PPIB10_CH14,
    PPIB10_CH15,
    PPIB10_CH16,
    PPIB10_CH17,
    PPIB10_CH18,
    PPIB10_CH19,
    PPIB10_CH20,
    PPIB10_CH21,
    PPIB10_CH22,
    PPIB10_CH23,
    PPIB10_CH24,
    PPIB10_CH25,
    PPIB10_CH26,
    PPIB10_CH27,
    PPIB10_CH28,
    PPIB10_CH29,
    PPIB10_CH30,
    PPIB10_CH31,

    // PPIB11 channels
    PPIB11_CH0,
    PPIB11_CH1,
    PPIB11_CH2,
    PPIB11_CH3,
    PPIB11_CH4,
    PPIB11_CH5,
    PPIB11_CH6,
    PPIB11_CH7,
    PPIB11_CH8,
    PPIB11_CH9,
    PPIB11_CH10,
    PPIB11_CH11,
    PPIB11_CH12,
    PPIB11_CH13,
    PPIB11_CH14,
    PPIB11_CH15,
    PPIB11_CH16,
    PPIB11_CH17,
    PPIB11_CH18,
    PPIB11_CH19,
    PPIB11_CH20,
    PPIB11_CH21,
    PPIB11_CH22,
    PPIB11_CH23,
    PPIB11_CH24,
    PPIB11_CH25,
    PPIB11_CH26,
    PPIB11_CH27,
    PPIB11_CH28,
    PPIB11_CH29,
    PPIB11_CH30,
    PPIB11_CH31,

    // PPIB20 channels
    PPIB20_CH0,
    PPIB20_CH1,
    PPIB20_CH2,
    PPIB20_CH3,
    PPIB20_CH4,
    PPIB20_CH5,
    PPIB20_CH6,
    PPIB20_CH7,
    PPIB20_CH8,
    PPIB20_CH9,
    PPIB20_CH10,
    PPIB20_CH11,
    PPIB20_CH12,
    PPIB20_CH13,
    PPIB20_CH14,
    PPIB20_CH15,
    PPIB20_CH16,
    PPIB20_CH17,
    PPIB20_CH18,
    PPIB20_CH19,
    PPIB20_CH20,
    PPIB20_CH21,
    PPIB20_CH22,
    PPIB20_CH23,
    PPIB20_CH24,
    PPIB20_CH25,
    PPIB20_CH26,
    PPIB20_CH27,
    PPIB20_CH28,
    PPIB20_CH29,
    PPIB20_CH30,
    PPIB20_CH31,

    // PPIB21 channels
    PPIB21_CH0,
    PPIB21_CH1,
    PPIB21_CH2,
    PPIB21_CH3,
    PPIB21_CH4,
    PPIB21_CH5,
    PPIB21_CH6,
    PPIB21_CH7,
    PPIB21_CH8,
    PPIB21_CH9,
    PPIB21_CH10,
    PPIB21_CH11,
    PPIB21_CH12,
    PPIB21_CH13,
    PPIB21_CH14,
    PPIB21_CH15,
    PPIB21_CH16,
    PPIB21_CH17,
    PPIB21_CH18,
    PPIB21_CH19,
    PPIB21_CH20,
    PPIB21_CH21,
    PPIB21_CH22,
    PPIB21_CH23,
    PPIB21_CH24,
    PPIB21_CH25,
    PPIB21_CH26,
    PPIB21_CH27,
    PPIB21_CH28,
    PPIB21_CH29,
    PPIB21_CH30,
    PPIB21_CH31,

    // PPIB22 channels
    PPIB22_CH0,
    PPIB22_CH1,
    PPIB22_CH2,
    PPIB22_CH3,
    PPIB22_CH4,
    PPIB22_CH5,
    PPIB22_CH6,
    PPIB22_CH7,
    PPIB22_CH8,
    PPIB22_CH9,
    PPIB22_CH10,
    PPIB22_CH11,
    PPIB22_CH12,
    PPIB22_CH13,
    PPIB22_CH14,
    PPIB22_CH15,
    PPIB22_CH16,
    PPIB22_CH17,
    PPIB22_CH18,
    PPIB22_CH19,
    PPIB22_CH20,
    PPIB22_CH21,
    PPIB22_CH22,
    PPIB22_CH23,
    PPIB22_CH24,
    PPIB22_CH25,
    PPIB22_CH26,
    PPIB22_CH27,
    PPIB22_CH28,
    PPIB22_CH29,
    PPIB22_CH30,
    PPIB22_CH31,

    // PPIB30 channels
    PPIB30_CH0,
    PPIB30_CH1,
    PPIB30_CH2,
    PPIB30_CH3,
    PPIB30_CH4,
    PPIB30_CH5,
    PPIB30_CH6,
    PPIB30_CH7,
    PPIB30_CH8,
    PPIB30_CH9,
    PPIB30_CH10,
    PPIB30_CH11,
    PPIB30_CH12,
    PPIB30_CH13,
    PPIB30_CH14,
    PPIB30_CH15,
    PPIB30_CH16,
    PPIB30_CH17,
    PPIB30_CH18,
    PPIB30_CH19,
    PPIB30_CH20,
    PPIB30_CH21,
    PPIB30_CH22,
    PPIB30_CH23,
    PPIB30_CH24,
    PPIB30_CH25,
    PPIB30_CH26,
    PPIB30_CH27,
    PPIB30_CH28,
    PPIB30_CH29,
    PPIB30_CH30,
    PPIB30_CH31,

    // PWM
    PWM20,
    PWM21,
    PWM22,

    // Radio
    RADIO,

    // RTC
    GRTC,
    RTC10,
    RTC30,

    #[cfg(feature = "_s")]
    // RRAMC
    RRAMC,

    // SAADC
    SAADC,

    // TEMP
    TEMP,

    // TIMERs
    TIMER00,

    // Dedicated to radio. Implemented for nrf-sdc
    TIMER10,

    TIMER20,
    TIMER21,
    TIMER22,
    TIMER23,
    TIMER24,

    // TWI/SPI
    SPI00,
    TWISPI20,
    TWISPI21,
    TWISPI22,
    TWISPI30,

    // WDT
    #[cfg(feature = "_ns")]
    WDT,
    #[cfg(feature = "_s")]
    WDT0,
    #[cfg(feature = "_s")]
    WDT1,
}

// GPIO port P0
impl_pin!(P0_00, 0, 0);
impl_pin!(P0_01, 0, 1);
impl_pin!(P0_02, 0, 2);
impl_pin!(P0_03, 0, 3);
impl_pin!(P0_04, 0, 4);
impl_pin!(P0_05, 0, 5);
impl_pin!(P0_06, 0, 6);

// GPIO port P1
impl_pin!(P1_00, 1, 0);
impl_pin!(P1_01, 1, 1);
impl_pin!(P1_02, 1, 2);
impl_pin!(P1_03, 1, 3);
impl_pin!(P1_04, 1, 4);
impl_pin!(P1_05, 1, 5);
impl_pin!(P1_06, 1, 6);
impl_pin!(P1_07, 1, 7);
impl_pin!(P1_08, 1, 8);
impl_pin!(P1_09, 1, 9);
impl_pin!(P1_10, 1, 10);
impl_pin!(P1_11, 1, 11);
impl_pin!(P1_12, 1, 12);
impl_pin!(P1_13, 1, 13);
impl_pin!(P1_14, 1, 14);
impl_pin!(P1_15, 1, 15);
impl_pin!(P1_16, 1, 16);

// GPIO port P2
impl_pin!(P2_00, 2, 0);
impl_pin!(P2_01, 2, 1);
impl_pin!(P2_02, 2, 2);
impl_pin!(P2_03, 2, 3);
impl_pin!(P2_04, 2, 4);
impl_pin!(P2_05, 2, 5);
impl_pin!(P2_06, 2, 6);
impl_pin!(P2_07, 2, 7);
impl_pin!(P2_08, 2, 8);
impl_pin!(P2_09, 2, 9);
impl_pin!(P2_10, 2, 10);

// DPPI00 channels
impl_ppi_channel!(PPI00_CH0, pac::DPPIC00, 0 => configurable);
impl_ppi_channel!(PPI00_CH1, pac::DPPIC00, 1 => configurable);
impl_ppi_channel!(PPI00_CH2, pac::DPPIC00, 2 => configurable);
impl_ppi_channel!(PPI00_CH3, pac::DPPIC00, 3 => configurable);
impl_ppi_channel!(PPI00_CH4, pac::DPPIC00, 4 => configurable);
impl_ppi_channel!(PPI00_CH5, pac::DPPIC00, 5 => configurable);
impl_ppi_channel!(PPI00_CH6, pac::DPPIC00, 6 => configurable);
impl_ppi_channel!(PPI00_CH7, pac::DPPIC00, 7 => configurable);

// DPPI10 channels
impl_ppi_channel!(PPI10_CH0, pac::DPPIC10, 0 => configurable);
impl_ppi_channel!(PPI10_CH1, pac::DPPIC10, 1 => configurable);
impl_ppi_channel!(PPI10_CH2, pac::DPPIC10, 2 => configurable);
impl_ppi_channel!(PPI10_CH3, pac::DPPIC10, 3 => configurable);
impl_ppi_channel!(PPI10_CH4, pac::DPPIC10, 4 => configurable);
impl_ppi_channel!(PPI10_CH5, pac::DPPIC10, 5 => configurable);
impl_ppi_channel!(PPI10_CH6, pac::DPPIC10, 6 => configurable);
impl_ppi_channel!(PPI10_CH7, pac::DPPIC10, 7 => configurable);
impl_ppi_channel!(PPI10_CH8, pac::DPPIC10, 8 => configurable);
impl_ppi_channel!(PPI10_CH9, pac::DPPIC10, 9 => configurable);
impl_ppi_channel!(PPI10_CH10, pac::DPPIC10, 10 => configurable);
impl_ppi_channel!(PPI10_CH11, pac::DPPIC10, 11 => configurable);
impl_ppi_channel!(PPI10_CH12, pac::DPPIC10, 12 => configurable);
impl_ppi_channel!(PPI10_CH13, pac::DPPIC10, 13 => configurable);
impl_ppi_channel!(PPI10_CH14, pac::DPPIC10, 14 => configurable);
impl_ppi_channel!(PPI10_CH15, pac::DPPIC10, 15 => configurable);
impl_ppi_channel!(PPI10_CH16, pac::DPPIC10, 16 => configurable);
impl_ppi_channel!(PPI10_CH17, pac::DPPIC10, 17 => configurable);
impl_ppi_channel!(PPI10_CH18, pac::DPPIC10, 18 => configurable);
impl_ppi_channel!(PPI10_CH19, pac::DPPIC10, 19 => configurable);
impl_ppi_channel!(PPI10_CH20, pac::DPPIC10, 20 => configurable);
impl_ppi_channel!(PPI10_CH21, pac::DPPIC10, 21 => configurable);
impl_ppi_channel!(PPI10_CH22, pac::DPPIC10, 22 => configurable);
impl_ppi_channel!(PPI10_CH23, pac::DPPIC10, 23 => configurable);

// DPPI20 channels
impl_ppi_channel!(PPI20_CH0, pac::DPPIC20, 0 => configurable);
impl_ppi_channel!(PPI20_CH1, pac::DPPIC20, 1 => configurable);
impl_ppi_channel!(PPI20_CH2, pac::DPPIC20, 2 => configurable);
impl_ppi_channel!(PPI20_CH3, pac::DPPIC20, 3 => configurable);
impl_ppi_channel!(PPI20_CH4, pac::DPPIC20, 4 => configurable);
impl_ppi_channel!(PPI20_CH5, pac::DPPIC20, 5 => configurable);
impl_ppi_channel!(PPI20_CH6, pac::DPPIC20, 6 => configurable);
impl_ppi_channel!(PPI20_CH7, pac::DPPIC20, 7 => configurable);
impl_ppi_channel!(PPI20_CH8, pac::DPPIC20, 8 => configurable);
impl_ppi_channel!(PPI20_CH9, pac::DPPIC20, 9 => configurable);
impl_ppi_channel!(PPI20_CH10, pac::DPPIC20, 10 => configurable);
impl_ppi_channel!(PPI20_CH11, pac::DPPIC20, 11 => configurable);
impl_ppi_channel!(PPI20_CH12, pac::DPPIC20, 12 => configurable);
impl_ppi_channel!(PPI20_CH13, pac::DPPIC20, 13 => configurable);
impl_ppi_channel!(PPI20_CH14, pac::DPPIC20, 14 => configurable);
impl_ppi_channel!(PPI20_CH15, pac::DPPIC20, 15 => configurable);

// DPPI30 channels
impl_ppi_channel!(PPI30_CH0, pac::DPPIC30, 0 => configurable);
impl_ppi_channel!(PPI30_CH1, pac::DPPIC30, 1 => configurable);
impl_ppi_channel!(PPI30_CH2, pac::DPPIC30, 2 => configurable);
impl_ppi_channel!(PPI30_CH3, pac::DPPIC30, 3 => configurable);

// DPPI00 groups
impl_ppi_group!(PPI00_GROUP0, pac::DPPIC00, 0);
impl_ppi_group!(PPI00_GROUP1, pac::DPPIC00, 1);

// DPPI10 groups
impl_ppi_group!(PPI10_GROUP0, pac::DPPIC20, 0);
impl_ppi_group!(PPI10_GROUP1, pac::DPPIC20, 1);
impl_ppi_group!(PPI10_GROUP2, pac::DPPIC20, 2);
impl_ppi_group!(PPI10_GROUP3, pac::DPPIC20, 3);
impl_ppi_group!(PPI10_GROUP4, pac::DPPIC20, 4);
impl_ppi_group!(PPI10_GROUP5, pac::DPPIC20, 5);

// DPPI20 groups
impl_ppi_group!(PPI20_GROUP0, pac::DPPIC20, 0);
impl_ppi_group!(PPI20_GROUP1, pac::DPPIC20, 1);
impl_ppi_group!(PPI20_GROUP2, pac::DPPIC20, 2);
impl_ppi_group!(PPI20_GROUP3, pac::DPPIC20, 3);
impl_ppi_group!(PPI20_GROUP4, pac::DPPIC20, 4);
impl_ppi_group!(PPI20_GROUP5, pac::DPPIC20, 5);

// DPPI30 groups
impl_ppi_group!(PPI30_GROUP0, pac::DPPIC30, 0);
impl_ppi_group!(PPI30_GROUP1, pac::DPPIC30, 1);

// PPI bridge channels
// PPIB00 channels
impl_ppib_channel!(PPIB00_CH0, pac::PPIB00, 0);
impl_ppib_channel!(PPIB00_CH1, pac::PPIB00, 1);
impl_ppib_channel!(PPIB00_CH2, pac::PPIB00, 2);
impl_ppib_channel!(PPIB00_CH3, pac::PPIB00, 3);
impl_ppib_channel!(PPIB00_CH4, pac::PPIB00, 4);
impl_ppib_channel!(PPIB00_CH5, pac::PPIB00, 5);
impl_ppib_channel!(PPIB00_CH6, pac::PPIB00, 6);
impl_ppib_channel!(PPIB00_CH7, pac::PPIB00, 7);
impl_ppib_channel!(PPIB00_CH8, pac::PPIB00, 8);
impl_ppib_channel!(PPIB00_CH9, pac::PPIB00, 9);
impl_ppib_channel!(PPIB00_CH10, pac::PPIB00, 10);
impl_ppib_channel!(PPIB00_CH11, pac::PPIB00, 11);
impl_ppib_channel!(PPIB00_CH12, pac::PPIB00, 12);
impl_ppib_channel!(PPIB00_CH13, pac::PPIB00, 13);
impl_ppib_channel!(PPIB00_CH14, pac::PPIB00, 14);
impl_ppib_channel!(PPIB00_CH15, pac::PPIB00, 15);
impl_ppib_channel!(PPIB00_CH16, pac::PPIB00, 16);
impl_ppib_channel!(PPIB00_CH17, pac::PPIB00, 17);
impl_ppib_channel!(PPIB00_CH18, pac::PPIB00, 18);
impl_ppib_channel!(PPIB00_CH19, pac::PPIB00, 19);
impl_ppib_channel!(PPIB00_CH20, pac::PPIB00, 20);
impl_ppib_channel!(PPIB00_CH21, pac::PPIB00, 21);
impl_ppib_channel!(PPIB00_CH22, pac::PPIB00, 22);
impl_ppib_channel!(PPIB00_CH23, pac::PPIB00, 23);
impl_ppib_channel!(PPIB00_CH24, pac::PPIB00, 24);
impl_ppib_channel!(PPIB00_CH25, pac::PPIB00, 25);
impl_ppib_channel!(PPIB00_CH26, pac::PPIB00, 26);
impl_ppib_channel!(PPIB00_CH27, pac::PPIB00, 27);
impl_ppib_channel!(PPIB00_CH28, pac::PPIB00, 28);
impl_ppib_channel!(PPIB00_CH29, pac::PPIB00, 29);
impl_ppib_channel!(PPIB00_CH30, pac::PPIB00, 30);
impl_ppib_channel!(PPIB00_CH31, pac::PPIB00, 31);

// PPIB01 channels
impl_ppib_channel!(PPIB01_CH0, pac::PPIB01, 0);
impl_ppib_channel!(PPIB01_CH1, pac::PPIB01, 1);
impl_ppib_channel!(PPIB01_CH2, pac::PPIB01, 2);
impl_ppib_channel!(PPIB01_CH3, pac::PPIB01, 3);
impl_ppib_channel!(PPIB01_CH4, pac::PPIB01, 4);
impl_ppib_channel!(PPIB01_CH5, pac::PPIB01, 5);
impl_ppib_channel!(PPIB01_CH6, pac::PPIB01, 6);
impl_ppib_channel!(PPIB01_CH7, pac::PPIB01, 7);
impl_ppib_channel!(PPIB01_CH8, pac::PPIB01, 8);
impl_ppib_channel!(PPIB01_CH9, pac::PPIB01, 9);
impl_ppib_channel!(PPIB01_CH10, pac::PPIB01, 10);
impl_ppib_channel!(PPIB01_CH11, pac::PPIB01, 11);
impl_ppib_channel!(PPIB01_CH12, pac::PPIB01, 12);
impl_ppib_channel!(PPIB01_CH13, pac::PPIB01, 13);
impl_ppib_channel!(PPIB01_CH14, pac::PPIB01, 14);
impl_ppib_channel!(PPIB01_CH15, pac::PPIB01, 15);
impl_ppib_channel!(PPIB01_CH16, pac::PPIB01, 16);
impl_ppib_channel!(PPIB01_CH17, pac::PPIB01, 17);
impl_ppib_channel!(PPIB01_CH18, pac::PPIB01, 18);
impl_ppib_channel!(PPIB01_CH19, pac::PPIB01, 19);
impl_ppib_channel!(PPIB01_CH20, pac::PPIB01, 20);
impl_ppib_channel!(PPIB01_CH21, pac::PPIB01, 21);
impl_ppib_channel!(PPIB01_CH22, pac::PPIB01, 22);
impl_ppib_channel!(PPIB01_CH23, pac::PPIB01, 23);
impl_ppib_channel!(PPIB01_CH24, pac::PPIB01, 24);
impl_ppib_channel!(PPIB01_CH25, pac::PPIB01, 25);
impl_ppib_channel!(PPIB01_CH26, pac::PPIB01, 26);
impl_ppib_channel!(PPIB01_CH27, pac::PPIB01, 27);
impl_ppib_channel!(PPIB01_CH28, pac::PPIB01, 28);
impl_ppib_channel!(PPIB01_CH29, pac::PPIB01, 29);
impl_ppib_channel!(PPIB01_CH30, pac::PPIB01, 30);
impl_ppib_channel!(PPIB01_CH31, pac::PPIB01, 31);

// PPIB10 channels
impl_ppib_channel!(PPIB10_CH0, pac::PPIB10, 0);
impl_ppib_channel!(PPIB10_CH1, pac::PPIB10, 1);
impl_ppib_channel!(PPIB10_CH2, pac::PPIB10, 2);
impl_ppib_channel!(PPIB10_CH3, pac::PPIB10, 3);
impl_ppib_channel!(PPIB10_CH4, pac::PPIB10, 4);
impl_ppib_channel!(PPIB10_CH5, pac::PPIB10, 5);
impl_ppib_channel!(PPIB10_CH6, pac::PPIB10, 6);
impl_ppib_channel!(PPIB10_CH7, pac::PPIB10, 7);
impl_ppib_channel!(PPIB10_CH8, pac::PPIB10, 8);
impl_ppib_channel!(PPIB10_CH9, pac::PPIB10, 9);
impl_ppib_channel!(PPIB10_CH10, pac::PPIB10, 10);
impl_ppib_channel!(PPIB10_CH11, pac::PPIB10, 11);
impl_ppib_channel!(PPIB10_CH12, pac::PPIB10, 12);
impl_ppib_channel!(PPIB10_CH13, pac::PPIB10, 13);
impl_ppib_channel!(PPIB10_CH14, pac::PPIB10, 14);
impl_ppib_channel!(PPIB10_CH15, pac::PPIB10, 15);
impl_ppib_channel!(PPIB10_CH16, pac::PPIB10, 16);
impl_ppib_channel!(PPIB10_CH17, pac::PPIB10, 17);
impl_ppib_channel!(PPIB10_CH18, pac::PPIB10, 18);
impl_ppib_channel!(PPIB10_CH19, pac::PPIB10, 19);
impl_ppib_channel!(PPIB10_CH20, pac::PPIB10, 20);
impl_ppib_channel!(PPIB10_CH21, pac::PPIB10, 21);
impl_ppib_channel!(PPIB10_CH22, pac::PPIB10, 22);
impl_ppib_channel!(PPIB10_CH23, pac::PPIB10, 23);
impl_ppib_channel!(PPIB10_CH24, pac::PPIB10, 24);
impl_ppib_channel!(PPIB10_CH25, pac::PPIB10, 25);
impl_ppib_channel!(PPIB10_CH26, pac::PPIB10, 26);
impl_ppib_channel!(PPIB10_CH27, pac::PPIB10, 27);
impl_ppib_channel!(PPIB10_CH28, pac::PPIB10, 28);
impl_ppib_channel!(PPIB10_CH29, pac::PPIB10, 29);
impl_ppib_channel!(PPIB10_CH30, pac::PPIB10, 30);
impl_ppib_channel!(PPIB10_CH31, pac::PPIB10, 31);

// PPIB11 channels
impl_ppib_channel!(PPIB11_CH0, pac::PPIB11, 0);
impl_ppib_channel!(PPIB11_CH1, pac::PPIB11, 1);
impl_ppib_channel!(PPIB11_CH2, pac::PPIB11, 2);
impl_ppib_channel!(PPIB11_CH3, pac::PPIB11, 3);
impl_ppib_channel!(PPIB11_CH4, pac::PPIB11, 4);
impl_ppib_channel!(PPIB11_CH5, pac::PPIB11, 5);
impl_ppib_channel!(PPIB11_CH6, pac::PPIB11, 6);
impl_ppib_channel!(PPIB11_CH7, pac::PPIB11, 7);
impl_ppib_channel!(PPIB11_CH8, pac::PPIB11, 8);
impl_ppib_channel!(PPIB11_CH9, pac::PPIB11, 9);
impl_ppib_channel!(PPIB11_CH10, pac::PPIB11, 10);
impl_ppib_channel!(PPIB11_CH11, pac::PPIB11, 11);
impl_ppib_channel!(PPIB11_CH12, pac::PPIB11, 12);
impl_ppib_channel!(PPIB11_CH13, pac::PPIB11, 13);
impl_ppib_channel!(PPIB11_CH14, pac::PPIB11, 14);
impl_ppib_channel!(PPIB11_CH15, pac::PPIB11, 15);
impl_ppib_channel!(PPIB11_CH16, pac::PPIB11, 16);
impl_ppib_channel!(PPIB11_CH17, pac::PPIB11, 17);
impl_ppib_channel!(PPIB11_CH18, pac::PPIB11, 18);
impl_ppib_channel!(PPIB11_CH19, pac::PPIB11, 19);
impl_ppib_channel!(PPIB11_CH20, pac::PPIB11, 20);
impl_ppib_channel!(PPIB11_CH21, pac::PPIB11, 21);
impl_ppib_channel!(PPIB11_CH22, pac::PPIB11, 22);
impl_ppib_channel!(PPIB11_CH23, pac::PPIB11, 23);
impl_ppib_channel!(PPIB11_CH24, pac::PPIB11, 24);
impl_ppib_channel!(PPIB11_CH25, pac::PPIB11, 25);
impl_ppib_channel!(PPIB11_CH26, pac::PPIB11, 26);
impl_ppib_channel!(PPIB11_CH27, pac::PPIB11, 27);
impl_ppib_channel!(PPIB11_CH28, pac::PPIB11, 28);
impl_ppib_channel!(PPIB11_CH29, pac::PPIB11, 29);
impl_ppib_channel!(PPIB11_CH30, pac::PPIB11, 30);
impl_ppib_channel!(PPIB11_CH31, pac::PPIB11, 31);

// PPIB20 channels
impl_ppib_channel!(PPIB20_CH0, pac::PPIB20, 0);
impl_ppib_channel!(PPIB20_CH1, pac::PPIB20, 1);
impl_ppib_channel!(PPIB20_CH2, pac::PPIB20, 2);
impl_ppib_channel!(PPIB20_CH3, pac::PPIB20, 3);
impl_ppib_channel!(PPIB20_CH4, pac::PPIB20, 4);
impl_ppib_channel!(PPIB20_CH5, pac::PPIB20, 5);
impl_ppib_channel!(PPIB20_CH6, pac::PPIB20, 6);
impl_ppib_channel!(PPIB20_CH7, pac::PPIB20, 7);
impl_ppib_channel!(PPIB20_CH8, pac::PPIB20, 8);
impl_ppib_channel!(PPIB20_CH9, pac::PPIB20, 9);
impl_ppib_channel!(PPIB20_CH10, pac::PPIB20, 10);
impl_ppib_channel!(PPIB20_CH11, pac::PPIB20, 11);
impl_ppib_channel!(PPIB20_CH12, pac::PPIB20, 12);
impl_ppib_channel!(PPIB20_CH13, pac::PPIB20, 13);
impl_ppib_channel!(PPIB20_CH14, pac::PPIB20, 14);
impl_ppib_channel!(PPIB20_CH15, pac::PPIB20, 15);
impl_ppib_channel!(PPIB20_CH16, pac::PPIB20, 16);
impl_ppib_channel!(PPIB20_CH17, pac::PPIB20, 17);
impl_ppib_channel!(PPIB20_CH18, pac::PPIB20, 18);
impl_ppib_channel!(PPIB20_CH19, pac::PPIB20, 19);
impl_ppib_channel!(PPIB20_CH20, pac::PPIB20, 20);
impl_ppib_channel!(PPIB20_CH21, pac::PPIB20, 21);
impl_ppib_channel!(PPIB20_CH22, pac::PPIB20, 22);
impl_ppib_channel!(PPIB20_CH23, pac::PPIB20, 23);
impl_ppib_channel!(PPIB20_CH24, pac::PPIB20, 24);
impl_ppib_channel!(PPIB20_CH25, pac::PPIB20, 25);
impl_ppib_channel!(PPIB20_CH26, pac::PPIB20, 26);
impl_ppib_channel!(PPIB20_CH27, pac::PPIB20, 27);
impl_ppib_channel!(PPIB20_CH28, pac::PPIB20, 28);
impl_ppib_channel!(PPIB20_CH29, pac::PPIB20, 29);
impl_ppib_channel!(PPIB20_CH30, pac::PPIB20, 30);
impl_ppib_channel!(PPIB20_CH31, pac::PPIB20, 31);

// PPIB21 channels
impl_ppib_channel!(PPIB21_CH0, pac::PPIB21, 0);
impl_ppib_channel!(PPIB21_CH1, pac::PPIB21, 1);
impl_ppib_channel!(PPIB21_CH2, pac::PPIB21, 2);
impl_ppib_channel!(PPIB21_CH3, pac::PPIB21, 3);
impl_ppib_channel!(PPIB21_CH4, pac::PPIB21, 4);
impl_ppib_channel!(PPIB21_CH5, pac::PPIB21, 5);
impl_ppib_channel!(PPIB21_CH6, pac::PPIB21, 6);
impl_ppib_channel!(PPIB21_CH7, pac::PPIB21, 7);
impl_ppib_channel!(PPIB21_CH8, pac::PPIB21, 8);
impl_ppib_channel!(PPIB21_CH9, pac::PPIB21, 9);
impl_ppib_channel!(PPIB21_CH10, pac::PPIB21, 10);
impl_ppib_channel!(PPIB21_CH11, pac::PPIB21, 11);
impl_ppib_channel!(PPIB21_CH12, pac::PPIB21, 12);
impl_ppib_channel!(PPIB21_CH13, pac::PPIB21, 13);
impl_ppib_channel!(PPIB21_CH14, pac::PPIB21, 14);
impl_ppib_channel!(PPIB21_CH15, pac::PPIB21, 15);
impl_ppib_channel!(PPIB21_CH16, pac::PPIB21, 16);
impl_ppib_channel!(PPIB21_CH17, pac::PPIB21, 17);
impl_ppib_channel!(PPIB21_CH18, pac::PPIB21, 18);
impl_ppib_channel!(PPIB21_CH19, pac::PPIB21, 19);
impl_ppib_channel!(PPIB21_CH20, pac::PPIB21, 20);
impl_ppib_channel!(PPIB21_CH21, pac::PPIB21, 21);
impl_ppib_channel!(PPIB21_CH22, pac::PPIB21, 22);
impl_ppib_channel!(PPIB21_CH23, pac::PPIB21, 23);
impl_ppib_channel!(PPIB21_CH24, pac::PPIB21, 24);
impl_ppib_channel!(PPIB21_CH25, pac::PPIB21, 25);
impl_ppib_channel!(PPIB21_CH26, pac::PPIB21, 26);
impl_ppib_channel!(PPIB21_CH27, pac::PPIB21, 27);
impl_ppib_channel!(PPIB21_CH28, pac::PPIB21, 28);
impl_ppib_channel!(PPIB21_CH29, pac::PPIB21, 29);
impl_ppib_channel!(PPIB21_CH30, pac::PPIB21, 30);
impl_ppib_channel!(PPIB21_CH31, pac::PPIB21, 31);

// PPIB22 channels
impl_ppib_channel!(PPIB22_CH0, pac::PPIB22, 0);
impl_ppib_channel!(PPIB22_CH1, pac::PPIB22, 1);
impl_ppib_channel!(PPIB22_CH2, pac::PPIB22, 2);
impl_ppib_channel!(PPIB22_CH3, pac::PPIB22, 3);
impl_ppib_channel!(PPIB22_CH4, pac::PPIB22, 4);
impl_ppib_channel!(PPIB22_CH5, pac::PPIB22, 5);
impl_ppib_channel!(PPIB22_CH6, pac::PPIB22, 6);
impl_ppib_channel!(PPIB22_CH7, pac::PPIB22, 7);
impl_ppib_channel!(PPIB22_CH8, pac::PPIB22, 8);
impl_ppib_channel!(PPIB22_CH9, pac::PPIB22, 9);
impl_ppib_channel!(PPIB22_CH10, pac::PPIB22, 10);
impl_ppib_channel!(PPIB22_CH11, pac::PPIB22, 11);
impl_ppib_channel!(PPIB22_CH12, pac::PPIB22, 12);
impl_ppib_channel!(PPIB22_CH13, pac::PPIB22, 13);
impl_ppib_channel!(PPIB22_CH14, pac::PPIB22, 14);
impl_ppib_channel!(PPIB22_CH15, pac::PPIB22, 15);
impl_ppib_channel!(PPIB22_CH16, pac::PPIB22, 16);
impl_ppib_channel!(PPIB22_CH17, pac::PPIB22, 17);
impl_ppib_channel!(PPIB22_CH18, pac::PPIB22, 18);
impl_ppib_channel!(PPIB22_CH19, pac::PPIB22, 19);
impl_ppib_channel!(PPIB22_CH20, pac::PPIB22, 20);
impl_ppib_channel!(PPIB22_CH21, pac::PPIB22, 21);
impl_ppib_channel!(PPIB22_CH22, pac::PPIB22, 22);
impl_ppib_channel!(PPIB22_CH23, pac::PPIB22, 23);
impl_ppib_channel!(PPIB22_CH24, pac::PPIB22, 24);
impl_ppib_channel!(PPIB22_CH25, pac::PPIB22, 25);
impl_ppib_channel!(PPIB22_CH26, pac::PPIB22, 26);
impl_ppib_channel!(PPIB22_CH27, pac::PPIB22, 27);
impl_ppib_channel!(PPIB22_CH28, pac::PPIB22, 28);
impl_ppib_channel!(PPIB22_CH29, pac::PPIB22, 29);
impl_ppib_channel!(PPIB22_CH30, pac::PPIB22, 30);
impl_ppib_channel!(PPIB22_CH31, pac::PPIB22, 31);

// PPIB30 channels
impl_ppib_channel!(PPIB30_CH0, pac::PPIB30, 0);
impl_ppib_channel!(PPIB30_CH1, pac::PPIB30, 1);
impl_ppib_channel!(PPIB30_CH2, pac::PPIB30, 2);
impl_ppib_channel!(PPIB30_CH3, pac::PPIB30, 3);
impl_ppib_channel!(PPIB30_CH4, pac::PPIB30, 4);
impl_ppib_channel!(PPIB30_CH5, pac::PPIB30, 5);
impl_ppib_channel!(PPIB30_CH6, pac::PPIB30, 6);
impl_ppib_channel!(PPIB30_CH7, pac::PPIB30, 7);
impl_ppib_channel!(PPIB30_CH8, pac::PPIB30, 8);
impl_ppib_channel!(PPIB30_CH9, pac::PPIB30, 9);
impl_ppib_channel!(PPIB30_CH10, pac::PPIB30, 10);
impl_ppib_channel!(PPIB30_CH11, pac::PPIB30, 11);
impl_ppib_channel!(PPIB30_CH12, pac::PPIB30, 12);
impl_ppib_channel!(PPIB30_CH13, pac::PPIB30, 13);
impl_ppib_channel!(PPIB30_CH14, pac::PPIB30, 14);
impl_ppib_channel!(PPIB30_CH15, pac::PPIB30, 15);
impl_ppib_channel!(PPIB30_CH16, pac::PPIB30, 16);
impl_ppib_channel!(PPIB30_CH17, pac::PPIB30, 17);
impl_ppib_channel!(PPIB30_CH18, pac::PPIB30, 18);
impl_ppib_channel!(PPIB30_CH19, pac::PPIB30, 19);
impl_ppib_channel!(PPIB30_CH20, pac::PPIB30, 20);
impl_ppib_channel!(PPIB30_CH21, pac::PPIB30, 21);
impl_ppib_channel!(PPIB30_CH22, pac::PPIB30, 22);
impl_ppib_channel!(PPIB30_CH23, pac::PPIB30, 23);
impl_ppib_channel!(PPIB30_CH24, pac::PPIB30, 24);
impl_ppib_channel!(PPIB30_CH25, pac::PPIB30, 25);
impl_ppib_channel!(PPIB30_CH26, pac::PPIB30, 26);
impl_ppib_channel!(PPIB30_CH27, pac::PPIB30, 27);
impl_ppib_channel!(PPIB30_CH28, pac::PPIB30, 28);
impl_ppib_channel!(PPIB30_CH29, pac::PPIB30, 29);
impl_ppib_channel!(PPIB30_CH30, pac::PPIB30, 30);
impl_ppib_channel!(PPIB30_CH31, pac::PPIB30, 31);

// PWM
impl_pwm!(PWM20, PWM20, PWM20);
impl_pwm!(PWM21, PWM21, PWM21);
impl_pwm!(PWM22, PWM22, PWM22);

// Radio
impl_radio!(RADIO, RADIO, RADIO_0);
// impl_radio!(RADIO, RADIO, RADIO_1);

// SAADC
// NOTE: SAADC uses "pin" abstraction, not "AIN"
impl_saadc_input!(P1_04, 1, 4);
impl_saadc_input!(P1_05, 1, 5);
impl_saadc_input!(P1_06, 1, 6);
impl_saadc_input!(P1_07, 1, 7);
impl_saadc_input!(P1_11, 1, 11);
impl_saadc_input!(P1_12, 1, 12);
impl_saadc_input!(P1_13, 1, 13);
impl_saadc_input!(P1_14, 1, 14);

// SPIM
impl_spim!(SPI00, SPIM00, SERIAL00);
impl_spim!(TWISPI20, SPIM20, SERIAL20);
impl_spim!(TWISPI21, SPIM21, SERIAL21);
impl_spim!(TWISPI22, SPIM22, SERIAL22);
impl_spim!(TWISPI30, SPIM30, SERIAL30);

// SPIS
impl_spis!(SPI00, SPIS00, SERIAL00);
impl_spis!(TWISPI20, SPIS20, SERIAL20);
impl_spis!(TWISPI21, SPIS21, SERIAL21);
impl_spis!(TWISPI22, SPIS22, SERIAL22);
impl_spis!(TWISPI30, SPIS30, SERIAL30);

// TWIM
impl_twim!(TWISPI20, TWIM20, SERIAL20);
impl_twim!(TWISPI21, TWIM21, SERIAL21);
impl_twim!(TWISPI22, TWIM22, SERIAL22);
impl_twim!(TWISPI30, TWIM30, SERIAL30);

// TWIS
impl_twis!(TWISPI20, TWIS20, SERIAL20);
impl_twis!(TWISPI21, TWIS21, SERIAL21);
impl_twis!(TWISPI22, TWIS22, SERIAL22);
impl_twis!(TWISPI30, TWIS30, SERIAL30);

// TIMER
impl_timer!(TIMER00, TIMER00, TIMER00);
// Dedicated to radio. Implemented for nrf-sdc
impl_timer!(TIMER10, TIMER10, TIMER10);
impl_timer!(TIMER20, TIMER20, TIMER20);
impl_timer!(TIMER21, TIMER21, TIMER21);
impl_timer!(TIMER22, TIMER22, TIMER22);
impl_timer!(TIMER23, TIMER23, TIMER23);
impl_timer!(TIMER24, TIMER24, TIMER24);

// UARTE
impl_uarte!(TWISPI20, UARTE20, SERIAL20);
impl_uarte!(TWISPI21, UARTE21, SERIAL21);
impl_uarte!(TWISPI22, UARTE22, SERIAL22);
impl_uarte!(TWISPI30, UARTE30, SERIAL30);

#[cfg(feature = "_ns")]
impl_wdt!(WDT, WDT31, WDT31, 0);
#[cfg(feature = "_s")]
impl_wdt!(WDT0, WDT31, WDT31, 0);
#[cfg(feature = "_s")]
impl_wdt!(WDT1, WDT30, WDT30, 1);

embassy_hal_internal::interrupt_mod!(
    SWI00,
    SWI01,
    SWI02,
    SWI03,
    SPU00,
    MPC00,
    AAR00_CCM00,
    ECB00,
    CRACEN,
    SERIAL00,
    RRAMC,
    VPR00,
    CTRLAP,
    TIMER00,
    SPU10,
    TIMER10,
    RTC10,
    EGU10,
    RADIO_0,
    RADIO_1,
    SPU20,
    SERIAL20,
    SERIAL21,
    SERIAL22,
    EGU20,
    TIMER20,
    TIMER21,
    TIMER22,
    TIMER23,
    TIMER24,
    PDM20,
    PDM21,
    PWM20,
    PWM21,
    PWM22,
    SAADC,
    NFCT,
    TEMP,
    GPIOTE20_0,
    GPIOTE20_1,
    TAMPC,
    I2S20,
    QDEC20,
    QDEC21,
    GRTC_0,
    GRTC_1,
    GRTC_2,
    GRTC_3,
    SPU30,
    SERIAL30,
    RTC30,
    COMP_LPCOMP,
    WDT30,
    WDT31,
    GPIOTE30_0,
    GPIOTE30_1,
    CLOCK_POWER,
);
