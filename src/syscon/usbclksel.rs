#[doc = "Reader of register USBCLKSEL"]
pub type R = crate::R<u32, super::USBCLKSEL>;
#[doc = "Writer for register USBCLKSEL"]
pub type W = crate::W<u32, super::USBCLKSEL>;
#[doc = "Register USBCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBDIV_A {
    #[doc = "0: The divider is turned off, no clock will be provided to the USB subsystem."]
    THE_DIVIDER_IS_TURNE = 0,
    #[doc = "4: PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    PLL0_DIV_4 = 4,
    #[doc = "6: PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    PLL0_DIV_6 = 6,
}
impl From<USBDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: USBDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USBDIV`"]
pub type USBDIV_R = crate::R<u8, USBDIV_A>;
impl USBDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USBDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USBDIV_A::THE_DIVIDER_IS_TURNE),
            4 => Val(USBDIV_A::PLL0_DIV_4),
            6 => Val(USBDIV_A::PLL0_DIV_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `THE_DIVIDER_IS_TURNE`"]
    #[inline(always)]
    pub fn is_the_divider_is_turne(&self) -> bool {
        *self == USBDIV_A::THE_DIVIDER_IS_TURNE
    }
    #[doc = "Checks if the value of the field is `PLL0_DIV_4`"]
    #[inline(always)]
    pub fn is_pll0_div_4(&self) -> bool {
        *self == USBDIV_A::PLL0_DIV_4
    }
    #[doc = "Checks if the value of the field is `PLL0_DIV_6`"]
    #[inline(always)]
    pub fn is_pll0_div_6(&self) -> bool {
        *self == USBDIV_A::PLL0_DIV_6
    }
}
#[doc = "Write proxy for field `USBDIV`"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The divider is turned off, no clock will be provided to the USB subsystem."]
    #[inline(always)]
    pub fn the_divider_is_turne(self) -> &'a mut W {
        self.variant(USBDIV_A::THE_DIVIDER_IS_TURNE)
    }
    #[doc = "PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    #[inline(always)]
    pub fn pll0_div_4(self) -> &'a mut W {
        self.variant(USBDIV_A::PLL0_DIV_4)
    }
    #[doc = "PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    #[inline(always)]
    pub fn pll0_div_6(self) -> &'a mut W {
        self.variant(USBDIV_A::PLL0_DIV_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Selects the input clock for the USB clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBSEL_A {
    #[doc = "0: Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    SYSCLK_IS_USED_AS_TH = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the USB clock divider."]
    THE_OUTPUT_OF_THE_MA = 1,
    #[doc = "2: The output of the Alt PLL is used as the input to the USB clock divider."]
    THE_OUTPUT_OF_THE_AL = 2,
}
impl From<USBSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USBSEL`"]
pub type USBSEL_R = crate::R<u8, USBSEL_A>;
impl USBSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USBSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USBSEL_A::SYSCLK_IS_USED_AS_TH),
            1 => Val(USBSEL_A::THE_OUTPUT_OF_THE_MA),
            2 => Val(USBSEL_A::THE_OUTPUT_OF_THE_AL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_IS_USED_AS_TH`"]
    #[inline(always)]
    pub fn is_sysclk_is_used_as_th(&self) -> bool {
        *self == USBSEL_A::SYSCLK_IS_USED_AS_TH
    }
    #[doc = "Checks if the value of the field is `THE_OUTPUT_OF_THE_MA`"]
    #[inline(always)]
    pub fn is_the_output_of_the_ma(&self) -> bool {
        *self == USBSEL_A::THE_OUTPUT_OF_THE_MA
    }
    #[doc = "Checks if the value of the field is `THE_OUTPUT_OF_THE_AL`"]
    #[inline(always)]
    pub fn is_the_output_of_the_al(&self) -> bool {
        *self == USBSEL_A::THE_OUTPUT_OF_THE_AL
    }
}
#[doc = "Write proxy for field `USBSEL`"]
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    #[inline(always)]
    pub fn sysclk_is_used_as_th(self) -> &'a mut W {
        self.variant(USBSEL_A::SYSCLK_IS_USED_AS_TH)
    }
    #[doc = "The output of the Main PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn the_output_of_the_ma(self) -> &'a mut W {
        self.variant(USBSEL_A::THE_OUTPUT_OF_THE_MA)
    }
    #[doc = "The output of the Alt PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn the_output_of_the_al(self) -> &'a mut W {
        self.variant(USBSEL_A::THE_OUTPUT_OF_THE_AL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W {
        USBSEL_W { w: self }
    }
}
