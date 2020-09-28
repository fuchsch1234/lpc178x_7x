#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - 128-bit signature Word 0"]
    pub fmsw0: FMSW0,
    #[doc = "0x30 - 128-bit signature Word 1"]
    pub fmsw1: FMSW1,
    #[doc = "0x34 - 128-bit signature Word 2"]
    pub fmsw2: FMSW2,
    #[doc = "0x38 - 128-bit signature Word 3"]
    pub fmsw3: FMSW3,
    _reserved6: [u8; 68usize],
    #[doc = "0x80 - EEPROM command register"]
    pub eecmd: EECMD,
    #[doc = "0x84 - EEPROM address register"]
    pub eeaddr: EEADDR,
    #[doc = "0x88 - EEPROM write data register"]
    pub eewdata: EEWDATA,
    #[doc = "0x8c - EEPROM read data register"]
    pub eerdata: EERDATA,
    #[doc = "0x90 - EEPROM wait state register"]
    pub eewstate: EEWSTATE,
    #[doc = "0x94 - EEPROM clock divider register"]
    pub eeclkdiv: EECLKDIV,
    #[doc = "0x98 - EEPROM power-down register"]
    pub eepwrdwn: EEPWRDWN,
    _reserved13: [u8; 3900usize],
    #[doc = "0xfd8 - EEPROM interrupt enable clear"]
    pub enclr: ENCLR,
    #[doc = "0xfdc - EEPROM interrupt enable set"]
    pub enset: ENSET,
    #[doc = "0xfe0 - Signature generation status register"]
    pub stat: STAT,
    #[doc = "0xfe4 - EEPROM interrupt enable"]
    pub inten: INTEN,
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub statclr: STATCLR,
}
#[doc = "Signature start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstart](fmsstart) module"]
pub type FMSSTART = crate::Reg<u32, _FMSSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSSTART;
#[doc = "`read()` method returns [fmsstart::R](fmsstart::R) reader structure"]
impl crate::Readable for FMSSTART {}
#[doc = "`write(|w| ..)` method takes [fmsstart::W](fmsstart::W) writer structure"]
impl crate::Writable for FMSSTART {}
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "Signature stop-address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstop](fmsstop) module"]
pub type FMSSTOP = crate::Reg<u32, _FMSSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSSTOP;
#[doc = "`read()` method returns [fmsstop::R](fmsstop::R) reader structure"]
impl crate::Readable for FMSSTOP {}
#[doc = "`write(|w| ..)` method takes [fmsstop::W](fmsstop::W) writer structure"]
impl crate::Writable for FMSSTOP {}
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "128-bit signature Word 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw0](fmsw0) module"]
pub type FMSW0 = crate::Reg<u32, _FMSW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW0;
#[doc = "`read()` method returns [fmsw0::R](fmsw0::R) reader structure"]
impl crate::Readable for FMSW0 {}
#[doc = "128-bit signature Word 0"]
pub mod fmsw0;
#[doc = "128-bit signature Word 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw1](fmsw1) module"]
pub type FMSW1 = crate::Reg<u32, _FMSW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW1;
#[doc = "`read()` method returns [fmsw1::R](fmsw1::R) reader structure"]
impl crate::Readable for FMSW1 {}
#[doc = "128-bit signature Word 1"]
pub mod fmsw1;
#[doc = "128-bit signature Word 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw2](fmsw2) module"]
pub type FMSW2 = crate::Reg<u32, _FMSW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW2;
#[doc = "`read()` method returns [fmsw2::R](fmsw2::R) reader structure"]
impl crate::Readable for FMSW2 {}
#[doc = "128-bit signature Word 2"]
pub mod fmsw2;
#[doc = "128-bit signature Word 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw3](fmsw3) module"]
pub type FMSW3 = crate::Reg<u32, _FMSW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW3;
#[doc = "`read()` method returns [fmsw3::R](fmsw3::R) reader structure"]
impl crate::Readable for FMSW3 {}
#[doc = "128-bit signature Word 3"]
pub mod fmsw3;
#[doc = "EEPROM command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecmd](eecmd) module"]
pub type EECMD = crate::Reg<u32, _EECMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECMD;
#[doc = "`read()` method returns [eecmd::R](eecmd::R) reader structure"]
impl crate::Readable for EECMD {}
#[doc = "`write(|w| ..)` method takes [eecmd::W](eecmd::W) writer structure"]
impl crate::Writable for EECMD {}
#[doc = "EEPROM command register"]
pub mod eecmd;
#[doc = "EEPROM address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeaddr](eeaddr) module"]
pub type EEADDR = crate::Reg<u32, _EEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEADDR;
#[doc = "`read()` method returns [eeaddr::R](eeaddr::R) reader structure"]
impl crate::Readable for EEADDR {}
#[doc = "`write(|w| ..)` method takes [eeaddr::W](eeaddr::W) writer structure"]
impl crate::Writable for EEADDR {}
#[doc = "EEPROM address register"]
pub mod eeaddr;
#[doc = "EEPROM write data register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eewdata](eewdata) module"]
pub type EEWDATA = crate::Reg<u32, _EEWDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEWDATA;
#[doc = "`write(|w| ..)` method takes [eewdata::W](eewdata::W) writer structure"]
impl crate::Writable for EEWDATA {}
#[doc = "EEPROM write data register"]
pub mod eewdata;
#[doc = "EEPROM read data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eerdata](eerdata) module"]
pub type EERDATA = crate::Reg<u32, _EERDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EERDATA;
#[doc = "`read()` method returns [eerdata::R](eerdata::R) reader structure"]
impl crate::Readable for EERDATA {}
#[doc = "EEPROM read data register"]
pub mod eerdata;
#[doc = "EEPROM wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eewstate](eewstate) module"]
pub type EEWSTATE = crate::Reg<u32, _EEWSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEWSTATE;
#[doc = "`read()` method returns [eewstate::R](eewstate::R) reader structure"]
impl crate::Readable for EEWSTATE {}
#[doc = "`write(|w| ..)` method takes [eewstate::W](eewstate::W) writer structure"]
impl crate::Writable for EEWSTATE {}
#[doc = "EEPROM wait state register"]
pub mod eewstate;
#[doc = "EEPROM clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeclkdiv](eeclkdiv) module"]
pub type EECLKDIV = crate::Reg<u32, _EECLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECLKDIV;
#[doc = "`read()` method returns [eeclkdiv::R](eeclkdiv::R) reader structure"]
impl crate::Readable for EECLKDIV {}
#[doc = "`write(|w| ..)` method takes [eeclkdiv::W](eeclkdiv::W) writer structure"]
impl crate::Writable for EECLKDIV {}
#[doc = "EEPROM clock divider register"]
pub mod eeclkdiv;
#[doc = "EEPROM power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eepwrdwn](eepwrdwn) module"]
pub type EEPWRDWN = crate::Reg<u32, _EEPWRDWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEPWRDWN;
#[doc = "`read()` method returns [eepwrdwn::R](eepwrdwn::R) reader structure"]
impl crate::Readable for EEPWRDWN {}
#[doc = "`write(|w| ..)` method takes [eepwrdwn::W](eepwrdwn::W) writer structure"]
impl crate::Writable for EEPWRDWN {}
#[doc = "EEPROM power-down register"]
pub mod eepwrdwn;
#[doc = "Signature generation status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Signature generation status register"]
pub mod stat;
#[doc = "EEPROM interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "EEPROM interrupt enable"]
pub mod inten;
#[doc = "Signature generation status clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statclr](statclr) module"]
pub type STATCLR = crate::Reg<u32, _STATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATCLR;
#[doc = "`write(|w| ..)` method takes [statclr::W](statclr::W) writer structure"]
impl crate::Writable for STATCLR {}
#[doc = "Signature generation status clear register"]
pub mod statclr;
#[doc = "EEPROM interrupt enable clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enclr](enclr) module"]
pub type ENCLR = crate::Reg<u32, _ENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENCLR;
#[doc = "`write(|w| ..)` method takes [enclr::W](enclr::W) writer structure"]
impl crate::Writable for ENCLR {}
#[doc = "EEPROM interrupt enable clear"]
pub mod enclr;
#[doc = "EEPROM interrupt enable set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enset](enset) module"]
pub type ENSET = crate::Reg<u32, _ENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENSET;
#[doc = "`write(|w| ..)` method takes [enset::W](enset::W) writer structure"]
impl crate::Writable for ENSET {}
#[doc = "EEPROM interrupt enable set"]
pub mod enset;
