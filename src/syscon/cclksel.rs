#[doc = "Reader of register CCLKSEL"]
pub type R = crate::R<u32, super::CCLKSEL>;
#[doc = "Writer for register CCLKSEL"]
pub type W = crate::W<u32, super::CCLKSEL>;
#[doc = "Register CCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCLKDIV`"]
pub type CCLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLKDIV`"]
pub struct CCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Selects the input clock for the CPU clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLKSEL_A {
    #[doc = "0: Sysclk is used as the input to the CPU clock divider."]
    SYSCLK_IS_USED_AS_TH = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the CPU clock divider."]
    THE_OUTPUT_OF_THE_MA = 1,
}
impl From<CCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCLKSEL`"]
pub type CCLKSEL_R = crate::R<bool, CCLKSEL_A>;
impl CCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLKSEL_A {
        match self.bits {
            false => CCLKSEL_A::SYSCLK_IS_USED_AS_TH,
            true => CCLKSEL_A::THE_OUTPUT_OF_THE_MA,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_IS_USED_AS_TH`"]
    #[inline(always)]
    pub fn is_sysclk_is_used_as_th(&self) -> bool {
        *self == CCLKSEL_A::SYSCLK_IS_USED_AS_TH
    }
    #[doc = "Checks if the value of the field is `THE_OUTPUT_OF_THE_MA`"]
    #[inline(always)]
    pub fn is_the_output_of_the_ma(&self) -> bool {
        *self == CCLKSEL_A::THE_OUTPUT_OF_THE_MA
    }
}
#[doc = "Write proxy for field `CCLKSEL`"]
pub struct CCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sysclk is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn sysclk_is_used_as_th(self) -> &'a mut W {
        self.variant(CCLKSEL_A::SYSCLK_IS_USED_AS_TH)
    }
    #[doc = "The output of the Main PLL is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn the_output_of_the_ma(self) -> &'a mut W {
        self.variant(CCLKSEL_A::THE_OUTPUT_OF_THE_MA)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclkdiv(&self) -> CCLKDIV_R {
        CCLKDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline(always)]
    pub fn cclksel(&self) -> CCLKSEL_R {
        CCLKSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclkdiv(&mut self) -> CCLKDIV_W {
        CCLKDIV_W { w: self }
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline(always)]
    pub fn cclksel(&mut self) -> CCLKSEL_W {
        CCLKSEL_W { w: self }
    }
}
