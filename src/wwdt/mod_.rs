#[doc = "Reader of register MOD"]
pub type R = crate::R<u32, super::MOD>;
#[doc = "Writer for register MOD"]
pub type W = crate::W<u32, super::MOD>;
#[doc = "Register MOD `reset()`'s with value 0"]
impl crate::ResetValue for super::MOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Watchdog enable bit. This bit is Set Only. See Table 652.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDEN_A {
    #[doc = "0: The watchdog timer is stopped."]
    STOP = 0,
    #[doc = "1: The watchdog timer is running."]
    RUN = 1,
}
impl From<WDEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDEN`"]
pub type WDEN_R = crate::R<bool, WDEN_A>;
impl WDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDEN_A {
        match self.bits {
            false => WDEN_A::STOP,
            true => WDEN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == WDEN_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WDEN_A::RUN
    }
}
#[doc = "Write proxy for field `WDEN`"]
pub struct WDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(WDEN_A::STOP)
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(WDEN_A::RUN)
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
#[doc = "Watchdog reset enable bit. This bit is Set Only. See Table 652.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESET_A {
    #[doc = "0: A watchdog timeout will not cause a chip reset."]
    NORESET = 0,
    #[doc = "1: A watchdog timeout will cause a chip reset."]
    RESET = 1,
}
impl From<WDRESET_A> for bool {
    #[inline(always)]
    fn from(variant: WDRESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDRESET`"]
pub type WDRESET_R = crate::R<bool, WDRESET_A>;
impl WDRESET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDRESET_A {
        match self.bits {
            false => WDRESET_A::NORESET,
            true => WDRESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_noreset(&self) -> bool {
        *self == WDRESET_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDRESET_A::RESET
    }
}
#[doc = "Write proxy for field `WDRESET`"]
pub struct WDRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDRESET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn noreset(self) -> &'a mut W {
        self.variant(WDRESET_A::NORESET)
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDRESET_A::RESET)
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
#[doc = "Reader of field `WDTOF`"]
pub type WDTOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTOF`"]
pub struct WDTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTOF_W<'a> {
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
#[doc = "Reader of field `WDINT`"]
pub type WDINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDINT`"]
pub struct WDINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDINT_W<'a> {
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
#[doc = "Watchdog update mode. This bit is Set Only. See Section WDPROTECT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECT_A {
    #[doc = "0: The watchdog reload value (WDTC) can be changed at any time."]
    CHANGE = 0,
    #[doc = "1: The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1."]
    CHANGE_W_CNT = 1,
}
impl From<WDPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: WDPROTECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDPROTECT`"]
pub type WDPROTECT_R = crate::R<bool, WDPROTECT_A>;
impl WDPROTECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDPROTECT_A {
        match self.bits {
            false => WDPROTECT_A::CHANGE,
            true => WDPROTECT_A::CHANGE_W_CNT,
        }
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == WDPROTECT_A::CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE_W_CNT`"]
    #[inline(always)]
    pub fn is_change_w_cnt(&self) -> bool {
        *self == WDPROTECT_A::CHANGE_W_CNT
    }
}
#[doc = "Write proxy for field `WDPROTECT`"]
pub struct WDPROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDPROTECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDPROTECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The watchdog reload value (WDTC) can be changed at any time."]
    #[inline(always)]
    pub fn change(self) -> &'a mut W {
        self.variant(WDPROTECT_A::CHANGE)
    }
    #[doc = "The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1."]
    #[inline(always)]
    pub fn change_w_cnt(self) -> &'a mut W {
        self.variant(WDPROTECT_A::CHANGE_W_CNT)
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
impl R {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    pub fn wdreset(&self) -> WDRESET_R {
        WDRESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1. See Section WDTOF."]
    #[inline(always)]
    pub fn wdtof(&self) -> WDTOF_R {
        WDTOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software. See Section WDINT."]
    #[inline(always)]
    pub fn wdint(&self) -> WDINT_R {
        WDINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only. See Section WDPROTECT."]
    #[inline(always)]
    pub fn wdprotect(&self) -> WDPROTECT_R {
        WDPROTECT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    pub fn wden(&mut self) -> WDEN_W {
        WDEN_W { w: self }
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    pub fn wdreset(&mut self) -> WDRESET_W {
        WDRESET_W { w: self }
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1. See Section WDTOF."]
    #[inline(always)]
    pub fn wdtof(&mut self) -> WDTOF_W {
        WDTOF_W { w: self }
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software. See Section WDINT."]
    #[inline(always)]
    pub fn wdint(&mut self) -> WDINT_W {
        WDINT_W { w: self }
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only. See Section WDPROTECT."]
    #[inline(always)]
    pub fn wdprotect(&mut self) -> WDPROTECT_W {
        WDPROTECT_W { w: self }
    }
}
