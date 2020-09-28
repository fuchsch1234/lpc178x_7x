#[doc = "Writer for register CLEAR"]
pub type W = crate::W<u32, super::CLEAR>;
#[doc = "Register CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CMDCRCFAILCLR`"]
pub struct CMDCRCFAILCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRCFAILCLR_W<'a> {
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
#[doc = "Write proxy for field `DATACRCFAILCLR`"]
pub struct DATACRCFAILCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACRCFAILCLR_W<'a> {
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
#[doc = "Write proxy for field `CMDTIMEOUTCLR`"]
pub struct CMDTIMEOUTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTIMEOUTCLR_W<'a> {
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
#[doc = "Write proxy for field `DATATIMEOUTCLR`"]
pub struct DATATIMEOUTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATIMEOUTCLR_W<'a> {
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
#[doc = "Write proxy for field `TXUNDERRUNCLR`"]
pub struct TXUNDERRUNCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUNCLR_W<'a> {
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
#[doc = "Write proxy for field `RXOVERRUNCLR`"]
pub struct RXOVERRUNCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUNCLR_W<'a> {
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
#[doc = "Write proxy for field `CMDRESPENDCLR`"]
pub struct CMDRESPENDCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRESPENDCLR_W<'a> {
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
#[doc = "Write proxy for field `CMDSENTCLR`"]
pub struct CMDSENTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENTCLR_W<'a> {
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
#[doc = "Write proxy for field `DATAENDCLR`"]
pub struct DATAENDCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENDCLR_W<'a> {
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
#[doc = "Write proxy for field `STARTBITERRCLR`"]
pub struct STARTBITERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTBITERRCLR_W<'a> {
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
#[doc = "Write proxy for field `DATABLOCKENDCLR`"]
pub struct DATABLOCKENDCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABLOCKENDCLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clears CmdCrcFail flag."]
    #[inline(always)]
    pub fn cmdcrcfailclr(&mut self) -> CMDCRCFAILCLR_W {
        CMDCRCFAILCLR_W { w: self }
    }
    #[doc = "Bit 1 - Clears DataCrcFail flag."]
    #[inline(always)]
    pub fn datacrcfailclr(&mut self) -> DATACRCFAILCLR_W {
        DATACRCFAILCLR_W { w: self }
    }
    #[doc = "Bit 2 - Clears CmdTimeOut flag."]
    #[inline(always)]
    pub fn cmdtimeoutclr(&mut self) -> CMDTIMEOUTCLR_W {
        CMDTIMEOUTCLR_W { w: self }
    }
    #[doc = "Bit 3 - Clears DataTimeOut flag."]
    #[inline(always)]
    pub fn datatimeoutclr(&mut self) -> DATATIMEOUTCLR_W {
        DATATIMEOUTCLR_W { w: self }
    }
    #[doc = "Bit 4 - Clears TxUnderrun flag."]
    #[inline(always)]
    pub fn txunderrunclr(&mut self) -> TXUNDERRUNCLR_W {
        TXUNDERRUNCLR_W { w: self }
    }
    #[doc = "Bit 5 - Clears RxOverrun flag."]
    #[inline(always)]
    pub fn rxoverrunclr(&mut self) -> RXOVERRUNCLR_W {
        RXOVERRUNCLR_W { w: self }
    }
    #[doc = "Bit 6 - Clears CmdRespEnd flag."]
    #[inline(always)]
    pub fn cmdrespendclr(&mut self) -> CMDRESPENDCLR_W {
        CMDRESPENDCLR_W { w: self }
    }
    #[doc = "Bit 7 - Clears CmdSent flag."]
    #[inline(always)]
    pub fn cmdsentclr(&mut self) -> CMDSENTCLR_W {
        CMDSENTCLR_W { w: self }
    }
    #[doc = "Bit 8 - Clears DataEnd flag."]
    #[inline(always)]
    pub fn dataendclr(&mut self) -> DATAENDCLR_W {
        DATAENDCLR_W { w: self }
    }
    #[doc = "Bit 9 - Clears StartBitErr flag."]
    #[inline(always)]
    pub fn startbiterrclr(&mut self) -> STARTBITERRCLR_W {
        STARTBITERRCLR_W { w: self }
    }
    #[doc = "Bit 10 - Clears DataBlockEnd flag."]
    #[inline(always)]
    pub fn datablockendclr(&mut self) -> DATABLOCKENDCLR_W {
        DATABLOCKENDCLR_W { w: self }
    }
}
