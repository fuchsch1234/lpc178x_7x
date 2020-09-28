#[doc = "Reader of register PLL%sCFG"]
pub type R = crate::R<u32, super::PLLCFG>;
#[doc = "Writer for register PLL%sCFG"]
pub type W = crate::W<u32, super::PLLCFG>;
#[doc = "Register PLL%sCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL Multiplier value. Supplies the value \"M\" in the PLL frequency calculations. Note: For details on selecting the right value for MSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - PLL Divider value. Supplies the value \"P\" in the PLL frequency calculations. Note: For details on selecting the right value for PSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL Multiplier value. Supplies the value \"M\" in the PLL frequency calculations. Note: For details on selecting the right value for MSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - PLL Divider value. Supplies the value \"P\" in the PLL frequency calculations. Note: For details on selecting the right value for PSEL see Section 3.10.4."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
}
