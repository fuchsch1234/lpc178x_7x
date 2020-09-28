#[doc = "Reader of register CLKSRCSEL"]
pub type R = crate::R<u32, super::CLKSRCSEL>;
#[doc = "Writer for register CLKSRCSEL"]
pub type W = crate::W<u32, super::CLKSRCSEL>;
#[doc = "Register CLKSRCSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKSRCSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the clock source for sysclk and PLL0 as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    SELECTS_THE_INTERNAL = 0,
    #[doc = "1: Selects the main oscillator as the sysclk and PLL0 clock source."]
    SELECTS_THE_MAIN_OSC = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSRC`"]
pub type CLKSRC_R = crate::R<bool, CLKSRC_A>;
impl CLKSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::SELECTS_THE_INTERNAL,
            true => CLKSRC_A::SELECTS_THE_MAIN_OSC,
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == CLKSRC_A::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == CLKSRC_A::SELECTS_THE_MAIN_OSC
    }
}
#[doc = "Write proxy for field `CLKSRC`"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKSRC_A::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the main oscillator as the sysclk and PLL0 clock source."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKSRC_A::SELECTS_THE_MAIN_OSC)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
}
