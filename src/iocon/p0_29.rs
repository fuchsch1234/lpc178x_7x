#[doc = "Reader of register P0_29"]
pub type R = crate::R<u32, super::P0_29>;
#[doc = "Writer for register P0_29"]
pub type W = crate::W<u32, super::P0_29>;
#[doc = "Register P0_29 `reset()`'s with value 0"]
impl crate::ResetValue for super::P0_29 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects pin function for pin P0\\[29\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P0_29 = 0,
    #[doc = "1: USB port 1 bidirectional D+ line."]
    USB_DP1 = 1,
    #[doc = "2: External interrupt 0 input."]
    EINT0 = 2,
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
            0 => Val(FUNC_A::P0_29),
            1 => Val(FUNC_A::USB_DP1),
            2 => Val(FUNC_A::EINT0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_29`"]
    #[inline(always)]
    pub fn is_p0_29(&self) -> bool {
        *self == FUNC_A::P0_29
    }
    #[doc = "Checks if the value of the field is `USB_DP1`"]
    #[inline(always)]
    pub fn is_usb_dp1(&self) -> bool {
        *self == FUNC_A::USB_DP1
    }
    #[doc = "Checks if the value of the field is `EINT0`"]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        *self == FUNC_A::EINT0
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
    pub fn p0_29(self) -> &'a mut W {
        self.variant(FUNC_A::P0_29)
    }
    #[doc = "USB port 1 bidirectional D+ line."]
    #[inline(always)]
    pub fn usb_dp1(self) -> &'a mut W {
        self.variant(FUNC_A::USB_DP1)
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut W {
        self.variant(FUNC_A::EINT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[29\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[29\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
