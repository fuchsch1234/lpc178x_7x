#[doc = "Reader of register EMCCLKSEL"]
pub type R = crate::R<u32, super::EMCCLKSEL>;
#[doc = "Writer for register EMCCLKSEL"]
pub type W = crate::W<u32, super::EMCCLKSEL>;
#[doc = "Register EMCCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::EMCCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the EMC clock rate relative to the CPU clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCDIV_A {
    #[doc = "0: The EMC uses the same clock as the CPU."]
    THE_EMC_USES_THE_SAM = 0,
    #[doc = "1: The EMC uses a clock at half the rate of the CPU."]
    THE_EMC_USES_A_CLOCK = 1,
}
impl From<EMCDIV_A> for bool {
    #[inline(always)]
    fn from(variant: EMCDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EMCDIV`"]
pub type EMCDIV_R = crate::R<bool, EMCDIV_A>;
impl EMCDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMCDIV_A {
        match self.bits {
            false => EMCDIV_A::THE_EMC_USES_THE_SAM,
            true => EMCDIV_A::THE_EMC_USES_A_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `THE_EMC_USES_THE_SAM`"]
    #[inline(always)]
    pub fn is_the_emc_uses_the_sam(&self) -> bool {
        *self == EMCDIV_A::THE_EMC_USES_THE_SAM
    }
    #[doc = "Checks if the value of the field is `THE_EMC_USES_A_CLOCK`"]
    #[inline(always)]
    pub fn is_the_emc_uses_a_clock(&self) -> bool {
        *self == EMCDIV_A::THE_EMC_USES_A_CLOCK
    }
}
#[doc = "Write proxy for field `EMCDIV`"]
pub struct EMCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMCDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The EMC uses the same clock as the CPU."]
    #[inline(always)]
    pub fn the_emc_uses_the_sam(self) -> &'a mut W {
        self.variant(EMCDIV_A::THE_EMC_USES_THE_SAM)
    }
    #[doc = "The EMC uses a clock at half the rate of the CPU."]
    #[inline(always)]
    pub fn the_emc_uses_a_clock(self) -> &'a mut W {
        self.variant(EMCDIV_A::THE_EMC_USES_A_CLOCK)
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
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline(always)]
    pub fn emcdiv(&self) -> EMCDIV_R {
        EMCDIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline(always)]
    pub fn emcdiv(&mut self) -> EMCDIV_W {
        EMCDIV_W { w: self }
    }
}
