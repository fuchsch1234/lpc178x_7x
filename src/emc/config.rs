#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM_A {
    #[doc = "0: Little-endian\r\nmode (POR reset value)."]
    LITTLEENDIAN = 0,
    #[doc = "1: Big-endian\r\nmode."]
    BIGENDIAN = 1,
}
impl From<EM_A> for bool {
    #[inline(always)]
    fn from(variant: EM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EM`"]
pub type EM_R = crate::R<bool, EM_A>;
impl EM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM_A {
        match self.bits {
            false => EM_A::LITTLEENDIAN,
            true => EM_A::BIGENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLEENDIAN`"]
    #[inline(always)]
    pub fn is_littleendian(&self) -> bool {
        *self == EM_A::LITTLEENDIAN
    }
    #[doc = "Checks if the value of the field is `BIGENDIAN`"]
    #[inline(always)]
    pub fn is_bigendian(&self) -> bool {
        *self == EM_A::BIGENDIAN
    }
}
#[doc = "Write proxy for field `EM`"]
pub struct EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little-endian mode (POR reset value)."]
    #[inline(always)]
    pub fn littleendian(self) -> &'a mut W {
        self.variant(EM_A::LITTLEENDIAN)
    }
    #[doc = "Big-endian mode."]
    #[inline(always)]
    pub fn bigendian(self) -> &'a mut W {
        self.variant(EM_A::BIGENDIAN)
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
#[doc = "CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKR_A {
    #[doc = "0: 1:1(POR reset value)"]
    PORRESET = 0,
    #[doc = "1: 1:2 (this option is not available on the LPC178x/177x)"]
    DONOTUSE = 1,
}
impl From<CLKR_A> for bool {
    #[inline(always)]
    fn from(variant: CLKR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKR`"]
pub type CLKR_R = crate::R<bool, CLKR_A>;
impl CLKR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKR_A {
        match self.bits {
            false => CLKR_A::PORRESET,
            true => CLKR_A::DONOTUSE,
        }
    }
    #[doc = "Checks if the value of the field is `PORRESET`"]
    #[inline(always)]
    pub fn is_porreset(&self) -> bool {
        *self == CLKR_A::PORRESET
    }
    #[doc = "Checks if the value of the field is `DONOTUSE`"]
    #[inline(always)]
    pub fn is_donotuse(&self) -> bool {
        *self == CLKR_A::DONOTUSE
    }
}
#[doc = "Write proxy for field `CLKR`"]
pub struct CLKR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1:1(POR reset value)"]
    #[inline(always)]
    pub fn porreset(self) -> &'a mut W {
        self.variant(CLKR_A::PORRESET)
    }
    #[doc = "1:2 (this option is not available on the LPC178x/177x)"]
    #[inline(always)]
    pub fn donotuse(self) -> &'a mut W {
        self.variant(CLKR_A::DONOTUSE)
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
impl R {
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&self) -> CLKR_R {
        CLKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline(always)]
    pub fn em(&mut self) -> EM_W {
        EM_W { w: self }
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&mut self) -> CLKR_W {
        CLKR_W { w: self }
    }
}
