#[doc = "Reader of register P0_12"]
pub type R = crate::R<u32, super::P0_12>;
#[doc = "Writer for register P0_12"]
pub type W = crate::W<u32, super::P0_12>;
#[doc = "Selects pin function for pin P0\\[12\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P0_12 = 0,
    #[doc = "1: Port Power enable signal for USB port                                             2."]
    USB_PPWR2 = 1,
    #[doc = "2: Master In Slave Out for SSP1."]
    SSP1_MISO = 2,
    #[doc = "3: A/D converter 0, input 6. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_6 = 3,
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
            0 => Val(FUNC_A::P0_12),
            1 => Val(FUNC_A::USB_PPWR2),
            2 => Val(FUNC_A::SSP1_MISO),
            3 => Val(FUNC_A::ADC0_IN_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_12`"]
    #[inline(always)]
    pub fn is_p0_12(&self) -> bool {
        *self == FUNC_A::P0_12
    }
    #[doc = "Checks if the value of the field is `USB_PPWR2`"]
    #[inline(always)]
    pub fn is_usb_ppwr2(&self) -> bool {
        *self == FUNC_A::USB_PPWR2
    }
    #[doc = "Checks if the value of the field is `SSP1_MISO`"]
    #[inline(always)]
    pub fn is_ssp1_miso(&self) -> bool {
        *self == FUNC_A::SSP1_MISO
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_6`"]
    #[inline(always)]
    pub fn is_adc0_in_6(&self) -> bool {
        *self == FUNC_A::ADC0_IN_6
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
    pub fn p0_12(self) -> &'a mut W {
        self.variant(FUNC_A::P0_12)
    }
    #[doc = "Port Power enable signal for USB port 2."]
    #[inline(always)]
    pub fn usb_ppwr2(self) -> &'a mut W {
        self.variant(FUNC_A::USB_PPWR2)
    }
    #[doc = "Master In Slave Out for SSP1."]
    #[inline(always)]
    pub fn ssp1_miso(self) -> &'a mut W {
        self.variant(FUNC_A::SSP1_MISO)
    }
    #[doc = "A/D converter 0, input 6. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_6(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[12\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[12\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
