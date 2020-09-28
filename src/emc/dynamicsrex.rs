#[doc = "Reader of register DYNAMICSREX"]
pub type R = crate::R<u32, super::DYNAMICSREX>;
#[doc = "Writer for register DYNAMICSREX"]
pub type W = crate::W<u32, super::DYNAMICSREX>;
#[doc = "Register DYNAMICSREX `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICSREX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TSREX`"]
pub type TSREX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSREX`"]
pub struct TSREX_W<'a> {
    w: &'a mut W,
}
impl<'a> TSREX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Self-refresh exit time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn tsrex(&self) -> TSREX_R {
        TSREX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Self-refresh exit time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn tsrex(&mut self) -> TSREX_W {
        TSREX_W { w: self }
    }
}
