#[doc = "Reader of register STATICWAITTURN%s"]
pub type R = crate::R<u32, super::STATICWAITTURN>;
#[doc = "Writer for register STATICWAITTURN%s"]
pub type W = crate::W<u32, super::STATICWAITTURN>;
#[doc = "Register STATICWAITTURN%s `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::STATICWAITTURN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `WAITTURN`"]
pub type WAITTURN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITTURN`"]
pub struct WAITTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITTURN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bus turn-around cycles. 0x0 - 0xE = (n + 1) CCLK turn-around cycles. Bus turn-around time is (WAITTURN + 1) x tCCLK. 0xF = 16 CCLK turn-around cycles (POR reset value)."]
    #[inline(always)]
    pub fn waitturn(&self) -> WAITTURN_R {
        WAITTURN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bus turn-around cycles. 0x0 - 0xE = (n + 1) CCLK turn-around cycles. Bus turn-around time is (WAITTURN + 1) x tCCLK. 0xF = 16 CCLK turn-around cycles (POR reset value)."]
    #[inline(always)]
    pub fn waitturn(&mut self) -> WAITTURN_W {
        WAITTURN_W { w: self }
    }
}
