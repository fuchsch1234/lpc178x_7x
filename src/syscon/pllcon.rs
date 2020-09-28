#[doc = "Reader of register PLL%sCON"]
pub type R = crate::R<u32, super::PLLCON>;
#[doc = "Writer for register PLL%sCON"]
pub type W = crate::W<u32, super::PLLCON>;
#[doc = "Register PLL%sCON `reset()`'s with value 0"]
impl crate::ResetValue for super::PLLCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLE`"]
pub type PLLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLE`"]
pub struct PLLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
    #[inline(always)]
    pub fn plle(&mut self) -> PLLE_W {
        PLLE_W { w: self }
    }
}
