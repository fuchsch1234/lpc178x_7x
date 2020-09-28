#[doc = "Writer for register STATCLR"]
pub type W = crate::W<u32, super::STATCLR>;
#[doc = "Register STATCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::STATCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SIG_DONE_CLR`"]
pub struct SIG_DONE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_DONE_CLR_W<'a> {
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
#[doc = "Write proxy for field `RDWR_CLR_ST`"]
pub struct RDWR_CLR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_CLR_ST_W<'a> {
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
#[doc = "Write proxy for field `PROG1_CLR_ST`"]
pub struct PROG1_CLR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG1_CLR_ST_W<'a> {
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
    #[doc = "Bit 2 - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
    #[inline(always)]
    pub fn sig_done_clr(&mut self) -> SIG_DONE_CLR_W {
        SIG_DONE_CLR_W { w: self }
    }
    #[doc = "Bit 26 - Clear read/write operation finished interrupt status bit (EEPROM). 0 leave corresponding bit unchanged. 1 clear corresponding bit."]
    #[inline(always)]
    pub fn rdwr_clr_st(&mut self) -> RDWR_CLR_ST_W {
        RDWR_CLR_ST_W { w: self }
    }
    #[doc = "Bit 28 - Clear program operation finished interrupt status bit for EEPROM device 1. 0 leave corresponding bit unchanged. 1 clear corresponding bit."]
    #[inline(always)]
    pub fn prog1_clr_st(&mut self) -> PROG1_CLR_ST_W {
        PROG1_CLR_ST_W { w: self }
    }
}
