#[doc = "Reader of register P0_26"]
pub type R = crate::R<u32, super::P0_26>;
#[doc = "Writer for register P0_26"]
pub type W = crate::W<u32, super::P0_26>;
#[doc = "Register P0_26 `reset()`'s with value 0x01b0"]
impl crate::ResetValue for super::P0_26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01b0
    }
}
#[doc = "Selects pin function for pin P0\\[26\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output\r\n                                            pin."]
    P0_26 = 0,
    #[doc = "1: A/D converter 0, input 3. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_3 = 1,
    #[doc = "2: D/A converter output. When configured as the DAC                                             output, the digital function of the pin must be                                             disabled."]
    DAC_OUT = 2,
    #[doc = "3: Receiver input for UART3."]
    U3_RXD = 3,
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
            0 => Val(FUNC_A::P0_26),
            1 => Val(FUNC_A::ADC0_IN_3),
            2 => Val(FUNC_A::DAC_OUT),
            3 => Val(FUNC_A::U3_RXD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_26`"]
    #[inline(always)]
    pub fn is_p0_26(&self) -> bool {
        *self == FUNC_A::P0_26
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_3`"]
    #[inline(always)]
    pub fn is_adc0_in_3(&self) -> bool {
        *self == FUNC_A::ADC0_IN_3
    }
    #[doc = "Checks if the value of the field is `DAC_OUT`"]
    #[inline(always)]
    pub fn is_dac_out(&self) -> bool {
        *self == FUNC_A::DAC_OUT
    }
    #[doc = "Checks if the value of the field is `U3_RXD`"]
    #[inline(always)]
    pub fn is_u3_rxd(&self) -> bool {
        *self == FUNC_A::U3_RXD
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
    pub fn p0_26(self) -> &'a mut W {
        self.variant(FUNC_A::P0_26)
    }
    #[doc = "A/D converter 0, input 3. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_3(self) -> &'a mut W {
        self.variant(FUNC_A::ADC0_IN_3)
    }
    #[doc = "D/A converter output. When configured as the DAC output, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn dac_out(self) -> &'a mut W {
        self.variant(FUNC_A::DAC_OUT)
    }
    #[doc = "Receiver input for UART3."]
    #[inline(always)]
    pub fn u3_rxd(self) -> &'a mut W {
        self.variant(FUNC_A::U3_RXD)
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
#[doc = "Selects Analog/Digital mode.\n\nValue on reset: 1"]
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
#[doc = "Selects 10 ns input glitch filter.\n\nValue on reset: 1"]
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
#[doc = "DAC output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: DAC disable."]
    DISABLE = 0,
    #[doc = "1: DAC enable."]
    ENABLE = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACEN`"]
pub type DACEN_R = crate::R<bool, DACEN_A>;
impl DACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::DISABLE,
            true => DACEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DACEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DACEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `DACEN`"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DACEN_A::DISABLE)
    }
    #[doc = "DAC enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DACEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[26\\]"]
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
    #[doc = "Bit 16 - DAC output enable."]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[26\\]"]
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
    #[doc = "Bit 16 - DAC output enable."]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
}
