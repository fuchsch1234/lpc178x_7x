#[doc = "Reader of register CLOCK"]
pub type R = crate::R<u32, super::CLOCK>;
#[doc = "Writer for register CLOCK"]
pub type W = crate::W<u32, super::CLOCK>;
#[doc = "Register CLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Enable SD card bus clock:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Clock disabled."]
    CLOCK_DISABLED_ = 0,
    #[doc = "1: Clock enabled."]
    CLOCK_ENABLED_ = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::CLOCK_DISABLED_,
            true => ENABLE_A::CLOCK_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_DISABLED_`"]
    #[inline(always)]
    pub fn is_clock_disabled_(&self) -> bool {
        *self == ENABLE_A::CLOCK_DISABLED_
    }
    #[doc = "Checks if the value of the field is `CLOCK_ENABLED_`"]
    #[inline(always)]
    pub fn is_clock_enabled_(&self) -> bool {
        *self == ENABLE_A::CLOCK_ENABLED_
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled."]
    #[inline(always)]
    pub fn clock_disabled_(self) -> &'a mut W {
        self.variant(ENABLE_A::CLOCK_DISABLED_)
    }
    #[doc = "Clock enabled."]
    #[inline(always)]
    pub fn clock_enabled_(self) -> &'a mut W {
        self.variant(ENABLE_A::CLOCK_ENABLED_)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Disable SD_CLK output when bus is idle:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSAVE_A {
    #[doc = "0: Always enabled."]
    ALWAYS_ENABLED_ = 0,
    #[doc = "1: Clock enabled when bus is active."]
    CLOCK_ENABLED_WHEN_B = 1,
}
impl From<PWRSAVE_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRSAVE`"]
pub type PWRSAVE_R = crate::R<bool, PWRSAVE_A>;
impl PWRSAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSAVE_A {
        match self.bits {
            false => PWRSAVE_A::ALWAYS_ENABLED_,
            true => PWRSAVE_A::CLOCK_ENABLED_WHEN_B,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ENABLED_`"]
    #[inline(always)]
    pub fn is_always_enabled_(&self) -> bool {
        *self == PWRSAVE_A::ALWAYS_ENABLED_
    }
    #[doc = "Checks if the value of the field is `CLOCK_ENABLED_WHEN_B`"]
    #[inline(always)]
    pub fn is_clock_enabled_when_b(&self) -> bool {
        *self == PWRSAVE_A::CLOCK_ENABLED_WHEN_B
    }
}
#[doc = "Write proxy for field `PWRSAVE`"]
pub struct PWRSAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Always enabled."]
    #[inline(always)]
    pub fn always_enabled_(self) -> &'a mut W {
        self.variant(PWRSAVE_A::ALWAYS_ENABLED_)
    }
    #[doc = "Clock enabled when bus is active."]
    #[inline(always)]
    pub fn clock_enabled_when_b(self) -> &'a mut W {
        self.variant(PWRSAVE_A::CLOCK_ENABLED_WHEN_B)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable bypass of clock divide logic:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Disable bypass."]
    DISABLE_BYPASS_ = 0,
    #[doc = "1: Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    ENABLE_BYPASS_MCLK_ = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLE_BYPASS_,
            true => BYPASS_A::ENABLE_BYPASS_MCLK_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_BYPASS_`"]
    #[inline(always)]
    pub fn is_disable_bypass_(&self) -> bool {
        *self == BYPASS_A::DISABLE_BYPASS_
    }
    #[doc = "Checks if the value of the field is `ENABLE_BYPASS_MCLK_`"]
    #[inline(always)]
    pub fn is_enable_bypass_mclk_(&self) -> bool {
        *self == BYPASS_A::ENABLE_BYPASS_MCLK_
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable bypass."]
    #[inline(always)]
    pub fn disable_bypass_(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLE_BYPASS_)
    }
    #[doc = "Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    #[inline(always)]
    pub fn enable_bypass_mclk_(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLE_BYPASS_MCLK_)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Enable wide bus mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDEBUS_A {
    #[doc = "0: Standard bus mode (only SD_DAT\\[0\\]
used)."]
    STANDARD_BUS_MODE_O = 0,
    #[doc = "1: Wide bus mode (SD_DAT\\[3:0\\]
used)"]
    WIDE_BUS_MODE_SD_DA = 1,
}
impl From<WIDEBUS_A> for bool {
    #[inline(always)]
    fn from(variant: WIDEBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIDEBUS`"]
pub type WIDEBUS_R = crate::R<bool, WIDEBUS_A>;
impl WIDEBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIDEBUS_A {
        match self.bits {
            false => WIDEBUS_A::STANDARD_BUS_MODE_O,
            true => WIDEBUS_A::WIDE_BUS_MODE_SD_DA,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_BUS_MODE_O`"]
    #[inline(always)]
    pub fn is_standard_bus_mode_o(&self) -> bool {
        *self == WIDEBUS_A::STANDARD_BUS_MODE_O
    }
    #[doc = "Checks if the value of the field is `WIDE_BUS_MODE_SD_DA`"]
    #[inline(always)]
    pub fn is_wide_bus_mode_sd_da(&self) -> bool {
        *self == WIDEBUS_A::WIDE_BUS_MODE_SD_DA
    }
}
#[doc = "Write proxy for field `WIDEBUS`"]
pub struct WIDEBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDEBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDEBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard bus mode (only SD_DAT\\[0\\]
used)."]
    #[inline(always)]
    pub fn standard_bus_mode_o(self) -> &'a mut W {
        self.variant(WIDEBUS_A::STANDARD_BUS_MODE_O)
    }
    #[doc = "Wide bus mode (SD_DAT\\[3:0\\]
used)"]
    #[inline(always)]
    pub fn wide_bus_mode_sd_da(self) -> &'a mut W {
        self.variant(WIDEBUS_A::WIDE_BUS_MODE_SD_DA)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bus clock period: SD_CLK frequency = MCLK / \\[2x(ClkDiv+1)\\]."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable SD card bus clock:"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable SD_CLK output when bus is idle:"]
    #[inline(always)]
    pub fn pwrsave(&self) -> PWRSAVE_R {
        PWRSAVE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable bypass of clock divide logic:"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable wide bus mode."]
    #[inline(always)]
    pub fn widebus(&self) -> WIDEBUS_R {
        WIDEBUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bus clock period: SD_CLK frequency = MCLK / \\[2x(ClkDiv+1)\\]."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 8 - Enable SD card bus clock:"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 9 - Disable SD_CLK output when bus is idle:"]
    #[inline(always)]
    pub fn pwrsave(&mut self) -> PWRSAVE_W {
        PWRSAVE_W { w: self }
    }
    #[doc = "Bit 10 - Enable bypass of clock divide logic:"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 11 - Enable wide bus mode."]
    #[inline(always)]
    pub fn widebus(&mut self) -> WIDEBUS_W {
        WIDEBUS_W { w: self }
    }
}
