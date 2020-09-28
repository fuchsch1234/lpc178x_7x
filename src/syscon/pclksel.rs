#[doc = "Reader of register PCLKSEL"]
pub type R = crate::R<u32, super::PCLKSEL>;
#[doc = "Writer for register PCLKSEL"]
pub type W = crate::W<u32, super::PCLKSEL>;
#[doc = "Register PCLKSEL `reset()`'s with value 0x10"]
impl crate::ResetValue for super::PCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `PCLKDIV`"]
pub type PCLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCLKDIV`"]
pub struct PCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals.. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
    #[inline(always)]
    pub fn pclkdiv(&self) -> PCLKDIV_R {
        PCLKDIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals.. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
    #[inline(always)]
    pub fn pclkdiv(&mut self) -> PCLKDIV_W {
        PCLKDIV_W { w: self }
    }
}
