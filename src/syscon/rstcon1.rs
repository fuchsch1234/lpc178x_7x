#[doc = "Reader of register RSTCON1"]
pub type R = crate::R<u32, super::RSTCON1>;
#[doc = "Writer for register RSTCON1"]
pub type W = crate::W<u32, super::RSTCON1>;
#[doc = "Register RSTCON1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCON1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSTIOCON`"]
pub type RSTIOCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIOCON`"]
pub struct RSTIOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIOCON_W<'a> {
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
#[doc = "Reader of field `RSTDAC`"]
pub type RSTDAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTDAC`"]
pub struct RSTDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDAC_W<'a> {
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
#[doc = "Reader of field `RSTCANACC`"]
pub type RSTCANACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTCANACC`"]
pub struct RSTCANACC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCANACC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reset control bit for the IOCON registers."]
    #[inline(always)]
    pub fn rstiocon(&self) -> RSTIOCON_R {
        RSTIOCON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - D/A converter (DAC) reset control bit."]
    #[inline(always)]
    pub fn rstdac(&self) -> RSTDAC_R {
        RSTDAC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAN acceptance filter reset control bit."]
    #[inline(always)]
    pub fn rstcanacc(&self) -> RSTCANACC_R {
        RSTCANACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset control bit for the IOCON registers."]
    #[inline(always)]
    pub fn rstiocon(&mut self) -> RSTIOCON_W {
        RSTIOCON_W { w: self }
    }
    #[doc = "Bit 1 - D/A converter (DAC) reset control bit."]
    #[inline(always)]
    pub fn rstdac(&mut self) -> RSTDAC_W {
        RSTDAC_W { w: self }
    }
    #[doc = "Bit 2 - CAN acceptance filter reset control bit."]
    #[inline(always)]
    pub fn rstcanacc(&mut self) -> RSTCANACC_W {
        RSTCANACC_W { w: self }
    }
}
