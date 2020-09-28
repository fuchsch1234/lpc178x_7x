#[doc = "Reader of register P2_10"]
pub type R = crate::R<u32, super::P2_10>;
#[doc = "Writer for register P2_10"]
pub type W = crate::W<u32, super::P2_10>;
#[doc = "Selects pin function for pin P2\\[10\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin. This\r\n                                            pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the\r\n                                            on-chip boot loader to take over control of the part\r\n                                            after a reset and go into ISP mode. "]
    P2_10 = 0,
    #[doc = "1: External interrupt 0 input."]
    EINT0 = 1,
    #[doc = "2: Non-maskable interrupt input."]
    NMI = 2,
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
            0 => Val(FUNC_A::P2_10),
            1 => Val(FUNC_A::EINT0),
            2 => Val(FUNC_A::NMI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P2_10`"]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        *self == FUNC_A::P2_10
    }
    #[doc = "Checks if the value of the field is `EINT0`"]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        *self == FUNC_A::EINT0
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == FUNC_A::NMI
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
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the on-chip boot loader to take over control of the part after a reset and go into ISP mode."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut W {
        self.variant(FUNC_A::P2_10)
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut W {
        self.variant(FUNC_A::EINT0)
    }
    #[doc = "Non-maskable interrupt input."]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut W {
        self.variant(FUNC_A::NMI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[10\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[10\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
