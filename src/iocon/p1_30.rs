#[doc = "Reader of register P1_30"]
pub type R = crate::R<u32, super::P1_30>;
#[doc = "Writer for register P1_30"]
pub type W = crate::W<u32, super::P1_30>;
#[doc = "Selects pin function for pin P1\\[30\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P1_30 = 0,
    #[doc = "1: Power Status for USB port 2."]
    USB_PWRD2 = 1,
    #[doc = "2: Monitors the presence of USB bus                                             power.This signal must be HIGH for USB reset to                                             occur."]
    USB_VBUS = 2,
    #[doc = "3: A/D converter 0, input 4. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_4 = 3,
    #[doc = "4: I2C0 data input/output                                             (this pin does not use a specialized I2C                                             pad."]
    I2C0_SDA = 4,
    #[doc = "5: RS-485/EIA-485 output enable signal for                                             UART3."]
    U3_OE = 5,
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
            0 => Val(FUNC_A::P1_30),
            1 => Val(FUNC_A::USB_PWRD2),
            2 => Val(FUNC_A::USB_VBUS),
            3 => Val(FUNC_A::ADC0_IN_4),
            4 => Val(FUNC_A::I2C0_SDA),
            5 => Val(FUNC_A::U3_OE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P1_30`"]
    #[inline(always)]
    pub fn is_p1_30(&self) -> bool {
        *self == FUNC_A::P1_30
    }
    #[doc = "Checks if the value of the field is `USB_PWRD2`"]
    #[inline(always)]
    pub fn is_usb_pwrd2(&self) -> bool {
        *self == FUNC_A::USB_PWRD2
    }
    #[doc = "Checks if the value of the field is `USB_VBUS`"]
    #[inline(always)]
    pub fn is_usb_vbus(&self) -> bool {
        *self == FUNC_A::USB_VBUS
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_4`"]
    #[inline(always)]
    pub fn is_adc0_in_4(&self) -> bool {
        *self == FUNC_A::ADC0_IN_4
    }
    #[doc = "Checks if the value of the field is `I2C0_SDA`"]
    #[inline(always)]
    pub fn is_i2c0_sda(&self) -> bool {
        *self == FUNC_A::I2C0_SDA
    }
    #[doc = "Checks if the value of the field is `U3_OE`"]
    #[inline(always)]
    pub fn is_u3_oe(&self) -> bool {
        *self == FUNC_A::U3_OE
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
    pub fn p1_30(self) -> &'a mut W {
        self.variant(FUNC_A::P1_30)
    }
    #[doc = "Power Status for USB port 2."]
    #[inline(always)]
    pub fn usb_pwrd2(self) -> &'a mut W {
        self.variant(FUNC_A::USB_PWRD2)
    }
    #[doc = "Monitors the presence of USB bus power.This signal must be HIGH for USB reset to occur."]
    #[inline(always)]
    pub fn usb_vbus(self) -> &'a mut W {
        self.variant(FUNC_A::USB_VBUS)
    }
    #[doc = "A/D converter 0, input 4. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_4(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_4)
    }
    #[doc = "I2C0 data input/output (this pin does not use a specialized I2C pad."]
    #[inline(always)]
    pub fn i2c0_sda(self) -> &'a mut W {
        self.variant(FUNC_A::I2C0_SDA)
    }
    #[doc = "RS-485/EIA-485 output enable signal for UART3."]
    #[inline(always)]
    pub fn u3_oe(self) -> &'a mut W {
        self.variant(FUNC_A::U3_OE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[30\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[30\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
