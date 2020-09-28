#[doc = "Reader of register PBOOST"]
pub type R = crate::R<u32, super::PBOOST>;
#[doc = "Writer for register PBOOST"]
pub type W = crate::W<u32, super::PBOOST>;
#[doc = "Register PBOOST `reset()`'s with value 0"]
impl crate::ResetValue for super::PBOOST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Boost`"]
pub type BOOST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Boost`"]
pub struct BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Boost control bits. 00 : Boost is off, operation must be below 100 MHz. 11 : Boost is on, operation up to 120 MHz is supported. Other values are not allowed."]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Boost control bits. 00 : Boost is off, operation must be below 100 MHz. 11 : Boost is on, operation up to 120 MHz is supported. Other values are not allowed."]
    #[inline(always)]
    pub fn boost(&mut self) -> BOOST_W {
        BOOST_W { w: self }
    }
}
