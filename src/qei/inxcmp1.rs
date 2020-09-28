#[doc = "Reader of register INXCMP1"]
pub type R = crate::R<u32, super::INXCMP1>;
#[doc = "Writer for register INXCMP1"]
pub type W = crate::W<u32, super::INXCMP1>;
#[doc = "Register INXCMP1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::INXCMP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ICMP1`"]
pub type ICMP1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICMP1`"]
pub struct ICMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ICMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Index compare value 1."]
    #[inline(always)]
    pub fn icmp1(&self) -> ICMP1_R {
        ICMP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Index compare value 1."]
    #[inline(always)]
    pub fn icmp1(&mut self) -> ICMP1_W {
        ICMP1_W { w: self }
    }
}
