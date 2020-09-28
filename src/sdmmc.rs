#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register."]
    pub pwr: PWR,
    #[doc = "0x04 - Clock control register."]
    pub clock: CLOCK,
    #[doc = "0x08 - Argument register."]
    pub argument: ARGUMENT,
    #[doc = "0x0c - Command register."]
    pub command: COMMAND,
    #[doc = "0x10 - Response command register."]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - Response register."]
    pub response: [RESPONSE; 4],
    #[doc = "0x24 - Data Timer."]
    pub datatimer: DATATIMER,
    #[doc = "0x28 - Data length register."]
    pub datalength: DATALENGTH,
    #[doc = "0x2c - Data control register."]
    pub datactrl: DATACTRL,
    #[doc = "0x30 - Data counter."]
    pub datacnt: DATACNT,
    #[doc = "0x34 - Status register."]
    pub status: STATUS,
    #[doc = "0x38 - Clear register."]
    pub clear: CLEAR,
    #[doc = "0x3c - Interrupt 0 mask register."]
    pub mask0: MASK0,
    _reserved13: [u8; 8usize],
    #[doc = "0x48 - FIFO Counter."]
    pub fifocnt: FIFOCNT,
    _reserved14: [u8; 52usize],
    #[doc = "0x80 - Data FIFO Register."]
    pub fifo: [FIFO; 16],
}
#[doc = "Power control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr](pwr) module"]
pub type PWR = crate::Reg<u32, _PWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR;
#[doc = "`read()` method returns [pwr::R](pwr::R) reader structure"]
impl crate::Readable for PWR {}
#[doc = "`write(|w| ..)` method takes [pwr::W](pwr::W) writer structure"]
impl crate::Writable for PWR {}
#[doc = "Power control register."]
pub mod pwr;
#[doc = "Clock control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "Clock control register."]
pub mod clock;
#[doc = "Argument register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argument](argument) module"]
pub type ARGUMENT = crate::Reg<u32, _ARGUMENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARGUMENT;
#[doc = "`read()` method returns [argument::R](argument::R) reader structure"]
impl crate::Readable for ARGUMENT {}
#[doc = "`write(|w| ..)` method takes [argument::W](argument::W) writer structure"]
impl crate::Writable for ARGUMENT {}
#[doc = "Argument register."]
pub mod argument;
#[doc = "Command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command](command) module"]
pub type COMMAND = crate::Reg<u32, _COMMAND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMMAND;
#[doc = "`read()` method returns [command::R](command::R) reader structure"]
impl crate::Readable for COMMAND {}
#[doc = "`write(|w| ..)` method takes [command::W](command::W) writer structure"]
impl crate::Writable for COMMAND {}
#[doc = "Command register."]
pub mod command;
#[doc = "Response command register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmd](respcmd) module"]
pub type RESPCMD = crate::Reg<u32, _RESPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPCMD;
#[doc = "`read()` method returns [respcmd::R](respcmd::R) reader structure"]
impl crate::Readable for RESPCMD {}
#[doc = "Response command register."]
pub mod respcmd;
#[doc = "Response register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [response](response) module"]
pub type RESPONSE = crate::Reg<u32, _RESPONSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPONSE;
#[doc = "`read()` method returns [response::R](response::R) reader structure"]
impl crate::Readable for RESPONSE {}
#[doc = "Response register."]
pub mod response;
#[doc = "Data Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datatimer](datatimer) module"]
pub type DATATIMER = crate::Reg<u32, _DATATIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATATIMER;
#[doc = "`read()` method returns [datatimer::R](datatimer::R) reader structure"]
impl crate::Readable for DATATIMER {}
#[doc = "`write(|w| ..)` method takes [datatimer::W](datatimer::W) writer structure"]
impl crate::Writable for DATATIMER {}
#[doc = "Data Timer."]
pub mod datatimer;
#[doc = "Data length register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalength](datalength) module"]
pub type DATALENGTH = crate::Reg<u32, _DATALENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATALENGTH;
#[doc = "`read()` method returns [datalength::R](datalength::R) reader structure"]
impl crate::Readable for DATALENGTH {}
#[doc = "`write(|w| ..)` method takes [datalength::W](datalength::W) writer structure"]
impl crate::Writable for DATALENGTH {}
#[doc = "Data length register."]
pub mod datalength;
#[doc = "Data control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datactrl](datactrl) module"]
pub type DATACTRL = crate::Reg<u32, _DATACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATACTRL;
#[doc = "`read()` method returns [datactrl::R](datactrl::R) reader structure"]
impl crate::Readable for DATACTRL {}
#[doc = "`write(|w| ..)` method takes [datactrl::W](datactrl::W) writer structure"]
impl crate::Writable for DATACTRL {}
#[doc = "Data control register."]
pub mod datactrl;
#[doc = "Data counter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datacnt](datacnt) module"]
pub type DATACNT = crate::Reg<u32, _DATACNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATACNT;
#[doc = "`read()` method returns [datacnt::R](datacnt::R) reader structure"]
impl crate::Readable for DATACNT {}
#[doc = "Data counter."]
pub mod datacnt;
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status register."]
pub mod status;
#[doc = "Clear register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clear](clear) module"]
pub type CLEAR = crate::Reg<u32, _CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEAR;
#[doc = "`write(|w| ..)` method takes [clear::W](clear::W) writer structure"]
impl crate::Writable for CLEAR {}
#[doc = "Clear register."]
pub mod clear;
#[doc = "Interrupt 0 mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask0](mask0) module"]
pub type MASK0 = crate::Reg<u32, _MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK0;
#[doc = "`read()` method returns [mask0::R](mask0::R) reader structure"]
impl crate::Readable for MASK0 {}
#[doc = "`write(|w| ..)` method takes [mask0::W](mask0::W) writer structure"]
impl crate::Writable for MASK0 {}
#[doc = "Interrupt 0 mask register."]
pub mod mask0;
#[doc = "FIFO Counter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocnt](fifocnt) module"]
pub type FIFOCNT = crate::Reg<u32, _FIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCNT;
#[doc = "`read()` method returns [fifocnt::R](fifocnt::R) reader structure"]
impl crate::Readable for FIFOCNT {}
#[doc = "FIFO Counter."]
pub mod fifocnt;
#[doc = "Data FIFO Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "Data FIFO Register."]
pub mod fifo;
