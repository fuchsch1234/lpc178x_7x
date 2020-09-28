#[doc = "Reader of register CLKOUTCFG"]
pub type R = crate::R<u32, super::CLKOUTCFG>;
#[doc = "Writer for register CLKOUTCFG"]
pub type W = crate::W<u32, super::CLKOUTCFG>;
#[doc = "Register CLKOUTCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKOUTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CLKOUTDIV`"]
pub type CLKOUTDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKOUTDIV`"]
pub struct CLKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLKOUT_EN`"]
pub type CLKOUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOUT_EN`"]
pub struct CLKOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_EN_W<'a> {
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
#[doc = "Reader of field `CLKOUT_ACT`"]
pub type CLKOUT_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOUT_ACT`"]
pub struct CLKOUT_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_ACT_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. 0x0 = Selects the CPU clock as the CLKOUT source. 0x1 = Selects the main oscillator as the CLKOUT source. 0x2 = Selects the Internal RC oscillator as the CLKOUT source. 0x3 = Selects the USB clock as the CLKOUT source. 0x4 = Selects the RTC oscillator as the CLKOUT source. 0x5 = Selects the SPIFI clock as the CLKOUT source. 0x6 = Selects the Watchdog oscillator as the CLKOUT source. Other settings are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0x0 = Clock is divided by 1. 0x1 = Clock is divided by 2. 0x2 = Clock is divided by 3. ... 0xF = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> CLKOUTDIV_R {
        CLKOUTDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&self) -> CLKOUT_EN_R {
        CLKOUT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&self) -> CLKOUT_ACT_R {
        CLKOUT_ACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. 0x0 = Selects the CPU clock as the CLKOUT source. 0x1 = Selects the main oscillator as the CLKOUT source. 0x2 = Selects the Internal RC oscillator as the CLKOUT source. 0x3 = Selects the USB clock as the CLKOUT source. 0x4 = Selects the RTC oscillator as the CLKOUT source. 0x5 = Selects the SPIFI clock as the CLKOUT source. 0x6 = Selects the Watchdog oscillator as the CLKOUT source. Other settings are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0x0 = Clock is divided by 1. 0x1 = Clock is divided by 2. 0x2 = Clock is divided by 3. ... 0xF = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&mut self) -> CLKOUTDIV_W {
        CLKOUTDIV_W { w: self }
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&mut self) -> CLKOUT_EN_W {
        CLKOUT_EN_W { w: self }
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&mut self) -> CLKOUT_ACT_W {
        CLKOUT_ACT_W { w: self }
    }
}
