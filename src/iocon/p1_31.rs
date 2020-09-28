#[doc = "Reader of register P1_31"]
pub type R = crate::R<u32, super::P1_31>;
#[doc = "Writer for register P1_31"]
pub type W = crate::W<u32, super::P1_31>;
#[doc = "Selects pin function for pin P1\\[31\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P1_31 = 0,
    #[doc = "1: Over-Current status for USB port 2."]
    USB_OVRCR2 = 1,
    #[doc = "2: Serial Clock for SSP1."]
    SSP1_SCK = 2,
    #[doc = "3: A/D converter 0, input 5. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_5 = 3,
    #[doc = "4: I2C0 clock                                             input/output (this pin does not use a specialized I2C                                             pad."]
    I2C0_SCL = 4,
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
            0 => Val(FUNC_A::P1_31),
            1 => Val(FUNC_A::USB_OVRCR2),
            2 => Val(FUNC_A::SSP1_SCK),
            3 => Val(FUNC_A::ADC0_IN_5),
            4 => Val(FUNC_A::I2C0_SCL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P1_31`"]
    #[inline(always)]
    pub fn is_p1_31(&self) -> bool {
        *self == FUNC_A::P1_31
    }
    #[doc = "Checks if the value of the field is `USB_OVRCR2`"]
    #[inline(always)]
    pub fn is_usb_ovrcr2(&self) -> bool {
        *self == FUNC_A::USB_OVRCR2
    }
    #[doc = "Checks if the value of the field is `SSP1_SCK`"]
    #[inline(always)]
    pub fn is_ssp1_sck(&self) -> bool {
        *self == FUNC_A::SSP1_SCK
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_5`"]
    #[inline(always)]
    pub fn is_adc0_in_5(&self) -> bool {
        *self == FUNC_A::ADC0_IN_5
    }
    #[doc = "Checks if the value of the field is `I2C0_SCL`"]
    #[inline(always)]
    pub fn is_i2c0_scl(&self) -> bool {
        *self == FUNC_A::I2C0_SCL
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
    pub fn p1_31(self) -> &'a mut W {
        self.variant(FUNC_A::P1_31)
    }
    #[doc = "Over-Current status for USB port 2."]
    #[inline(always)]
    pub fn usb_ovrcr2(self) -> &'a mut W {
        self.variant(FUNC_A::USB_OVRCR2)
    }
    #[doc = "Serial Clock for SSP1."]
    #[inline(always)]
    pub fn ssp1_sck(self) -> &'a mut W {
        self.variant(FUNC_A::SSP1_SCK)
    }
    #[doc = "A/D converter 0, input 5. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_5(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_5)
    }
    #[doc = "I2C0 clock input/output (this pin does not use a specialized I2C pad."]
    #[inline(always)]
    pub fn i2c0_scl(self) -> &'a mut W {
        self.variant(FUNC_A::I2C0_SCL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[31\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[31\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
