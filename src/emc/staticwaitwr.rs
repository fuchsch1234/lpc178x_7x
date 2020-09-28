#[doc = "Reader of register STATICWAITWR%s"]
pub type R = crate::R<u32, super::STATICWAITWR>;
#[doc = "Writer for register STATICWAITWR%s"]
pub type W = crate::W<u32, super::STATICWAITWR>;
#[doc = "Register STATICWAITWR%s `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::STATICWAITWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `WAITWR`"]
pub type WAITWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITWR`"]
pub struct WAITWR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Write wait states. SRAM wait state time for write accesses after the first read: 0x0 - 0x1E = (n + 2) CCLK cycle write access time. The wait state time for write accesses after the first read is WAITWR (n + 2) x tCCLK. 0x1F = 33 CCLK cycle write access time (POR reset value)."]
    #[inline(always)]
    pub fn waitwr(&self) -> WAITWR_R {
        WAITWR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write wait states. SRAM wait state time for write accesses after the first read: 0x0 - 0x1E = (n + 2) CCLK cycle write access time. The wait state time for write accesses after the first read is WAITWR (n + 2) x tCCLK. 0x1F = 33 CCLK cycle write access time (POR reset value)."]
    #[inline(always)]
    pub fn waitwr(&mut self) -> WAITWR_W {
        WAITWR_W { w: self }
    }
}
