#[doc = "Reader of register FILTERPHB"]
pub type R = crate::R<u32, super::FILTERPHB>;
#[doc = "Writer for register FILTERPHB"]
pub type W = crate::W<u32, super::FILTERPHB>;
#[doc = "Register FILTERPHB `reset()`'s with value 0"]
impl crate::ResetValue for super::FILTERPHB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILTB`"]
pub type FILTB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FILTB`"]
pub struct FILTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay for PhB."]
    #[inline(always)]
    pub fn filtb(&self) -> FILTB_R {
        FILTB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay for PhB."]
    #[inline(always)]
    pub fn filtb(&mut self) -> FILTB_W {
        FILTB_W { w: self }
    }
}
