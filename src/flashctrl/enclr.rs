#[doc = "Writer for register ENCLR"]
pub type W = crate::W<u32, super::ENCLR>;
#[doc = "Register ENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RDWR_CLR_EN`"]
pub struct RDWR_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `PROG1_CLR_EN`"]
pub struct PROG1_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG1_CLR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl W {
    #[doc = "Bit 26 - Clear read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline(always)]
    pub fn rdwr_clr_en(&mut self) -> RDWR_CLR_EN_W {
        RDWR_CLR_EN_W { w: self }
    }
    #[doc = "Bit 28 - Clear program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline(always)]
    pub fn prog1_clr_en(&mut self) -> PROG1_CLR_EN_W {
        PROG1_CLR_EN_W { w: self }
    }
}
