#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 124usize],
    #[doc = "0x80 - PLL Control register"]
    pub pll0con: PLLCON,
    #[doc = "0x84 - PLL Configuration register"]
    pub pll0cfg: PLLCFG,
    #[doc = "0x88 - PLL Status register"]
    pub pll0stat: PLLSTAT,
    #[doc = "0x8c - PLL0 Feed register"]
    pub pll0feed: PLLFEED,
    _reserved5: [u8; 16usize],
    #[doc = "0xa0 - PLL Control register"]
    pub pll1con: PLLCON,
    #[doc = "0xa4 - PLL Configuration register"]
    pub pll1cfg: PLLCFG,
    #[doc = "0xa8 - PLL Status register"]
    pub pll1stat: PLLSTAT,
    #[doc = "0xac - PLL0 Feed register"]
    pub pll1feed: PLLFEED,
    _reserved9: [u8; 16usize],
    #[doc = "0xc0 - Power Control register"]
    pub pcon: PCON,
    #[doc = "0xc4 - Power Control for Peripherals"]
    pub pconp: PCONP,
    _reserved11: [u8; 56usize],
    #[doc = "0x100 - External Memory Controller Clock Selection register"]
    pub emcclksel: EMCCLKSEL,
    #[doc = "0x104 - CPU Clock Selection register"]
    pub cclksel: CCLKSEL,
    #[doc = "0x108 - USB Clock Selection register"]
    pub usbclksel: USBCLKSEL,
    #[doc = "0x10c - Clock Source Select Register"]
    pub clksrcsel: CLKSRCSEL,
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    pub cansleepclr: CANSLEEPCLR,
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    pub canwakeflags: CANWAKEFLAGS,
    _reserved17: [u8; 40usize],
    #[doc = "0x140 - External Interrupt Flag Register"]
    pub extint: EXTINT,
    _reserved18: [u8; 4usize],
    #[doc = "0x148 - External Interrupt Mode register"]
    pub extmode: EXTMODE,
    #[doc = "0x14c - External Interrupt Polarity Register"]
    pub extpolar: EXTPOLAR,
    _reserved20: [u8; 48usize],
    #[doc = "0x180 - Reset Source Identification Register"]
    pub rsid: RSID,
    _reserved21: [u8; 4usize],
    #[doc = "0x188 - Matrix arbitration register"]
    pub matrixarb: MATRIXARB,
    _reserved22: [u8; 20usize],
    #[doc = "0x1a0 - System Control and Status"]
    pub scs: SCS,
    _reserved23: [u8; 4usize],
    #[doc = "0x1a8 - Peripheral Clock Selection register"]
    pub pclksel: PCLKSEL,
    _reserved24: [u8; 4usize],
    #[doc = "0x1b0 - Power boost register"]
    pub pboost: PBOOST,
    #[doc = "0x1b4 - SPIFI Clock Selection register"]
    pub spificlksel: SPIFICLKSEL,
    #[doc = "0x1b8 - LCD Clock configuration register"]
    pub lcd_cfg: LCD_CFG,
    _reserved27: [u8; 4usize],
    #[doc = "0x1c0 - USB Interrupt Status"]
    pub usbintst: USBINTST,
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    pub dmacreqsel: DMACREQSEL,
    #[doc = "0x1c8 - Clock Output Configuration register"]
    pub clkoutcfg: CLKOUTCFG,
    #[doc = "0x1cc - Individual peripheral reset control bits"]
    pub rstcon0: RSTCON0,
    #[doc = "0x1d0 - Individual peripheral reset control bits"]
    pub rstcon1: RSTCON1,
    _reserved32: [u8; 8usize],
    #[doc = "0x1dc - Values for the 4 programmable delays associated with SDRAM operation."]
    pub emcdlyctl: EMCDLYCTL,
    #[doc = "0x1e0 - Controls the calibration counter for programmable delays and returns the result value."]
    pub emccal: EMCCAL,
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](flashcfg) module"]
pub type FLASHCFG = crate::Reg<u32, _FLASHCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCFG;
#[doc = "`read()` method returns [flashcfg::R](flashcfg::R) reader structure"]
impl crate::Readable for FLASHCFG {}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](flashcfg::W) writer structure"]
impl crate::Writable for FLASHCFG {}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLL Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcon](pllcon) module"]
pub type PLLCON = crate::Reg<u32, _PLLCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCON;
#[doc = "`read()` method returns [pllcon::R](pllcon::R) reader structure"]
impl crate::Readable for PLLCON {}
#[doc = "`write(|w| ..)` method takes [pllcon::W](pllcon::W) writer structure"]
impl crate::Writable for PLLCON {}
#[doc = "PLL Control register"]
pub mod pllcon;
#[doc = "PLL Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfg](pllcfg) module"]
pub type PLLCFG = crate::Reg<u32, _PLLCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFG;
#[doc = "`read()` method returns [pllcfg::R](pllcfg::R) reader structure"]
impl crate::Readable for PLLCFG {}
#[doc = "`write(|w| ..)` method takes [pllcfg::W](pllcfg::W) writer structure"]
impl crate::Writable for PLLCFG {}
#[doc = "PLL Configuration register"]
pub mod pllcfg;
#[doc = "PLL Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllstat](pllstat) module"]
pub type PLLSTAT = crate::Reg<u32, _PLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSTAT;
#[doc = "`read()` method returns [pllstat::R](pllstat::R) reader structure"]
impl crate::Readable for PLLSTAT {}
#[doc = "PLL Status register"]
pub mod pllstat;
#[doc = "PLL0 Feed register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllfeed](pllfeed) module"]
pub type PLLFEED = crate::Reg<u32, _PLLFEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLFEED;
#[doc = "`write(|w| ..)` method takes [pllfeed::W](pllfeed::W) writer structure"]
impl crate::Writable for PLLFEED {}
#[doc = "PLL0 Feed register"]
pub mod pllfeed;
#[doc = "Power Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](pcon) module"]
pub type PCON = crate::Reg<u32, _PCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCON;
#[doc = "`read()` method returns [pcon::R](pcon::R) reader structure"]
impl crate::Readable for PCON {}
#[doc = "`write(|w| ..)` method takes [pcon::W](pcon::W) writer structure"]
impl crate::Writable for PCON {}
#[doc = "Power Control register"]
pub mod pcon;
#[doc = "Power Control for Peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconp](pconp) module"]
pub type PCONP = crate::Reg<u32, _PCONP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCONP;
#[doc = "`read()` method returns [pconp::R](pconp::R) reader structure"]
impl crate::Readable for PCONP {}
#[doc = "`write(|w| ..)` method takes [pconp::W](pconp::W) writer structure"]
impl crate::Writable for PCONP {}
#[doc = "Power Control for Peripherals"]
pub mod pconp;
#[doc = "External Memory Controller Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcclksel](emcclksel) module"]
pub type EMCCLKSEL = crate::Reg<u32, _EMCCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMCCLKSEL;
#[doc = "`read()` method returns [emcclksel::R](emcclksel::R) reader structure"]
impl crate::Readable for EMCCLKSEL {}
#[doc = "`write(|w| ..)` method takes [emcclksel::W](emcclksel::W) writer structure"]
impl crate::Writable for EMCCLKSEL {}
#[doc = "External Memory Controller Clock Selection register"]
pub mod emcclksel;
#[doc = "CPU Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cclksel](cclksel) module"]
pub type CCLKSEL = crate::Reg<u32, _CCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCLKSEL;
#[doc = "`read()` method returns [cclksel::R](cclksel::R) reader structure"]
impl crate::Readable for CCLKSEL {}
#[doc = "`write(|w| ..)` method takes [cclksel::W](cclksel::W) writer structure"]
impl crate::Writable for CCLKSEL {}
#[doc = "CPU Clock Selection register"]
pub mod cclksel;
#[doc = "USB Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclksel](usbclksel) module"]
pub type USBCLKSEL = crate::Reg<u32, _USBCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCLKSEL;
#[doc = "`read()` method returns [usbclksel::R](usbclksel::R) reader structure"]
impl crate::Readable for USBCLKSEL {}
#[doc = "`write(|w| ..)` method takes [usbclksel::W](usbclksel::W) writer structure"]
impl crate::Writable for USBCLKSEL {}
#[doc = "USB Clock Selection register"]
pub mod usbclksel;
#[doc = "Clock Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksrcsel](clksrcsel) module"]
pub type CLKSRCSEL = crate::Reg<u32, _CLKSRCSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSRCSEL;
#[doc = "`read()` method returns [clksrcsel::R](clksrcsel::R) reader structure"]
impl crate::Readable for CLKSRCSEL {}
#[doc = "`write(|w| ..)` method takes [clksrcsel::W](clksrcsel::W) writer structure"]
impl crate::Writable for CLKSRCSEL {}
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cansleepclr](cansleepclr) module"]
pub type CANSLEEPCLR = crate::Reg<u32, _CANSLEEPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANSLEEPCLR;
#[doc = "`read()` method returns [cansleepclr::R](cansleepclr::R) reader structure"]
impl crate::Readable for CANSLEEPCLR {}
#[doc = "`write(|w| ..)` method takes [cansleepclr::W](cansleepclr::W) writer structure"]
impl crate::Writable for CANSLEEPCLR {}
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "Allows reading the wake-up state of the CAN channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canwakeflags](canwakeflags) module"]
pub type CANWAKEFLAGS = crate::Reg<u32, _CANWAKEFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANWAKEFLAGS;
#[doc = "`read()` method returns [canwakeflags::R](canwakeflags::R) reader structure"]
impl crate::Readable for CANWAKEFLAGS {}
#[doc = "`write(|w| ..)` method takes [canwakeflags::W](canwakeflags::W) writer structure"]
impl crate::Writable for CANWAKEFLAGS {}
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "External Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extint](extint) module"]
pub type EXTINT = crate::Reg<u32, _EXTINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTINT;
#[doc = "`read()` method returns [extint::R](extint::R) reader structure"]
impl crate::Readable for EXTINT {}
#[doc = "`write(|w| ..)` method takes [extint::W](extint::W) writer structure"]
impl crate::Writable for EXTINT {}
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "External Interrupt Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmode](extmode) module"]
pub type EXTMODE = crate::Reg<u32, _EXTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMODE;
#[doc = "`read()` method returns [extmode::R](extmode::R) reader structure"]
impl crate::Readable for EXTMODE {}
#[doc = "`write(|w| ..)` method takes [extmode::W](extmode::W) writer structure"]
impl crate::Writable for EXTMODE {}
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "External Interrupt Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extpolar](extpolar) module"]
pub type EXTPOLAR = crate::Reg<u32, _EXTPOLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTPOLAR;
#[doc = "`read()` method returns [extpolar::R](extpolar::R) reader structure"]
impl crate::Readable for EXTPOLAR {}
#[doc = "`write(|w| ..)` method takes [extpolar::W](extpolar::W) writer structure"]
impl crate::Writable for EXTPOLAR {}
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "Reset Source Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsid](rsid) module"]
pub type RSID = crate::Reg<u32, _RSID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSID;
#[doc = "`read()` method returns [rsid::R](rsid::R) reader structure"]
impl crate::Readable for RSID {}
#[doc = "`write(|w| ..)` method takes [rsid::W](rsid::W) writer structure"]
impl crate::Writable for RSID {}
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "Matrix arbitration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrixarb](matrixarb) module"]
pub type MATRIXARB = crate::Reg<u32, _MATRIXARB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIXARB;
#[doc = "`read()` method returns [matrixarb::R](matrixarb::R) reader structure"]
impl crate::Readable for MATRIXARB {}
#[doc = "`write(|w| ..)` method takes [matrixarb::W](matrixarb::W) writer structure"]
impl crate::Writable for MATRIXARB {}
#[doc = "Matrix arbitration register"]
pub mod matrixarb;
#[doc = "System Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs](scs) module"]
pub type SCS = crate::Reg<u32, _SCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCS;
#[doc = "`read()` method returns [scs::R](scs::R) reader structure"]
impl crate::Readable for SCS {}
#[doc = "`write(|w| ..)` method takes [scs::W](scs::W) writer structure"]
impl crate::Writable for SCS {}
#[doc = "System Control and Status"]
pub mod scs;
#[doc = "Peripheral Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksel](pclksel) module"]
pub type PCLKSEL = crate::Reg<u32, _PCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCLKSEL;
#[doc = "`read()` method returns [pclksel::R](pclksel::R) reader structure"]
impl crate::Readable for PCLKSEL {}
#[doc = "`write(|w| ..)` method takes [pclksel::W](pclksel::W) writer structure"]
impl crate::Writable for PCLKSEL {}
#[doc = "Peripheral Clock Selection register"]
pub mod pclksel;
#[doc = "Power boost register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pboost](pboost) module"]
pub type PBOOST = crate::Reg<u32, _PBOOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBOOST;
#[doc = "`read()` method returns [pboost::R](pboost::R) reader structure"]
impl crate::Readable for PBOOST {}
#[doc = "`write(|w| ..)` method takes [pboost::W](pboost::W) writer structure"]
impl crate::Writable for PBOOST {}
#[doc = "Power boost register"]
pub mod pboost;
#[doc = "SPIFI Clock Selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spificlksel](spificlksel) module"]
pub type SPIFICLKSEL = crate::Reg<u32, _SPIFICLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIFICLKSEL;
#[doc = "`read()` method returns [spificlksel::R](spificlksel::R) reader structure"]
impl crate::Readable for SPIFICLKSEL {}
#[doc = "`write(|w| ..)` method takes [spificlksel::W](spificlksel::W) writer structure"]
impl crate::Writable for SPIFICLKSEL {}
#[doc = "SPIFI Clock Selection register"]
pub mod spificlksel;
#[doc = "LCD Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cfg](lcd_cfg) module"]
pub type LCD_CFG = crate::Reg<u32, _LCD_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCD_CFG;
#[doc = "`read()` method returns [lcd_cfg::R](lcd_cfg::R) reader structure"]
impl crate::Readable for LCD_CFG {}
#[doc = "`write(|w| ..)` method takes [lcd_cfg::W](lcd_cfg::W) writer structure"]
impl crate::Writable for LCD_CFG {}
#[doc = "LCD Clock configuration register"]
pub mod lcd_cfg;
#[doc = "USB Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintst](usbintst) module"]
pub type USBINTST = crate::Reg<u32, _USBINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBINTST;
#[doc = "`read()` method returns [usbintst::R](usbintst::R) reader structure"]
impl crate::Readable for USBINTST {}
#[doc = "`write(|w| ..)` method takes [usbintst::W](usbintst::W) writer structure"]
impl crate::Writable for USBINTST {}
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacreqsel](dmacreqsel) module"]
pub type DMACREQSEL = crate::Reg<u32, _DMACREQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACREQSEL;
#[doc = "`read()` method returns [dmacreqsel::R](dmacreqsel::R) reader structure"]
impl crate::Readable for DMACREQSEL {}
#[doc = "`write(|w| ..)` method takes [dmacreqsel::W](dmacreqsel::W) writer structure"]
impl crate::Writable for DMACREQSEL {}
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "Clock Output Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutcfg](clkoutcfg) module"]
pub type CLKOUTCFG = crate::Reg<u32, _CLKOUTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTCFG;
#[doc = "`read()` method returns [clkoutcfg::R](clkoutcfg::R) reader structure"]
impl crate::Readable for CLKOUTCFG {}
#[doc = "`write(|w| ..)` method takes [clkoutcfg::W](clkoutcfg::W) writer structure"]
impl crate::Writable for CLKOUTCFG {}
#[doc = "Clock Output Configuration register"]
pub mod clkoutcfg;
#[doc = "Individual peripheral reset control bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcon0](rstcon0) module"]
pub type RSTCON0 = crate::Reg<u32, _RSTCON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCON0;
#[doc = "`read()` method returns [rstcon0::R](rstcon0::R) reader structure"]
impl crate::Readable for RSTCON0 {}
#[doc = "`write(|w| ..)` method takes [rstcon0::W](rstcon0::W) writer structure"]
impl crate::Writable for RSTCON0 {}
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon0;
#[doc = "Individual peripheral reset control bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcon1](rstcon1) module"]
pub type RSTCON1 = crate::Reg<u32, _RSTCON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCON1;
#[doc = "`read()` method returns [rstcon1::R](rstcon1::R) reader structure"]
impl crate::Readable for RSTCON1 {}
#[doc = "`write(|w| ..)` method takes [rstcon1::W](rstcon1::W) writer structure"]
impl crate::Writable for RSTCON1 {}
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon1;
#[doc = "Values for the 4 programmable delays associated with SDRAM operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcdlyctl](emcdlyctl) module"]
pub type EMCDLYCTL = crate::Reg<u32, _EMCDLYCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMCDLYCTL;
#[doc = "`read()` method returns [emcdlyctl::R](emcdlyctl::R) reader structure"]
impl crate::Readable for EMCDLYCTL {}
#[doc = "`write(|w| ..)` method takes [emcdlyctl::W](emcdlyctl::W) writer structure"]
impl crate::Writable for EMCDLYCTL {}
#[doc = "Values for the 4 programmable delays associated with SDRAM operation."]
pub mod emcdlyctl;
#[doc = "Controls the calibration counter for programmable delays and returns the result value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emccal](emccal) module"]
pub type EMCCAL = crate::Reg<u32, _EMCCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMCCAL;
#[doc = "`read()` method returns [emccal::R](emccal::R) reader structure"]
impl crate::Readable for EMCCAL {}
#[doc = "`write(|w| ..)` method takes [emccal::W](emccal::W) writer structure"]
impl crate::Writable for EMCCAL {}
#[doc = "Controls the calibration counter for programmable delays and returns the result value."]
pub mod emccal;
