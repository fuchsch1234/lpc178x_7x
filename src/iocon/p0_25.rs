#[doc = "Reader of register P0_25"]
pub type R = crate::R<u32, super::P0_25>;
#[doc = "Writer for register P0_25"]
pub type W = crate::W<u32, super::P0_25>;
#[doc = "Selects pin function for pin P0\\[25\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P0_25 = 0,
    #[doc = "1: A/D converter 0, input 2. When configured as an  ADC input, the digital function of the pin must be  disabled."]
    ADC0_IN_2 = 1,
    #[doc = "2: Receive data. It is driven by the transmitter and  read by the receiver. Corresponds to the signal SD in   the I2S-bus specification."]
    I2S_RX_SDA = 2,
    #[doc = "3: Transmitter output for UART3."]
    U3_TXD = 3,
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
            0 => Val(FUNC_A::P0_25),
            1 => Val(FUNC_A::ADC0_IN_2),
            2 => Val(FUNC_A::I2S_RX_SDA),
            3 => Val(FUNC_A::U3_TXD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_25`"]
    #[inline(always)]
    pub fn is_p0_25(&self) -> bool {
        *self == FUNC_A::P0_25
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_2`"]
    #[inline(always)]
    pub fn is_adc0_in_2(&self) -> bool {
        *self == FUNC_A::ADC0_IN_2
    }
    #[doc = "Checks if the value of the field is `I2S_RX_SDA`"]
    #[inline(always)]
    pub fn is_i2s_rx_sda(&self) -> bool {
        *self == FUNC_A::I2S_RX_SDA
    }
    #[doc = "Checks if the value of the field is `U3_TXD`"]
    #[inline(always)]
    pub fn is_u3_txd(&self) -> bool {
        *self == FUNC_A::U3_TXD
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
    pub fn p0_25(self) -> &'a mut W {
        self.variant(FUNC_A::P0_25)
    }
    #[doc = "A/D converter 0, input 2. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_2(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_2)
    }
    #[doc = "Receive data. It is driven by the transmitter and read by the receiver. Corresponds to the signal SD in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_rx_sda(self) -> &'a mut W {
        self.variant(FUNC_A::I2S_RX_SDA)
    }
    #[doc = "Transmitter output for UART3."]
    #[inline(always)]
    pub fn u3_txd(self) -> &'a mut W {
        self.variant(FUNC_A::U3_TXD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[25\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[25\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
