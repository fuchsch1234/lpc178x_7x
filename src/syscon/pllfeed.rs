#[doc = "Writer for register PLL%sFEED"]
pub type W = crate::W<u32, super::PLLFEED>;
#[doc = "Register PLL%sFEED `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLFEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PLLFEED`"]
pub struct PLLFEED_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - The PLL feed sequence must be written to this register in order for the related PLL's configuration and control register changes to take effect."]
    #[inline(always)]
    pub fn pllfeed(&mut self) -> PLLFEED_W {
        PLLFEED_W { w: self }
    }
}
