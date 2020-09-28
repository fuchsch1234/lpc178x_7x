#[doc = "Reader of register DYNAMICREADCONFIG"]
pub type R = crate::R<u32, super::DYNAMICREADCONFIG>;
#[doc = "Writer for register DYNAMICREADCONFIG"]
pub type W = crate::W<u32, super::DYNAMICREADCONFIG>;
#[doc = "Register DYNAMICREADCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DYNAMICREADCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Read data strategy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RD_A {
    #[doc = "0: Clock out delayed strategy, using CLKOUT (command not delayed, clock out delayed). POR reset value."]
    CLOCK_OUT_DELAY = 0,
    #[doc = "1: Command delayed strategy, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    EMCCLK_DELAY = 1,
    #[doc = "2: Command delayed strategy plus one clock cycle, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    EMCCLK_PLUS_ONE_DELAY = 2,
    #[doc = "3: Command delayed strategy plus two clock cycles, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    EMCCLK_PLUS_TWO_DELAY = 3,
}
impl From<RD_A> for u8 {
    #[inline(always)]
    fn from(variant: RD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u8, RD_A>;
impl RD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_A {
        match self.bits {
            0 => RD_A::CLOCK_OUT_DELAY,
            1 => RD_A::EMCCLK_DELAY,
            2 => RD_A::EMCCLK_PLUS_ONE_DELAY,
            3 => RD_A::EMCCLK_PLUS_TWO_DELAY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_OUT_DELAY`"]
    #[inline(always)]
    pub fn is_clock_out_delay(&self) -> bool {
        *self == RD_A::CLOCK_OUT_DELAY
    }
    #[doc = "Checks if the value of the field is `EMCCLK_DELAY`"]
    #[inline(always)]
    pub fn is_emcclk_delay(&self) -> bool {
        *self == RD_A::EMCCLK_DELAY
    }
    #[doc = "Checks if the value of the field is `EMCCLK_PLUS_ONE_DELAY`"]
    #[inline(always)]
    pub fn is_emcclk_plus_one_delay(&self) -> bool {
        *self == RD_A::EMCCLK_PLUS_ONE_DELAY
    }
    #[doc = "Checks if the value of the field is `EMCCLK_PLUS_TWO_DELAY`"]
    #[inline(always)]
    pub fn is_emcclk_plus_two_delay(&self) -> bool {
        *self == RD_A::EMCCLK_PLUS_TWO_DELAY
    }
}
#[doc = "Write proxy for field `RD`"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock out delayed strategy, using CLKOUT (command not delayed, clock out delayed). POR reset value."]
    #[inline(always)]
    pub fn clock_out_delay(self) -> &'a mut W {
        self.variant(RD_A::CLOCK_OUT_DELAY)
    }
    #[doc = "Command delayed strategy, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    #[inline(always)]
    pub fn emcclk_delay(self) -> &'a mut W {
        self.variant(RD_A::EMCCLK_DELAY)
    }
    #[doc = "Command delayed strategy plus one clock cycle, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    #[inline(always)]
    pub fn emcclk_plus_one_delay(self) -> &'a mut W {
        self.variant(RD_A::EMCCLK_PLUS_ONE_DELAY)
    }
    #[doc = "Command delayed strategy plus two clock cycles, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    #[inline(always)]
    pub fn emcclk_plus_two_delay(self) -> &'a mut W {
        self.variant(RD_A::EMCCLK_PLUS_TWO_DELAY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Read data strategy"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read data strategy"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
}
