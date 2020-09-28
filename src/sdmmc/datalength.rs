#[doc = "Reader of register DATALENGTH"]
pub type R = crate::R<u32, super::DATALENGTH>;
#[doc = "Writer for register DATALENGTH"]
pub type W = crate::W<u32, super::DATALENGTH>;
#[doc = "Register DATALENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::DATALENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATALENGTH`"]
pub type DATALENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATALENGTH`"]
pub struct DATALENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data length value"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data length value"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W {
        DATALENGTH_W { w: self }
    }
}
