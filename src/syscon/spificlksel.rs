#[doc = "Reader of register SPIFICLKSEL"]
pub type R = crate::R<u32, super::SPIFICLKSEL>;
#[doc = "Writer for register SPIFICLKSEL"]
pub type W = crate::W<u32, super::SPIFICLKSEL>;
#[doc = "Register SPIFICLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPIFICLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPIFIDIV`"]
pub type SPIFIDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPIFIDIV`"]
pub struct SPIFIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Selects the input clock for the USB clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPIFISEL_A {
    #[doc = "0: Sysclk is used as the input to the SPIFI clock divider."]
    SYSCLK_IS_USED_AS_TH = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the SPIFI clock divider."]
    THE_OUTPUT_OF_THE_MA = 1,
    #[doc = "2: The output of the Alt PLL is used as the input to the SPIFI clock divider."]
    THE_OUTPUT_OF_THE_AL = 2,
}
impl From<SPIFISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPIFISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPIFISEL`"]
pub type SPIFISEL_R = crate::R<u8, SPIFISEL_A>;
impl SPIFISEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPIFISEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPIFISEL_A::SYSCLK_IS_USED_AS_TH),
            1 => Val(SPIFISEL_A::THE_OUTPUT_OF_THE_MA),
            2 => Val(SPIFISEL_A::THE_OUTPUT_OF_THE_AL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_IS_USED_AS_TH`"]
    #[inline(always)]
    pub fn is_sysclk_is_used_as_th(&self) -> bool {
        *self == SPIFISEL_A::SYSCLK_IS_USED_AS_TH
    }
    #[doc = "Checks if the value of the field is `THE_OUTPUT_OF_THE_MA`"]
    #[inline(always)]
    pub fn is_the_output_of_the_ma(&self) -> bool {
        *self == SPIFISEL_A::THE_OUTPUT_OF_THE_MA
    }
    #[doc = "Checks if the value of the field is `THE_OUTPUT_OF_THE_AL`"]
    #[inline(always)]
    pub fn is_the_output_of_the_al(&self) -> bool {
        *self == SPIFISEL_A::THE_OUTPUT_OF_THE_AL
    }
}
#[doc = "Write proxy for field `SPIFISEL`"]
pub struct SPIFISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIFISEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sysclk is used as the input to the SPIFI clock divider."]
    #[inline(always)]
    pub fn sysclk_is_used_as_th(self) -> &'a mut W {
        self.variant(SPIFISEL_A::SYSCLK_IS_USED_AS_TH)
    }
    #[doc = "The output of the Main PLL is used as the input to the SPIFI clock divider."]
    #[inline(always)]
    pub fn the_output_of_the_ma(self) -> &'a mut W {
        self.variant(SPIFISEL_A::THE_OUTPUT_OF_THE_MA)
    }
    #[doc = "The output of the Alt PLL is used as the input to the SPIFI clock divider."]
    #[inline(always)]
    pub fn the_output_of_the_al(self) -> &'a mut W {
        self.variant(SPIFISEL_A::THE_OUTPUT_OF_THE_AL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
    #[inline(always)]
    pub fn spifidiv(&self) -> SPIFIDIV_R {
        SPIFIDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn spifisel(&self) -> SPIFISEL_R {
        SPIFISEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
    #[inline(always)]
    pub fn spifidiv(&mut self) -> SPIFIDIV_W {
        SPIFIDIV_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn spifisel(&mut self) -> SPIFISEL_W {
        SPIFISEL_W { w: self }
    }
}
