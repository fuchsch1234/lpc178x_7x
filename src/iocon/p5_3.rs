#[doc = "Reader of register P5_3"]
pub type R = crate::R<u32, super::P5_3>;
#[doc = "Writer for register P5_3"]
pub type W = crate::W<u32, super::P5_3>;
#[doc = "Register P5_3 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::P5_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Selects pin function for pin P5\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P5_3 = 0,
    #[doc = "4: Receiver input for USART4."]
    U4_RXD = 4,
    #[doc = "5: I2C0 clock                                             input/output (this pin uses a specialized                                                 I2C pad that supports                                                 I2C Fast Mode                                             Plus."]
    I2C0_SCL = 5,
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
            0 => Val(FUNC_A::P5_3),
            4 => Val(FUNC_A::U4_RXD),
            5 => Val(FUNC_A::I2C0_SCL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P5_3`"]
    #[inline(always)]
    pub fn is_p5_3(&self) -> bool {
        *self == FUNC_A::P5_3
    }
    #[doc = "Checks if the value of the field is `U4_RXD`"]
    #[inline(always)]
    pub fn is_u4_rxd(&self) -> bool {
        *self == FUNC_A::U4_RXD
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
    pub fn p5_3(self) -> &'a mut W {
        self.variant(FUNC_A::P5_3)
    }
    #[doc = "Receiver input for USART4."]
    #[inline(always)]
    pub fn u4_rxd(self) -> &'a mut W {
        self.variant(FUNC_A::U4_RXD)
    }
    #[doc = "I2C0 clock input/output (this pin uses a specialized I2C pad that supports I2C Fast Mode Plus."]
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
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_A {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1, LOW on pin\r\n                                reads as 0)."]
    INPUT_NOT_INVERTED_ = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as\r\n                                1)."]
    INPUT_INVERTED_HIGH = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV`"]
pub type INV_R = crate::R<bool, INV_A>;
impl INV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::INPUT_NOT_INVERTED_,
            true => INV_A::INPUT_INVERTED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED_`"]
    #[inline(always)]
    pub fn is_input_not_inverted_(&self) -> bool {
        *self == INV_A::INPUT_NOT_INVERTED_
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline(always)]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INV_A::INPUT_INVERTED_HIGH
    }
}
#[doc = "Write proxy for field `INV`"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn input_not_inverted_(self) -> &'a mut W {
        self.variant(INV_A::INPUT_NOT_INVERTED_)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn input_inverted_high(self) -> &'a mut W {
        self.variant(INV_A::INPUT_INVERTED_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_A {
    #[doc = "0: I2C 50ns glitch filter and slew rate control\r\n                                enabled."]
    ENABLED = 0,
    #[doc = "1: I2C 50ns glitch filter and slew rate control\r\n                                disabled."]
    DISABLED = 1,
}
impl From<HS_A> for bool {
    #[inline(always)]
    fn from(variant: HS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HS`"]
pub type HS_R = crate::R<bool, HS_A>;
impl HS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_A {
        match self.bits {
            false => HS_A::ENABLED,
            true => HS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HS_A::DISABLED
    }
}
#[doc = "Write proxy for field `HS`"]
pub struct HS_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C 50ns glitch filter and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HS_A::ENABLED)
    }
    #[doc = "I2C 50ns glitch filter and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Controls sink current capability of the pin, only for P5\\[2\\]
and P5\\[3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIDRIVE_A {
    #[doc = "0: Output drive sink is 4 mA. This is sufficient for standard\r\n                                and fast mode I2C."]
    LOWDRIVE = 0,
    #[doc = "1: Output drive sink is 20 mA. This is needed for Fast Mode\r\n                                Plus I2C. Refer to the appropriate specific device data sheet for\r\n                                details."]
    HIGHDRIVE = 1,
}
impl From<HIDRIVE_A> for bool {
    #[inline(always)]
    fn from(variant: HIDRIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIDRIVE`"]
pub type HIDRIVE_R = crate::R<bool, HIDRIVE_A>;
impl HIDRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIDRIVE_A {
        match self.bits {
            false => HIDRIVE_A::LOWDRIVE,
            true => HIDRIVE_A::HIGHDRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOWDRIVE`"]
    #[inline(always)]
    pub fn is_lowdrive(&self) -> bool {
        *self == HIDRIVE_A::LOWDRIVE
    }
    #[doc = "Checks if the value of the field is `HIGHDRIVE`"]
    #[inline(always)]
    pub fn is_highdrive(&self) -> bool {
        *self == HIDRIVE_A::HIGHDRIVE
    }
}
#[doc = "Write proxy for field `HIDRIVE`"]
pub struct HIDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIDRIVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline(always)]
    pub fn lowdrive(self) -> &'a mut W {
        self.variant(HIDRIVE_A::LOWDRIVE)
    }
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn highdrive(self) -> &'a mut W {
        self.variant(HIDRIVE_A::HIGHDRIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P5\\[3\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5\\[2\\]
and P5\\[3\\]."]
    #[inline(always)]
    pub fn hidrive(&self) -> HIDRIVE_R {
        HIDRIVE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P5\\[3\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W { w: self }
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5\\[2\\]
and P5\\[3\\]."]
    #[inline(always)]
    pub fn hidrive(&mut self) -> HIDRIVE_W {
        HIDRIVE_W { w: self }
    }
}
