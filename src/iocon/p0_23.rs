#[doc = "Reader of register P0_23"]
pub type R = crate::R<u32, super::P0_23>;
#[doc = "Writer for register P0_23"]
pub type W = crate::W<u32, super::P0_23>;
#[doc = "Register P0_23 `reset()`'s with value 0x01b0"]
impl crate::ResetValue for super::P0_23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01b0
    }
}
#[doc = "Selects pin function for pin P0\\[23\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P0_23 = 0,
    #[doc = "1: A/D converter 0, input 0. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_0 = 1,
    #[doc = "2: Receive Clock. It is driven by the master and                                             received by the slave. Corresponds to the signal SCK in                                             the                                                 I2S-bus                                                 specification."]
    I2S_RX_SCK = 2,
    #[doc = "3: Capture input for Timer 3, channel 0."]
    T3_CAP0 = 3,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FUNC`"]
pub type FUNC_R = crate::R<u8, FUNC_A>;
impl FUNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FUNC_A::P0_23),
            1 => Val(FUNC_A::ADC0_IN_0),
            2 => Val(FUNC_A::I2S_RX_SCK),
            3 => Val(FUNC_A::T3_CAP0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_23`"]
    #[inline(always)]
    pub fn is_p0_23(&self) -> bool {
        *self == FUNC_A::P0_23
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_0`"]
    #[inline(always)]
    pub fn is_adc0_in_0(&self) -> bool {
        *self == FUNC_A::ADC0_IN_0
    }
    #[doc = "Checks if the value of the field is `I2S_RX_SCK`"]
    #[inline(always)]
    pub fn is_i2s_rx_sck(&self) -> bool {
        *self == FUNC_A::I2S_RX_SCK
    }
    #[doc = "Checks if the value of the field is `T3_CAP0`"]
    #[inline(always)]
    pub fn is_t3_cap0(&self) -> bool {
        *self == FUNC_A::T3_CAP0
    }
}
#[doc = "Write proxy for field `FUNC`"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_23(self) -> &'a mut W {
        self.variant(FUNC_A::P0_23)
    }
    #[doc = "A/D converter 0, input 0. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_0(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_0)
    }
    #[doc = "Receive Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_rx_sck(self) -> &'a mut W {
        self.variant(FUNC_A::I2S_RX_SCK)
    }
    #[doc = "Capture input for Timer 3, channel 0."]
    #[inline(always)]
    pub fn t3_cap0(self) -> &'a mut W {
        self.variant(FUNC_A::T3_CAP0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[23\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[23\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
