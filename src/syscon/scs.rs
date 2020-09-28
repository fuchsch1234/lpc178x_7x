#[doc = "Reader of register SCS"]
pub type R = crate::R<u32, super::SCS>;
#[doc = "Writer for register SCS"]
pub type W = crate::W<u32, super::SCS>;
#[doc = "Register SCS `reset()`'s with value 0"]
impl crate::ResetValue for super::SCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCSC_A {
    #[doc = "0: Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    STATIC_MEMORY_ADDRESS_SHIFT = 0,
    #[doc = "1: Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    STATIC_MEMORY_ADDRESS = 1,
}
impl From<EMCSC_A> for bool {
    #[inline(always)]
    fn from(variant: EMCSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMCSC`"]
pub type EMCSC_R = crate::R<bool, EMCSC_A>;
impl EMCSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCSC_A {
        match self.bits {
            false => EMCSC_A::STATIC_MEMORY_ADDRESS_SHIFT,
            true => EMCSC_A::STATIC_MEMORY_ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC_MEMORY_ADDRESS_SHIFT`"]
    #[inline(always)]
    pub fn is_static_memory_address_shift(&self) -> bool {
        *self == EMCSC_A::STATIC_MEMORY_ADDRESS_SHIFT
    }
    #[doc = "Checks if the value of the field is `STATIC_MEMORY_ADDRESS`"]
    #[inline(always)]
    pub fn is_static_memory_address(&self) -> bool {
        *self == EMCSC_A::STATIC_MEMORY_ADDRESS
    }
}
#[doc = "Write proxy for field `EMCSC`"]
pub struct EMCSC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    #[inline(always)]
    pub fn static_memory_address_shift(self) -> &'a mut W {
        self.variant(EMCSC_A::STATIC_MEMORY_ADDRESS_SHIFT)
    }
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    #[inline(always)]
    pub fn static_memory_address(self) -> &'a mut W {
        self.variant(EMCSC_A::STATIC_MEMORY_ADDRESS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCRD_A {
    #[doc = "0: Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    BOTH_EMC_RESETS_ARE_ = 0,
    #[doc = "1: Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    MANY_PORTIONS_OF_THE = 1,
}
impl From<EMCRD_A> for bool {
    #[inline(always)]
    fn from(variant: EMCRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMCRD`"]
pub type EMCRD_R = crate::R<bool, EMCRD_A>;
impl EMCRD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCRD_A {
        match self.bits {
            false => EMCRD_A::BOTH_EMC_RESETS_ARE_,
            true => EMCRD_A::MANY_PORTIONS_OF_THE,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH_EMC_RESETS_ARE_`"]
    #[inline(always)]
    pub fn is_both_emc_resets_are_(&self) -> bool {
        *self == EMCRD_A::BOTH_EMC_RESETS_ARE_
    }
    #[doc = "Checks if the value of the field is `MANY_PORTIONS_OF_THE`"]
    #[inline(always)]
    pub fn is_many_portions_of_the(&self) -> bool {
        *self == EMCRD_A::MANY_PORTIONS_OF_THE
    }
}
#[doc = "Write proxy for field `EMCRD`"]
pub struct EMCRD_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCRD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    #[inline(always)]
    pub fn both_emc_resets_are_(self) -> &'a mut W {
        self.variant(EMCRD_A::BOTH_EMC_RESETS_ARE_)
    }
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    #[inline(always)]
    pub fn many_portions_of_the(self) -> &'a mut W {
        self.variant(EMCRD_A::MANY_PORTIONS_OF_THE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "External Memory Controller burst control. Also see Section 10.10 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCBC_A {
    #[doc = "0: Burst enabled."]
    BURST_ENABLED_ = 0,
    #[doc = "1: Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    BURST_DISABLED_THIS = 1,
}
impl From<EMCBC_A> for bool {
    #[inline(always)]
    fn from(variant: EMCBC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMCBC`"]
pub type EMCBC_R = crate::R<bool, EMCBC_A>;
impl EMCBC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCBC_A {
        match self.bits {
            false => EMCBC_A::BURST_ENABLED_,
            true => EMCBC_A::BURST_DISABLED_THIS,
        }
    }
    #[doc = "Checks if the value of the field is `BURST_ENABLED_`"]
    #[inline(always)]
    pub fn is_burst_enabled_(&self) -> bool {
        *self == EMCBC_A::BURST_ENABLED_
    }
    #[doc = "Checks if the value of the field is `BURST_DISABLED_THIS`"]
    #[inline(always)]
    pub fn is_burst_disabled_this(&self) -> bool {
        *self == EMCBC_A::BURST_DISABLED_THIS
    }
}
#[doc = "Write proxy for field `EMCBC`"]
pub struct EMCBC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCBC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Burst enabled."]
    #[inline(always)]
    pub fn burst_enabled_(self) -> &'a mut W {
        self.variant(EMCBC_A::BURST_ENABLED_)
    }
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    #[inline(always)]
    pub fn burst_disabled_this(self) -> &'a mut W {
        self.variant(EMCBC_A::BURST_DISABLED_THIS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCIPWRAL_A {
    #[doc = "0: SD_PWR is active low (inverted output of the SD Card interface block)."]
    SD_PWR_IS_ACTIVE_LOW = 0,
    #[doc = "1: SD_PWR is active high (follows the output of the SD Card interface block)."]
    SD_PWR_IS_ACTIVE_HIG = 1,
}
impl From<MCIPWRAL_A> for bool {
    #[inline(always)]
    fn from(variant: MCIPWRAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCIPWRAL`"]
pub type MCIPWRAL_R = crate::R<bool, MCIPWRAL_A>;
impl MCIPWRAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCIPWRAL_A {
        match self.bits {
            false => MCIPWRAL_A::SD_PWR_IS_ACTIVE_LOW,
            true => MCIPWRAL_A::SD_PWR_IS_ACTIVE_HIG,
        }
    }
    #[doc = "Checks if the value of the field is `SD_PWR_IS_ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_low(&self) -> bool {
        *self == MCIPWRAL_A::SD_PWR_IS_ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `SD_PWR_IS_ACTIVE_HIG`"]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_hig(&self) -> bool {
        *self == MCIPWRAL_A::SD_PWR_IS_ACTIVE_HIG
    }
}
#[doc = "Write proxy for field `MCIPWRAL`"]
pub struct MCIPWRAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCIPWRAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCIPWRAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_low(self) -> &'a mut W {
        self.variant(MCIPWRAL_A::SD_PWR_IS_ACTIVE_LOW)
    }
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_hig(self) -> &'a mut W {
        self.variant(MCIPWRAL_A::SD_PWR_IS_ACTIVE_HIG)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRS_A {
    #[doc = "0: The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    FREQUENCY_RANGE_1_20_MHZ = 0,
    #[doc = "1: The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    FREQUENCY_RANGE_15_25_MHZ = 1,
}
impl From<OSCRS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCRS`"]
pub type OSCRS_R = crate::R<bool, OSCRS_A>;
impl OSCRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCRS_A {
        match self.bits {
            false => OSCRS_A::FREQUENCY_RANGE_1_20_MHZ,
            true => OSCRS_A::FREQUENCY_RANGE_15_25_MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `FREQUENCY_RANGE_1_20_MHZ`"]
    #[inline(always)]
    pub fn is_frequency_range_1_20_mhz(&self) -> bool {
        *self == OSCRS_A::FREQUENCY_RANGE_1_20_MHZ
    }
    #[doc = "Checks if the value of the field is `FREQUENCY_RANGE_15_25_MHZ`"]
    #[inline(always)]
    pub fn is_frequency_range_15_25_mhz(&self) -> bool {
        *self == OSCRS_A::FREQUENCY_RANGE_15_25_MHZ
    }
}
#[doc = "Write proxy for field `OSCRS`"]
pub struct OSCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn frequency_range_1_20_mhz(self) -> &'a mut W {
        self.variant(OSCRS_A::FREQUENCY_RANGE_1_20_MHZ)
    }
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn frequency_range_15_25_mhz(self) -> &'a mut W {
        self.variant(OSCRS_A::FREQUENCY_RANGE_15_25_MHZ)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCEN_A {
    #[doc = "0: The main oscillator is disabled."]
    DISABLED = 0,
    #[doc = "1: The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED = 1,
}
impl From<OSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCEN`"]
pub type OSCEN_R = crate::R<bool, OSCEN_A>;
impl OSCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCEN_A {
        match self.bits {
            false => OSCEN_A::DISABLED,
            true => OSCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `OSCEN`"]
pub struct OSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The main oscillator is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSCEN_A::DISABLED)
    }
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSCEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTAT_A {
    #[doc = "0: The main oscillator is not ready to be used as a clock source."]
    NOT_READY = 0,
    #[doc = "1: The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY = 1,
}
impl From<OSCSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCSTAT`"]
pub type OSCSTAT_R = crate::R<bool, OSCSTAT_A>;
impl OSCSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSTAT_A {
        match self.bits {
            false => OSCSTAT_A::NOT_READY,
            true => OSCSTAT_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == OSCSTAT_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == OSCSTAT_A::READY
    }
}
#[doc = "Write proxy for field `OSCSTAT`"]
pub struct OSCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(OSCSTAT_A::NOT_READY)
    }
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(OSCSTAT_A::READY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
    #[inline(always)]
    pub fn emcsc(&self) -> EMCSC_R {
        EMCSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
    #[inline(always)]
    pub fn emcrd(&self) -> EMCRD_R {
        EMCRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
    #[inline(always)]
    pub fn emcbc(&self) -> EMCBC_R {
        EMCBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline(always)]
    pub fn mcipwral(&self) -> MCIPWRAL_R {
        MCIPWRAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrs(&self) -> OSCRS_R {
        OSCRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OSCSTAT_R {
        OSCSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
    #[inline(always)]
    pub fn emcsc(&mut self) -> EMCSC_W {
        EMCSC_W { w: self }
    }
    #[doc = "Bit 1 - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
    #[inline(always)]
    pub fn emcrd(&mut self) -> EMCRD_W {
        EMCRD_W { w: self }
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
    #[inline(always)]
    pub fn emcbc(&mut self) -> EMCBC_W {
        EMCBC_W { w: self }
    }
    #[doc = "Bit 3 - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline(always)]
    pub fn mcipwral(&mut self) -> MCIPWRAL_W {
        MCIPWRAL_W { w: self }
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrs(&mut self) -> OSCRS_W {
        OSCRS_W { w: self }
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&mut self) -> OSCEN_W {
        OSCEN_W { w: self }
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&mut self) -> OSCSTAT_W {
        OSCSTAT_W { w: self }
    }
}
