#[doc = "Reader of register P0_22"]
pub type R = crate::R<u32, super::P0_22>;
#[doc = "Writer for register P0_22"]
pub type W = crate::W<u32, super::P0_22>;
#[doc = "Register P0_22 `reset()`'s with value 0x30"]
impl crate::ResetValue for super::P0_22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "Selects pin function for pin P0\\[22\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P0_22 = 0,
    #[doc = "1: Request to Send output for UART1. Can also be                                             configured to be an RS-485/EIA-485 output enable signal                                             for UART1."]
    U1_RTS = 1,
    #[doc = "2: Data line 0 for SD card interface."]
    SD_DAT_0 = 2,
    #[doc = "3: Transmitter output for USART4 (input/output in                                             smart card mode)."]
    U4_TXD = 3,
    #[doc = "4: CAN1 transmitter output."]
    CAN_TD1 = 4,
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
            0 => Val(FUNC_A::P0_22),
            1 => Val(FUNC_A::U1_RTS),
            2 => Val(FUNC_A::SD_DAT_0),
            3 => Val(FUNC_A::U4_TXD),
            4 => Val(FUNC_A::CAN_TD1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_22`"]
    #[inline(always)]
    pub fn is_p0_22(&self) -> bool {
        *self == FUNC_A::P0_22
    }
    #[doc = "Checks if the value of the field is `U1_RTS`"]
    #[inline(always)]
    pub fn is_u1_rts(&self) -> bool {
        *self == FUNC_A::U1_RTS
    }
    #[doc = "Checks if the value of the field is `SD_DAT_0`"]
    #[inline(always)]
    pub fn is_sd_dat_0(&self) -> bool {
        *self == FUNC_A::SD_DAT_0
    }
    #[doc = "Checks if the value of the field is `U4_TXD`"]
    #[inline(always)]
    pub fn is_u4_txd(&self) -> bool {
        *self == FUNC_A::U4_TXD
    }
    #[doc = "Checks if the value of the field is `CAN_TD1`"]
    #[inline(always)]
    pub fn is_can_td1(&self) -> bool {
        *self == FUNC_A::CAN_TD1
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
    pub fn p0_22(self) -> &'a mut W {
        self.variant(FUNC_A::P0_22)
    }
    #[doc = "Request to Send output for UART1. Can also be configured to be an RS-485/EIA-485 output enable signal for UART1."]
    #[inline(always)]
    pub fn u1_rts(self) -> &'a mut W {
        self.variant(FUNC_A::U1_RTS)
    }
    #[doc = "Data line 0 for SD card interface."]
    #[inline(always)]
    pub fn sd_dat_0(self) -> &'a mut W {
        self.variant(FUNC_A::SD_DAT_0)
    }
    #[doc = "Transmitter output for USART4 (input/output in smart card mode)."]
    #[inline(always)]
    pub fn u4_txd(self) -> &'a mut W {
        self.variant(FUNC_A::U4_TXD)
    }
    #[doc = "CAN1 transmitter output."]
    #[inline(always)]
    pub fn can_td1(self) -> &'a mut W {
        self.variant(FUNC_A::CAN_TD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive (no pull-down/pull-up resistor\r\n                                enabled)."]
    INACTIVE_NO_PULL_DO = 0,
    #[doc = "1: Pull-down resistor enabled."]
    PULL_DOWN_RESISTOR_E = 1,
    #[doc = "2: Pull-up resistor enabled."]
    PULL_UP_RESISTOR_ENA = 2,
    #[doc = "3: Repeater mode."]
    REPEATER_MODE_ = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::INACTIVE_NO_PULL_DO,
            1 => MODE_A::PULL_DOWN_RESISTOR_E,
            2 => MODE_A::PULL_UP_RESISTOR_ENA,
            3 => MODE_A::REPEATER_MODE_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_NO_PULL_DO`"]
    #[inline(always)]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == MODE_A::INACTIVE_NO_PULL_DO
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_RESISTOR_E`"]
    #[inline(always)]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == MODE_A::PULL_DOWN_RESISTOR_E
    }
    #[doc = "Checks if the value of the field is `PULL_UP_RESISTOR_ENA`"]
    #[inline(always)]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == MODE_A::PULL_UP_RESISTOR_ENA
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE_`"]
    #[inline(always)]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == MODE_A::REPEATER_MODE_
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive_no_pull_do(self) -> &'a mut W {
        self.variant(MODE_A::INACTIVE_NO_PULL_DO)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down_resistor_e(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN_RESISTOR_E)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up_resistor_ena(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP_RESISTOR_ENA)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater_mode_(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER_MODE_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYS_A {
    #[doc = "0: Disable."]
    DISABLE_ = 0,
    #[doc = "1: Enable."]
    ENABLE_ = 1,
}
impl From<HYS_A> for bool {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HYS`"]
pub type HYS_R = crate::R<bool, HYS_A>;
impl HYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            false => HYS_A::DISABLE_,
            true => HYS_A::ENABLE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_`"]
    #[inline(always)]
    pub fn is_disable_(&self) -> bool {
        *self == HYS_A::DISABLE_
    }
    #[doc = "Checks if the value of the field is `ENABLE_`"]
    #[inline(always)]
    pub fn is_enable_(&self) -> bool {
        *self == HYS_A::ENABLE_
    }
}
#[doc = "Write proxy for field `HYS`"]
pub struct HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable_(self) -> &'a mut W {
        self.variant(HYS_A::DISABLE_)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable_(self) -> &'a mut W {
        self.variant(HYS_A::ENABLE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
#[doc = "Selects Analog/Digital mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMODE_A {
    #[doc = "0: Analog input mode."]
    ANALOG_INPUT_MODE_ = 0,
    #[doc = "1: Digital functional mode."]
    DIGITAL_FUNCTIONAL_M = 1,
}
impl From<ADMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADMODE`"]
pub type ADMODE_R = crate::R<bool, ADMODE_A>;
impl ADMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMODE_A {
        match self.bits {
            false => ADMODE_A::ANALOG_INPUT_MODE_,
            true => ADMODE_A::DIGITAL_FUNCTIONAL_M,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT_MODE_`"]
    #[inline(always)]
    pub fn is_analog_input_mode_(&self) -> bool {
        *self == ADMODE_A::ANALOG_INPUT_MODE_
    }
    #[doc = "Checks if the value of the field is `DIGITAL_FUNCTIONAL_M`"]
    #[inline(always)]
    pub fn is_digital_functional_m(&self) -> bool {
        *self == ADMODE_A::DIGITAL_FUNCTIONAL_M
    }
}
#[doc = "Write proxy for field `ADMODE`"]
pub struct ADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Analog input mode."]
    #[inline(always)]
    pub fn analog_input_mode_(self) -> &'a mut W {
        self.variant(ADMODE_A::ANALOG_INPUT_MODE_)
    }
    #[doc = "Digital functional mode."]
    #[inline(always)]
    pub fn digital_functional_m(self) -> &'a mut W {
        self.variant(ADMODE_A::DIGITAL_FUNCTIONAL_M)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Selects 10 ns input glitch filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTR_A {
    #[doc = "0: Filter disabled."]
    FILTER_DISABLED_ = 0,
    #[doc = "1: Filter enabled."]
    FILTER_ENABLED_ = 1,
}
impl From<FILTR_A> for bool {
    #[inline(always)]
    fn from(variant: FILTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FILTR`"]
pub type FILTR_R = crate::R<bool, FILTR_A>;
impl FILTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTR_A {
        match self.bits {
            false => FILTR_A::FILTER_DISABLED_,
            true => FILTR_A::FILTER_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_DISABLED_`"]
    #[inline(always)]
    pub fn is_filter_disabled_(&self) -> bool {
        *self == FILTR_A::FILTER_DISABLED_
    }
    #[doc = "Checks if the value of the field is `FILTER_ENABLED_`"]
    #[inline(always)]
    pub fn is_filter_enabled_(&self) -> bool {
        *self == FILTR_A::FILTER_ENABLED_
    }
}
#[doc = "Write proxy for field `FILTR`"]
pub struct FILTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn filter_disabled_(self) -> &'a mut W {
        self.variant(FILTR_A::FILTER_DISABLED_)
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn filter_enabled_(self) -> &'a mut W {
        self.variant(FILTR_A::FILTER_ENABLED_)
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
#[doc = "Open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_A {
    #[doc = "0: Disable."]
    DISABLE_ = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain\r\n                                mode."]
    OPEN_DRAIN_MODE_ENAB = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD`"]
pub type OD_R = crate::R<bool, OD_A>;
impl OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::DISABLE_,
            true => OD_A::OPEN_DRAIN_MODE_ENAB,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_`"]
    #[inline(always)]
    pub fn is_disable_(&self) -> bool {
        *self == OD_A::DISABLE_
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_MODE_ENAB`"]
    #[inline(always)]
    pub fn is_open_drain_mode_enab(&self) -> bool {
        *self == OD_A::OPEN_DRAIN_MODE_ENAB
    }
}
#[doc = "Write proxy for field `OD`"]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable_(self) -> &'a mut W {
        self.variant(OD_A::DISABLE_)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode."]
    #[inline(always)]
    pub fn open_drain_mode_enab(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN_MODE_ENAB)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[22\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline(always)]
    pub fn filtr(&self) -> FILTR_R {
        FILTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[22\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W {
        HYS_W { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W {
        ADMODE_W { w: self }
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline(always)]
    pub fn filtr(&mut self) -> FILTR_W {
        FILTR_W { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
}
