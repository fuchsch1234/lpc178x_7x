#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls operation of the memory controller."]
    pub control: CONTROL,
    #[doc = "0x04 - Provides EMC status information."]
    pub status: STATUS,
    #[doc = "0x08 - Configures operation of the memory controller"]
    pub config: CONFIG,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - Controls dynamic memory operation."]
    pub dynamiccontrol: DYNAMICCONTROL,
    #[doc = "0x24 - Configures dynamic memory refresh."]
    pub dynamicrefresh: DYNAMICREFRESH,
    #[doc = "0x28 - Configures dynamic memory read strategy."]
    pub dynamicreadconfig: DYNAMICREADCONFIG,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - Precharge command period."]
    pub dynamicrp: DYNAMICRP,
    #[doc = "0x34 - Active to precharge command period."]
    pub dynamicras: DYNAMICRAS,
    #[doc = "0x38 - Self-refresh exit time."]
    pub dynamicsrex: DYNAMICSREX,
    #[doc = "0x3c - Last-data-out to active command time."]
    pub dynamicapr: DYNAMICAPR,
    #[doc = "0x40 - Data-in to active command time."]
    pub dynamicdal: DYNAMICDAL,
    #[doc = "0x44 - Write recovery time."]
    pub dynamicwr: DYNAMICWR,
    #[doc = "0x48 - Selects the active to active command period."]
    pub dynamicrc: DYNAMICRC,
    #[doc = "0x4c - Selects the auto-refresh period."]
    pub dynamicrfc: DYNAMICRFC,
    #[doc = "0x50 - Time for exit self-refresh to active command."]
    pub dynamicxsr: DYNAMICXSR,
    #[doc = "0x54 - Latency for active bank A to active bank B."]
    pub dynamicrrd: DYNAMICRRD,
    #[doc = "0x58 - Time for load mode register to active command."]
    pub dynamicmrd: DYNAMICMRD,
    _reserved17: [u8; 36usize],
    #[doc = "0x80 - Time for long static memory read and write transfers."]
    pub staticextendedwait: STATICEXTENDEDWAIT,
    _reserved18: [u8; 124usize],
    #[doc = "0x100 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig0: DYNAMICCONFIG,
    #[doc = "0x104 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas0: DYNAMICRASCAS,
    _reserved20: [u8; 24usize],
    #[doc = "0x120 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig1: DYNAMICCONFIG,
    #[doc = "0x124 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas1: DYNAMICRASCAS,
    _reserved22: [u8; 24usize],
    #[doc = "0x140 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig2: DYNAMICCONFIG,
    #[doc = "0x144 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas2: DYNAMICRASCAS,
    _reserved24: [u8; 24usize],
    #[doc = "0x160 - Configuration information for EMC_DYCS0."]
    pub dynamicconfig3: DYNAMICCONFIG,
    #[doc = "0x164 - RAS and CAS latencies for EMC_DYCS0."]
    pub dynamicrascas3: DYNAMICRASCAS,
    _reserved26: [u8; 152usize],
    #[doc = "0x200 - Configuration for EMC_CS0."]
    pub staticconfig0: STATICCONFIG,
    #[doc = "0x204 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen0: STATICWAITWEN,
    #[doc = "0x208 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen0: STATICWAITOEN,
    #[doc = "0x20c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd0: STATICWAITRD,
    #[doc = "0x210 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage0: STATICWAITPAGE,
    #[doc = "0x214 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr0: STATICWAITWR,
    #[doc = "0x218 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn0: STATICWAITTURN,
    _reserved33: [u8; 4usize],
    #[doc = "0x220 - Configuration for EMC_CS0."]
    pub staticconfig1: STATICCONFIG,
    #[doc = "0x224 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen1: STATICWAITWEN,
    #[doc = "0x228 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen1: STATICWAITOEN,
    #[doc = "0x22c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd1: STATICWAITRD,
    #[doc = "0x230 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage1: STATICWAITPAGE,
    #[doc = "0x234 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr1: STATICWAITWR,
    #[doc = "0x238 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn1: STATICWAITTURN,
    _reserved40: [u8; 4usize],
    #[doc = "0x240 - Configuration for EMC_CS0."]
    pub staticconfig2: STATICCONFIG,
    #[doc = "0x244 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen2: STATICWAITWEN,
    #[doc = "0x248 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen2: STATICWAITOEN,
    #[doc = "0x24c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd2: STATICWAITRD,
    #[doc = "0x250 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage2: STATICWAITPAGE,
    #[doc = "0x254 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr2: STATICWAITWR,
    #[doc = "0x258 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn2: STATICWAITTURN,
    _reserved47: [u8; 4usize],
    #[doc = "0x260 - Configuration for EMC_CS0."]
    pub staticconfig3: STATICCONFIG,
    #[doc = "0x264 - Delay from EMC_CS0 to write enable."]
    pub staticwaitwen3: STATICWAITWEN,
    #[doc = "0x268 - Delay from EMC_CS0 or address change, whichever is later, to output enable."]
    pub staticwaitoen3: STATICWAITOEN,
    #[doc = "0x26c - Delay from EMC_CS0 to a read access."]
    pub staticwaitrd3: STATICWAITRD,
    #[doc = "0x270 - Delay for asynchronous page mode sequential accesses for EMC_CS0."]
    pub staticwaitpage3: STATICWAITPAGE,
    #[doc = "0x274 - Delay from EMC_CS0 to a write access."]
    pub staticwaitwr3: STATICWAITWR,
    #[doc = "0x278 - Number of bus turnaround cycles EMC_CS0."]
    pub staticwaitturn3: STATICWAITTURN,
}
#[doc = "Controls operation of the memory controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Controls operation of the memory controller."]
pub mod control;
#[doc = "Provides EMC status information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Provides EMC status information."]
pub mod status;
#[doc = "Configures operation of the memory controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configures operation of the memory controller"]
pub mod config;
#[doc = "Controls dynamic memory operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamiccontrol](dynamiccontrol) module"]
pub type DYNAMICCONTROL = crate::Reg<u32, _DYNAMICCONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICCONTROL;
#[doc = "`read()` method returns [dynamiccontrol::R](dynamiccontrol::R) reader structure"]
impl crate::Readable for DYNAMICCONTROL {}
#[doc = "`write(|w| ..)` method takes [dynamiccontrol::W](dynamiccontrol::W) writer structure"]
impl crate::Writable for DYNAMICCONTROL {}
#[doc = "Controls dynamic memory operation."]
pub mod dynamiccontrol;
#[doc = "Configures dynamic memory refresh.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrefresh](dynamicrefresh) module"]
pub type DYNAMICREFRESH = crate::Reg<u32, _DYNAMICREFRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICREFRESH;
#[doc = "`read()` method returns [dynamicrefresh::R](dynamicrefresh::R) reader structure"]
impl crate::Readable for DYNAMICREFRESH {}
#[doc = "`write(|w| ..)` method takes [dynamicrefresh::W](dynamicrefresh::W) writer structure"]
impl crate::Writable for DYNAMICREFRESH {}
#[doc = "Configures dynamic memory refresh."]
pub mod dynamicrefresh;
#[doc = "Configures dynamic memory read strategy.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicreadconfig](dynamicreadconfig) module"]
pub type DYNAMICREADCONFIG = crate::Reg<u32, _DYNAMICREADCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICREADCONFIG;
#[doc = "`read()` method returns [dynamicreadconfig::R](dynamicreadconfig::R) reader structure"]
impl crate::Readable for DYNAMICREADCONFIG {}
#[doc = "`write(|w| ..)` method takes [dynamicreadconfig::W](dynamicreadconfig::W) writer structure"]
impl crate::Writable for DYNAMICREADCONFIG {}
#[doc = "Configures dynamic memory read strategy."]
pub mod dynamicreadconfig;
#[doc = "Precharge command period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrp](dynamicrp) module"]
pub type DYNAMICRP = crate::Reg<u32, _DYNAMICRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRP;
#[doc = "`read()` method returns [dynamicrp::R](dynamicrp::R) reader structure"]
impl crate::Readable for DYNAMICRP {}
#[doc = "`write(|w| ..)` method takes [dynamicrp::W](dynamicrp::W) writer structure"]
impl crate::Writable for DYNAMICRP {}
#[doc = "Precharge command period."]
pub mod dynamicrp;
#[doc = "Active to precharge command period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicras](dynamicras) module"]
pub type DYNAMICRAS = crate::Reg<u32, _DYNAMICRAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRAS;
#[doc = "`read()` method returns [dynamicras::R](dynamicras::R) reader structure"]
impl crate::Readable for DYNAMICRAS {}
#[doc = "`write(|w| ..)` method takes [dynamicras::W](dynamicras::W) writer structure"]
impl crate::Writable for DYNAMICRAS {}
#[doc = "Active to precharge command period."]
pub mod dynamicras;
#[doc = "Self-refresh exit time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicsrex](dynamicsrex) module"]
pub type DYNAMICSREX = crate::Reg<u32, _DYNAMICSREX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICSREX;
#[doc = "`read()` method returns [dynamicsrex::R](dynamicsrex::R) reader structure"]
impl crate::Readable for DYNAMICSREX {}
#[doc = "`write(|w| ..)` method takes [dynamicsrex::W](dynamicsrex::W) writer structure"]
impl crate::Writable for DYNAMICSREX {}
#[doc = "Self-refresh exit time."]
pub mod dynamicsrex;
#[doc = "Last-data-out to active command time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicapr](dynamicapr) module"]
pub type DYNAMICAPR = crate::Reg<u32, _DYNAMICAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICAPR;
#[doc = "`read()` method returns [dynamicapr::R](dynamicapr::R) reader structure"]
impl crate::Readable for DYNAMICAPR {}
#[doc = "`write(|w| ..)` method takes [dynamicapr::W](dynamicapr::W) writer structure"]
impl crate::Writable for DYNAMICAPR {}
#[doc = "Last-data-out to active command time."]
pub mod dynamicapr;
#[doc = "Data-in to active command time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicdal](dynamicdal) module"]
pub type DYNAMICDAL = crate::Reg<u32, _DYNAMICDAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICDAL;
#[doc = "`read()` method returns [dynamicdal::R](dynamicdal::R) reader structure"]
impl crate::Readable for DYNAMICDAL {}
#[doc = "`write(|w| ..)` method takes [dynamicdal::W](dynamicdal::W) writer structure"]
impl crate::Writable for DYNAMICDAL {}
#[doc = "Data-in to active command time."]
pub mod dynamicdal;
#[doc = "Write recovery time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicwr](dynamicwr) module"]
pub type DYNAMICWR = crate::Reg<u32, _DYNAMICWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICWR;
#[doc = "`read()` method returns [dynamicwr::R](dynamicwr::R) reader structure"]
impl crate::Readable for DYNAMICWR {}
#[doc = "`write(|w| ..)` method takes [dynamicwr::W](dynamicwr::W) writer structure"]
impl crate::Writable for DYNAMICWR {}
#[doc = "Write recovery time."]
pub mod dynamicwr;
#[doc = "Selects the active to active command period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrc](dynamicrc) module"]
pub type DYNAMICRC = crate::Reg<u32, _DYNAMICRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRC;
#[doc = "`read()` method returns [dynamicrc::R](dynamicrc::R) reader structure"]
impl crate::Readable for DYNAMICRC {}
#[doc = "`write(|w| ..)` method takes [dynamicrc::W](dynamicrc::W) writer structure"]
impl crate::Writable for DYNAMICRC {}
#[doc = "Selects the active to active command period."]
pub mod dynamicrc;
#[doc = "Selects the auto-refresh period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrfc](dynamicrfc) module"]
pub type DYNAMICRFC = crate::Reg<u32, _DYNAMICRFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRFC;
#[doc = "`read()` method returns [dynamicrfc::R](dynamicrfc::R) reader structure"]
impl crate::Readable for DYNAMICRFC {}
#[doc = "`write(|w| ..)` method takes [dynamicrfc::W](dynamicrfc::W) writer structure"]
impl crate::Writable for DYNAMICRFC {}
#[doc = "Selects the auto-refresh period."]
pub mod dynamicrfc;
#[doc = "Time for exit self-refresh to active command.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicxsr](dynamicxsr) module"]
pub type DYNAMICXSR = crate::Reg<u32, _DYNAMICXSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICXSR;
#[doc = "`read()` method returns [dynamicxsr::R](dynamicxsr::R) reader structure"]
impl crate::Readable for DYNAMICXSR {}
#[doc = "`write(|w| ..)` method takes [dynamicxsr::W](dynamicxsr::W) writer structure"]
impl crate::Writable for DYNAMICXSR {}
#[doc = "Time for exit self-refresh to active command."]
pub mod dynamicxsr;
#[doc = "Latency for active bank A to active bank B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrrd](dynamicrrd) module"]
pub type DYNAMICRRD = crate::Reg<u32, _DYNAMICRRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRRD;
#[doc = "`read()` method returns [dynamicrrd::R](dynamicrrd::R) reader structure"]
impl crate::Readable for DYNAMICRRD {}
#[doc = "`write(|w| ..)` method takes [dynamicrrd::W](dynamicrrd::W) writer structure"]
impl crate::Writable for DYNAMICRRD {}
#[doc = "Latency for active bank A to active bank B."]
pub mod dynamicrrd;
#[doc = "Time for load mode register to active command.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicmrd](dynamicmrd) module"]
pub type DYNAMICMRD = crate::Reg<u32, _DYNAMICMRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICMRD;
#[doc = "`read()` method returns [dynamicmrd::R](dynamicmrd::R) reader structure"]
impl crate::Readable for DYNAMICMRD {}
#[doc = "`write(|w| ..)` method takes [dynamicmrd::W](dynamicmrd::W) writer structure"]
impl crate::Writable for DYNAMICMRD {}
#[doc = "Time for load mode register to active command."]
pub mod dynamicmrd;
#[doc = "Time for long static memory read and write transfers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticextendedwait](staticextendedwait) module"]
pub type STATICEXTENDEDWAIT = crate::Reg<u32, _STATICEXTENDEDWAIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICEXTENDEDWAIT;
#[doc = "`read()` method returns [staticextendedwait::R](staticextendedwait::R) reader structure"]
impl crate::Readable for STATICEXTENDEDWAIT {}
#[doc = "`write(|w| ..)` method takes [staticextendedwait::W](staticextendedwait::W) writer structure"]
impl crate::Writable for STATICEXTENDEDWAIT {}
#[doc = "Time for long static memory read and write transfers."]
pub mod staticextendedwait;
#[doc = "Configuration information for EMC_DYCS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicconfig](dynamicconfig) module"]
pub type DYNAMICCONFIG = crate::Reg<u32, _DYNAMICCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICCONFIG;
#[doc = "`read()` method returns [dynamicconfig::R](dynamicconfig::R) reader structure"]
impl crate::Readable for DYNAMICCONFIG {}
#[doc = "`write(|w| ..)` method takes [dynamicconfig::W](dynamicconfig::W) writer structure"]
impl crate::Writable for DYNAMICCONFIG {}
#[doc = "Configuration information for EMC_DYCS0."]
pub mod dynamicconfig;
#[doc = "RAS and CAS latencies for EMC_DYCS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrascas](dynamicrascas) module"]
pub type DYNAMICRASCAS = crate::Reg<u32, _DYNAMICRASCAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRASCAS;
#[doc = "`read()` method returns [dynamicrascas::R](dynamicrascas::R) reader structure"]
impl crate::Readable for DYNAMICRASCAS {}
#[doc = "`write(|w| ..)` method takes [dynamicrascas::W](dynamicrascas::W) writer structure"]
impl crate::Writable for DYNAMICRASCAS {}
#[doc = "RAS and CAS latencies for EMC_DYCS0."]
pub mod dynamicrascas;
#[doc = "Configuration for EMC_CS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticconfig](staticconfig) module"]
pub type STATICCONFIG = crate::Reg<u32, _STATICCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICCONFIG;
#[doc = "`read()` method returns [staticconfig::R](staticconfig::R) reader structure"]
impl crate::Readable for STATICCONFIG {}
#[doc = "`write(|w| ..)` method takes [staticconfig::W](staticconfig::W) writer structure"]
impl crate::Writable for STATICCONFIG {}
#[doc = "Configuration for EMC_CS0."]
pub mod staticconfig;
#[doc = "Delay from EMC_CS0 to write enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitwen](staticwaitwen) module"]
pub type STATICWAITWEN = crate::Reg<u32, _STATICWAITWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITWEN;
#[doc = "`read()` method returns [staticwaitwen::R](staticwaitwen::R) reader structure"]
impl crate::Readable for STATICWAITWEN {}
#[doc = "`write(|w| ..)` method takes [staticwaitwen::W](staticwaitwen::W) writer structure"]
impl crate::Writable for STATICWAITWEN {}
#[doc = "Delay from EMC_CS0 to write enable."]
pub mod staticwaitwen;
#[doc = "Delay from EMC_CS0 or address change, whichever is later, to output enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitoen](staticwaitoen) module"]
pub type STATICWAITOEN = crate::Reg<u32, _STATICWAITOEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITOEN;
#[doc = "`read()` method returns [staticwaitoen::R](staticwaitoen::R) reader structure"]
impl crate::Readable for STATICWAITOEN {}
#[doc = "`write(|w| ..)` method takes [staticwaitoen::W](staticwaitoen::W) writer structure"]
impl crate::Writable for STATICWAITOEN {}
#[doc = "Delay from EMC_CS0 or address change, whichever is later, to output enable."]
pub mod staticwaitoen;
#[doc = "Delay from EMC_CS0 to a read access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitrd](staticwaitrd) module"]
pub type STATICWAITRD = crate::Reg<u32, _STATICWAITRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITRD;
#[doc = "`read()` method returns [staticwaitrd::R](staticwaitrd::R) reader structure"]
impl crate::Readable for STATICWAITRD {}
#[doc = "`write(|w| ..)` method takes [staticwaitrd::W](staticwaitrd::W) writer structure"]
impl crate::Writable for STATICWAITRD {}
#[doc = "Delay from EMC_CS0 to a read access."]
pub mod staticwaitrd;
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitpage](staticwaitpage) module"]
pub type STATICWAITPAGE = crate::Reg<u32, _STATICWAITPAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITPAGE;
#[doc = "`read()` method returns [staticwaitpage::R](staticwaitpage::R) reader structure"]
impl crate::Readable for STATICWAITPAGE {}
#[doc = "`write(|w| ..)` method takes [staticwaitpage::W](staticwaitpage::W) writer structure"]
impl crate::Writable for STATICWAITPAGE {}
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CS0."]
pub mod staticwaitpage;
#[doc = "Delay from EMC_CS0 to a write access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitwr](staticwaitwr) module"]
pub type STATICWAITWR = crate::Reg<u32, _STATICWAITWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITWR;
#[doc = "`read()` method returns [staticwaitwr::R](staticwaitwr::R) reader structure"]
impl crate::Readable for STATICWAITWR {}
#[doc = "`write(|w| ..)` method takes [staticwaitwr::W](staticwaitwr::W) writer structure"]
impl crate::Writable for STATICWAITWR {}
#[doc = "Delay from EMC_CS0 to a write access."]
pub mod staticwaitwr;
#[doc = "Number of bus turnaround cycles EMC_CS0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitturn](staticwaitturn) module"]
pub type STATICWAITTURN = crate::Reg<u32, _STATICWAITTURN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICWAITTURN;
#[doc = "`read()` method returns [staticwaitturn::R](staticwaitturn::R) reader structure"]
impl crate::Readable for STATICWAITTURN {}
#[doc = "`write(|w| ..)` method takes [staticwaitturn::W](staticwaitturn::W) writer structure"]
impl crate::Writable for STATICWAITTURN {}
#[doc = "Number of bus turnaround cycles EMC_CS0."]
pub mod staticwaitturn;
