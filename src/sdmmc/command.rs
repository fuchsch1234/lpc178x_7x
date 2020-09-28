#[doc = "Reader of register COMMAND"]
pub type R = crate::R<u32, super::COMMAND>;
#[doc = "Writer for register COMMAND"]
pub type W = crate::W<u32, super::COMMAND>;
#[doc = "Register COMMAND `reset()`'s with value 0"]
impl crate::ResetValue for super::COMMAND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CmdIndex`"]
pub type CMDINDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CmdIndex`"]
pub struct CMDINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `Response`"]
pub type RESPONSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Response`"]
pub struct RESPONSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_W<'a> {
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
#[doc = "Reader of field `LongRsp`"]
pub type LONGRSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LongRsp`"]
pub struct LONGRSP_W<'a> {
    w: &'a mut W,
}
impl<'a> LONGRSP_W<'a> {
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
#[doc = "Reader of field `Interrupt`"]
pub type INTERRUPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Interrupt`"]
pub struct INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_W<'a> {
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
#[doc = "Reader of field `Pending`"]
pub type PENDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Pending`"]
pub struct PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDING_W<'a> {
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
#[doc = "Reader of field `Enable`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Enable`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - If set, CPSM waits for a response."]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If set, CPSM receives a 136 bit long response."]
    #[inline(always)]
    pub fn long_rsp(&self) -> LONGRSP_R {
        LONGRSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set, CPSM disables command timer and waits for interrupt request."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If set, CPSM waits for CmdPend before it starts sending a command."]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If set, CPSM is enabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CMDINDEX_W {
        CMDINDEX_W { w: self }
    }
    #[doc = "Bit 6 - If set, CPSM waits for a response."]
    #[inline(always)]
    pub fn response(&mut self) -> RESPONSE_W {
        RESPONSE_W { w: self }
    }
    #[doc = "Bit 7 - If set, CPSM receives a 136 bit long response."]
    #[inline(always)]
    pub fn long_rsp(&mut self) -> LONGRSP_W {
        LONGRSP_W { w: self }
    }
    #[doc = "Bit 8 - If set, CPSM disables command timer and waits for interrupt request."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W {
        INTERRUPT_W { w: self }
    }
    #[doc = "Bit 9 - If set, CPSM waits for CmdPend before it starts sending a command."]
    #[inline(always)]
    pub fn pending(&mut self) -> PENDING_W {
        PENDING_W { w: self }
    }
    #[doc = "Bit 10 - If set, CPSM is enabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
