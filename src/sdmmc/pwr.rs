#[doc = "Reader of register PWR"]
pub type R = crate::R<u32, super::PWR>;
#[doc = "Writer for register PWR"]
pub type W = crate::W<u32, super::PWR>;
#[doc = "Register PWR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTRL_A {
    #[doc = "0: Power-off"]
    POWER_OFF = 0,
    #[doc = "2: Power-up"]
    POWER_UP = 2,
    #[doc = "3: Power-on"]
    POWER_ON = 3,
}
impl From<CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<u8, CTRL_A>;
impl CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTRL_A::POWER_OFF),
            2 => Val(CTRL_A::POWER_UP),
            3 => Val(CTRL_A::POWER_ON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POWER_OFF`"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == CTRL_A::POWER_OFF
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == CTRL_A::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWER_ON`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == CTRL_A::POWER_ON
    }
}
#[doc = "Write proxy for field `CTRL`"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Power-off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut W {
        self.variant(CTRL_A::POWER_OFF)
    }
    #[doc = "Power-up"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(CTRL_A::POWER_UP)
    }
    #[doc = "Power-on"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut W {
        self.variant(CTRL_A::POWER_ON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `OPENDRAIN`"]
pub type OPENDRAIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPENDRAIN`"]
pub struct OPENDRAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENDRAIN_W<'a> {
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
#[doc = "Reader of field `ROD`"]
pub type ROD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROD`"]
pub struct ROD_W<'a> {
    w: &'a mut W,
}
impl<'a> ROD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Power control"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 6 - SD_CMD output control."]
    #[inline(always)]
    pub fn opendrain(&self) -> OPENDRAIN_R {
        OPENDRAIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rod control."]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power control"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
    #[doc = "Bit 6 - SD_CMD output control."]
    #[inline(always)]
    pub fn opendrain(&mut self) -> OPENDRAIN_W {
        OPENDRAIN_W { w: self }
    }
    #[doc = "Bit 7 - Rod control."]
    #[inline(always)]
    pub fn rod(&mut self) -> ROD_W {
        ROD_W { w: self }
    }
}
