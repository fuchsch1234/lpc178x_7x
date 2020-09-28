#[doc = "Reader of register ERCONTROL"]
pub type R = crate::R<u32, super::ERCONTROL>;
#[doc = "Writer for register ERCONTROL"]
pub type W = crate::W<u32, super::ERCONTROL>;
#[doc = "Register ERCONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::ERCONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt and wakeup enable for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTWAKE_EN0_A {
    #[doc = "0: No interrupt or wakeup will be generated by event channel 0."]
    NO_INTERRUPT_OR_WAKE = 0,
    #[doc = "1: An event in channel 0 will trigger an (RTC) interrupt and a wake-up request."]
    AN_EVENT_IN_CHANNEL_ = 1,
}
impl From<INTWAKE_EN0_A> for bool {
    #[inline(always)]
    fn from(variant: INTWAKE_EN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTWAKE_EN0`"]
pub type INTWAKE_EN0_R = crate::R<bool, INTWAKE_EN0_A>;
impl INTWAKE_EN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTWAKE_EN0_A {
        match self.bits {
            false => INTWAKE_EN0_A::NO_INTERRUPT_OR_WAKE,
            true => INTWAKE_EN0_A::AN_EVENT_IN_CHANNEL_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_OR_WAKE`"]
    #[inline(always)]
    pub fn is_no_interrupt_or_wake(&self) -> bool {
        *self == INTWAKE_EN0_A::NO_INTERRUPT_OR_WAKE
    }
    #[doc = "Checks if the value of the field is `AN_EVENT_IN_CHANNEL_`"]
    #[inline(always)]
    pub fn is_an_event_in_channel_(&self) -> bool {
        *self == INTWAKE_EN0_A::AN_EVENT_IN_CHANNEL_
    }
}
#[doc = "Write proxy for field `INTWAKE_EN0`"]
pub struct INTWAKE_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTWAKE_EN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTWAKE_EN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt or wakeup will be generated by event channel 0."]
    #[inline(always)]
    pub fn no_interrupt_or_wake(self) -> &'a mut W {
        self.variant(INTWAKE_EN0_A::NO_INTERRUPT_OR_WAKE)
    }
    #[doc = "An event in channel 0 will trigger an (RTC) interrupt and a wake-up request."]
    #[inline(always)]
    pub fn an_event_in_channel_(self) -> &'a mut W {
        self.variant(INTWAKE_EN0_A::AN_EVENT_IN_CHANNEL_)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Enables automatically clearing the RTC general purpose registers when an event occurs on channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCLEAR_EN0_A {
    #[doc = "0: Channel 0 has no influence on the general purpose registers."]
    NOGPREG = 0,
    #[doc = "1: An event in channel 0 will clear the general purpose registers asynchronously."]
    CLRGPREG = 1,
}
impl From<GPCLEAR_EN0_A> for bool {
    #[inline(always)]
    fn from(variant: GPCLEAR_EN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPCLEAR_EN0`"]
pub type GPCLEAR_EN0_R = crate::R<bool, GPCLEAR_EN0_A>;
impl GPCLEAR_EN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCLEAR_EN0_A {
        match self.bits {
            false => GPCLEAR_EN0_A::NOGPREG,
            true => GPCLEAR_EN0_A::CLRGPREG,
        }
    }
    #[doc = "Checks if the value of the field is `NOGPREG`"]
    #[inline(always)]
    pub fn is_nogpreg(&self) -> bool {
        *self == GPCLEAR_EN0_A::NOGPREG
    }
    #[doc = "Checks if the value of the field is `CLRGPREG`"]
    #[inline(always)]
    pub fn is_clrgpreg(&self) -> bool {
        *self == GPCLEAR_EN0_A::CLRGPREG
    }
}
#[doc = "Write proxy for field `GPCLEAR_EN0`"]
pub struct GPCLEAR_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCLEAR_EN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCLEAR_EN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 0 has no influence on the general purpose registers."]
    #[inline(always)]
    pub fn nogpreg(self) -> &'a mut W {
        self.variant(GPCLEAR_EN0_A::NOGPREG)
    }
    #[doc = "An event in channel 0 will clear the general purpose registers asynchronously."]
    #[inline(always)]
    pub fn clrgpreg(self) -> &'a mut W {
        self.variant(GPCLEAR_EN0_A::CLRGPREG)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Selects the polarity of an event on input pin RTC_EV0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL0_A {
    #[doc = "0: A channel 0 event is defined as a negative edge on RTC_EV0."]
    NEG = 0,
    #[doc = "1: A channel 0 event is defined as a positive edge on RTC_EV0."]
    POS = 1,
}
impl From<POL0_A> for bool {
    #[inline(always)]
    fn from(variant: POL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL0`"]
pub type POL0_R = crate::R<bool, POL0_A>;
impl POL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL0_A {
        match self.bits {
            false => POL0_A::NEG,
            true => POL0_A::POS,
        }
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == POL0_A::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == POL0_A::POS
    }
}
#[doc = "Write proxy for field `POL0`"]
pub struct POL0_W<'a> {
    w: &'a mut W,
}
impl<'a> POL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A channel 0 event is defined as a negative edge on RTC_EV0."]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(POL0_A::NEG)
    }
    #[doc = "A channel 0 event is defined as a positive edge on RTC_EV0."]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(POL0_A::POS)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Event enable control for channel 0.\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV0_INPUT_EN_A {
    #[doc = "0: Event 0 input is disabled and forced high internally."]
    DISABLED = 0,
    #[doc = "1: Event 0 input is enabled."]
    ENABLED = 1,
}
impl From<EV0_INPUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EV0_INPUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EV0_INPUT_EN`"]
pub type EV0_INPUT_EN_R = crate::R<bool, EV0_INPUT_EN_A>;
impl EV0_INPUT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV0_INPUT_EN_A {
        match self.bits {
            false => EV0_INPUT_EN_A::DISABLED,
            true => EV0_INPUT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EV0_INPUT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EV0_INPUT_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `EV0_INPUT_EN`"]
pub struct EV0_INPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_INPUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV0_INPUT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event 0 input is disabled and forced high internally."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EV0_INPUT_EN_A::DISABLED)
    }
    #[doc = "Event 0 input is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EV0_INPUT_EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Interrupt and wakeup enable for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTWAKE_EN1_A {
    #[doc = "0: No interrupt or wakeup will be generated by event channel 1."]
    NO_INTERRUPT_OR_WAKE = 0,
    #[doc = "1: An event in channel 1 will trigger an (RTC) interrupt and a wake-up request."]
    WAKEUP = 1,
}
impl From<INTWAKE_EN1_A> for bool {
    #[inline(always)]
    fn from(variant: INTWAKE_EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTWAKE_EN1`"]
pub type INTWAKE_EN1_R = crate::R<bool, INTWAKE_EN1_A>;
impl INTWAKE_EN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTWAKE_EN1_A {
        match self.bits {
            false => INTWAKE_EN1_A::NO_INTERRUPT_OR_WAKE,
            true => INTWAKE_EN1_A::WAKEUP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_OR_WAKE`"]
    #[inline(always)]
    pub fn is_no_interrupt_or_wake(&self) -> bool {
        *self == INTWAKE_EN1_A::NO_INTERRUPT_OR_WAKE
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == INTWAKE_EN1_A::WAKEUP
    }
}
#[doc = "Write proxy for field `INTWAKE_EN1`"]
pub struct INTWAKE_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTWAKE_EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTWAKE_EN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt or wakeup will be generated by event channel 1."]
    #[inline(always)]
    pub fn no_interrupt_or_wake(self) -> &'a mut W {
        self.variant(INTWAKE_EN1_A::NO_INTERRUPT_OR_WAKE)
    }
    #[doc = "An event in channel 1 will trigger an (RTC) interrupt and a wake-up request."]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(INTWAKE_EN1_A::WAKEUP)
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
#[doc = "Enables automatically clearing the RTC general purpose registers when an event occurs on channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCLEAR_EN1_A {
    #[doc = "0: Channel 1 has no influence on the general purpose registers."]
    NOGPREG = 0,
    #[doc = "1: A n event in channel 1 will clear the general purpose registers asynchronously."]
    CLRGPREG = 1,
}
impl From<GPCLEAR_EN1_A> for bool {
    #[inline(always)]
    fn from(variant: GPCLEAR_EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPCLEAR_EN1`"]
pub type GPCLEAR_EN1_R = crate::R<bool, GPCLEAR_EN1_A>;
impl GPCLEAR_EN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCLEAR_EN1_A {
        match self.bits {
            false => GPCLEAR_EN1_A::NOGPREG,
            true => GPCLEAR_EN1_A::CLRGPREG,
        }
    }
    #[doc = "Checks if the value of the field is `NOGPREG`"]
    #[inline(always)]
    pub fn is_nogpreg(&self) -> bool {
        *self == GPCLEAR_EN1_A::NOGPREG
    }
    #[doc = "Checks if the value of the field is `CLRGPREG`"]
    #[inline(always)]
    pub fn is_clrgpreg(&self) -> bool {
        *self == GPCLEAR_EN1_A::CLRGPREG
    }
}
#[doc = "Write proxy for field `GPCLEAR_EN1`"]
pub struct GPCLEAR_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCLEAR_EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCLEAR_EN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 1 has no influence on the general purpose registers."]
    #[inline(always)]
    pub fn nogpreg(self) -> &'a mut W {
        self.variant(GPCLEAR_EN1_A::NOGPREG)
    }
    #[doc = "A n event in channel 1 will clear the general purpose registers asynchronously."]
    #[inline(always)]
    pub fn clrgpreg(self) -> &'a mut W {
        self.variant(GPCLEAR_EN1_A::CLRGPREG)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Selects the polarity of an event on input pin RTC_EV1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL1_A {
    #[doc = "0: A channel 1 event is defined as a negative edge on RTC_EV1."]
    NEG = 0,
    #[doc = "1: A channel 1 event is defined as a positive edge on RTC_EV1."]
    POS = 1,
}
impl From<POL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL1`"]
pub type POL1_R = crate::R<bool, POL1_A>;
impl POL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL1_A {
        match self.bits {
            false => POL1_A::NEG,
            true => POL1_A::POS,
        }
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == POL1_A::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == POL1_A::POS
    }
}
#[doc = "Write proxy for field `POL1`"]
pub struct POL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A channel 1 event is defined as a negative edge on RTC_EV1."]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(POL1_A::NEG)
    }
    #[doc = "A channel 1 event is defined as a positive edge on RTC_EV1."]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(POL1_A::POS)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Event enable control for channel 1.\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV1_INPUT_EN_A {
    #[doc = "0: Event 1 input is disabled and forced high internally."]
    DISABLED = 0,
    #[doc = "1: Event 1 input is enabled."]
    ENABLED = 1,
}
impl From<EV1_INPUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EV1_INPUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EV1_INPUT_EN`"]
pub type EV1_INPUT_EN_R = crate::R<bool, EV1_INPUT_EN_A>;
impl EV1_INPUT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV1_INPUT_EN_A {
        match self.bits {
            false => EV1_INPUT_EN_A::DISABLED,
            true => EV1_INPUT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EV1_INPUT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EV1_INPUT_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `EV1_INPUT_EN`"]
pub struct EV1_INPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_INPUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV1_INPUT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event 1 input is disabled and forced high internally."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EV1_INPUT_EN_A::DISABLED)
    }
    #[doc = "Event 1 input is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EV1_INPUT_EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Interrupt and wakeup enable for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTWAKE_EN2_A {
    #[doc = "0: No interrupt or wakeup will be generated by event channel 2."]
    NO_INTERRUPT_OR_WAKE = 0,
    #[doc = "1: An event in channel 2 will trigger an (RTC) interrupt and a wake-up request."]
    WAKEUP = 1,
}
impl From<INTWAKE_EN2_A> for bool {
    #[inline(always)]
    fn from(variant: INTWAKE_EN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTWAKE_EN2`"]
pub type INTWAKE_EN2_R = crate::R<bool, INTWAKE_EN2_A>;
impl INTWAKE_EN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTWAKE_EN2_A {
        match self.bits {
            false => INTWAKE_EN2_A::NO_INTERRUPT_OR_WAKE,
            true => INTWAKE_EN2_A::WAKEUP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_OR_WAKE`"]
    #[inline(always)]
    pub fn is_no_interrupt_or_wake(&self) -> bool {
        *self == INTWAKE_EN2_A::NO_INTERRUPT_OR_WAKE
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == INTWAKE_EN2_A::WAKEUP
    }
}
#[doc = "Write proxy for field `INTWAKE_EN2`"]
pub struct INTWAKE_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTWAKE_EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTWAKE_EN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt or wakeup will be generated by event channel 2."]
    #[inline(always)]
    pub fn no_interrupt_or_wake(self) -> &'a mut W {
        self.variant(INTWAKE_EN2_A::NO_INTERRUPT_OR_WAKE)
    }
    #[doc = "An event in channel 2 will trigger an (RTC) interrupt and a wake-up request."]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(INTWAKE_EN2_A::WAKEUP)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Enables automatically clearing the RTC general purpose registers when an event occurs on channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCLEAR_EN2_A {
    #[doc = "0: Channel 2 has no influence on the general purpose registers."]
    NOGPREG = 0,
    #[doc = "1: An event in channel 2 will clear the general purpose registers asynchronously."]
    CLRGPREG = 1,
}
impl From<GPCLEAR_EN2_A> for bool {
    #[inline(always)]
    fn from(variant: GPCLEAR_EN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPCLEAR_EN2`"]
pub type GPCLEAR_EN2_R = crate::R<bool, GPCLEAR_EN2_A>;
impl GPCLEAR_EN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPCLEAR_EN2_A {
        match self.bits {
            false => GPCLEAR_EN2_A::NOGPREG,
            true => GPCLEAR_EN2_A::CLRGPREG,
        }
    }
    #[doc = "Checks if the value of the field is `NOGPREG`"]
    #[inline(always)]
    pub fn is_nogpreg(&self) -> bool {
        *self == GPCLEAR_EN2_A::NOGPREG
    }
    #[doc = "Checks if the value of the field is `CLRGPREG`"]
    #[inline(always)]
    pub fn is_clrgpreg(&self) -> bool {
        *self == GPCLEAR_EN2_A::CLRGPREG
    }
}
#[doc = "Write proxy for field `GPCLEAR_EN2`"]
pub struct GPCLEAR_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCLEAR_EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPCLEAR_EN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 2 has no influence on the general purpose registers."]
    #[inline(always)]
    pub fn nogpreg(self) -> &'a mut W {
        self.variant(GPCLEAR_EN2_A::NOGPREG)
    }
    #[doc = "An event in channel 2 will clear the general purpose registers asynchronously."]
    #[inline(always)]
    pub fn clrgpreg(self) -> &'a mut W {
        self.variant(GPCLEAR_EN2_A::CLRGPREG)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Selects the polarity of an event on input pin RTC_EV2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL2_A {
    #[doc = "0: A channel 2 event is defined as a negative edge on RTC_EV2."]
    NEG = 0,
    #[doc = "1: A channel 2 event is defined as a positive edge on RTC_EV2."]
    POS = 1,
}
impl From<POL2_A> for bool {
    #[inline(always)]
    fn from(variant: POL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL2`"]
pub type POL2_R = crate::R<bool, POL2_A>;
impl POL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL2_A {
        match self.bits {
            false => POL2_A::NEG,
            true => POL2_A::POS,
        }
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == POL2_A::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == POL2_A::POS
    }
}
#[doc = "Write proxy for field `POL2`"]
pub struct POL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A channel 2 event is defined as a negative edge on RTC_EV2."]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(POL2_A::NEG)
    }
    #[doc = "A channel 2 event is defined as a positive edge on RTC_EV2."]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(POL2_A::POS)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Event enable control for channel 2.\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV2_INPUT_EN_A {
    #[doc = "0: Event 2 input is disabled and forced high internally."]
    DISABLED = 0,
    #[doc = "1: Event 2 input is enabled."]
    ENABLED = 1,
}
impl From<EV2_INPUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EV2_INPUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EV2_INPUT_EN`"]
pub type EV2_INPUT_EN_R = crate::R<bool, EV2_INPUT_EN_A>;
impl EV2_INPUT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EV2_INPUT_EN_A {
        match self.bits {
            false => EV2_INPUT_EN_A::DISABLED,
            true => EV2_INPUT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EV2_INPUT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EV2_INPUT_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `EV2_INPUT_EN`"]
pub struct EV2_INPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_INPUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV2_INPUT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event 2 input is disabled and forced high internally."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EV2_INPUT_EN_A::DISABLED)
    }
    #[doc = "Event 2 input is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EV2_INPUT_EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Controls enabling the Event Monitor/Recorder and selecting its operating frequency.\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERMODE_A {
    #[doc = "0: Event Monitor/Recorder clocks are disabled. Operation of the Event Monitor/Recorder is disabled except for asynchronous clearing of GP registers if selected."]
    DISABLED = 0,
    #[doc = "1: Enable Event Monitor/Recorder and select a 16 Hz sample clock for event input edge detection and glitch suppression. Pulses (in either direction) shorter than 62.5 ms to 125 ms will be filtered out."]
    ENABLE_EVENT_MONITOR16HZ = 1,
    #[doc = "2: Enable Event Monitor/Recorder and select a 64 Hz sample clock for event input edge detection and glitch suppression. Pulses (in either direction) shorter than 15.6 ms to 31.2 ms will be filtered out."]
    ENABLE_EVENT_MONITOR64HZ = 2,
    #[doc = "3: Enable Event Monitor/Recorder and select a 1 kHz sample clock for event input edge detection and glitch suppression. Pulses (in either direction) shorter than 1 ms to 2 ms will be filtered out."]
    ENABLE_EVENT_MONITOR1KHZ = 3,
}
impl From<ERMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ERMODE`"]
pub type ERMODE_R = crate::R<u8, ERMODE_A>;
impl ERMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERMODE_A {
        match self.bits {
            0 => ERMODE_A::DISABLED,
            1 => ERMODE_A::ENABLE_EVENT_MONITOR16HZ,
            2 => ERMODE_A::ENABLE_EVENT_MONITOR64HZ,
            3 => ERMODE_A::ENABLE_EVENT_MONITOR1KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLE_EVENT_MONITOR16HZ`"]
    #[inline(always)]
    pub fn is_enable_event_monitor16hz(&self) -> bool {
        *self == ERMODE_A::ENABLE_EVENT_MONITOR16HZ
    }
    #[doc = "Checks if the value of the field is `ENABLE_EVENT_MONITOR64HZ`"]
    #[inline(always)]
    pub fn is_enable_event_monitor64hz(&self) -> bool {
        *self == ERMODE_A::ENABLE_EVENT_MONITOR64HZ
    }
    #[doc = "Checks if the value of the field is `ENABLE_EVENT_MONITOR1KHZ`"]
    #[inline(always)]
    pub fn is_enable_event_monitor1khz(&self) -> bool {
        *self == ERMODE_A::ENABLE_EVENT_MONITOR1KHZ
    }
}
#[doc = "Write proxy for field `ERMODE`"]
pub struct ERMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Event Monitor/Recorder clocks are disabled. Operation of the Event Monitor/Recorder is disabled except for asynchronous clearing of GP registers if selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERMODE_A::DISABLED)
    }
    #[doc = "Enable Event Monitor/Recorder and select a 16 Hz sample clock for event input edge detection and glitch suppression. Pulses (in either direction) shorter than 62.5 ms to 125 ms will be filtered out."]
    #[inline(always)]
    pub fn enable_event_monitor16hz(self) -> &'a mut W {
        self.variant(ERMODE_A::ENABLE_EVENT_MONITOR16HZ)
    }
    #[doc = "Enable Event Monitor/Recorder and select a 64 Hz sample clock for event input edge detection and glitch suppression. Pulses (in either direction) shorter than 15.6 ms to 31.2 ms will be filtered out."]
    #[inline(always)]
    pub fn enable_event_monitor64hz(self) -> &'a mut W {
        self.variant(ERMODE_A::ENABLE_EVENT_MONITOR64HZ)
    }
    #[doc = "Enable Event Monitor/Recorder and select a 1 kHz sample clock for event input edge detection and glitch suppression. Pulses (in either direction) shorter than 1 ms to 2 ms will be filtered out."]
    #[inline(always)]
    pub fn enable_event_monitor1khz(self) -> &'a mut W {
        self.variant(ERMODE_A::ENABLE_EVENT_MONITOR1KHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt and wakeup enable for channel 0."]
    #[inline(always)]
    pub fn intwake_en0(&self) -> INTWAKE_EN0_R {
        INTWAKE_EN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables automatically clearing the RTC general purpose registers when an event occurs on channel 0."]
    #[inline(always)]
    pub fn gpclear_en0(&self) -> GPCLEAR_EN0_R {
        GPCLEAR_EN0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects the polarity of an event on input pin RTC_EV0."]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Event enable control for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn ev0_input_en(&self) -> EV0_INPUT_EN_R {
        EV0_INPUT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt and wakeup enable for channel 1."]
    #[inline(always)]
    pub fn intwake_en1(&self) -> INTWAKE_EN1_R {
        INTWAKE_EN1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables automatically clearing the RTC general purpose registers when an event occurs on channel 1."]
    #[inline(always)]
    pub fn gpclear_en1(&self) -> GPCLEAR_EN1_R {
        GPCLEAR_EN1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects the polarity of an event on input pin RTC_EV1."]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Event enable control for channel 1.\\[1\\]"]
    #[inline(always)]
    pub fn ev1_input_en(&self) -> EV1_INPUT_EN_R {
        EV1_INPUT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt and wakeup enable for channel 2."]
    #[inline(always)]
    pub fn intwake_en2(&self) -> INTWAKE_EN2_R {
        INTWAKE_EN2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables automatically clearing the RTC general purpose registers when an event occurs on channel 2."]
    #[inline(always)]
    pub fn gpclear_en2(&self) -> GPCLEAR_EN2_R {
        GPCLEAR_EN2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Selects the polarity of an event on input pin RTC_EV2."]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Event enable control for channel 2.\\[1\\]"]
    #[inline(always)]
    pub fn ev2_input_en(&self) -> EV2_INPUT_EN_R {
        EV2_INPUT_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Controls enabling the Event Monitor/Recorder and selecting its operating frequency.\\[2\\]"]
    #[inline(always)]
    pub fn ermode(&self) -> ERMODE_R {
        ERMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt and wakeup enable for channel 0."]
    #[inline(always)]
    pub fn intwake_en0(&mut self) -> INTWAKE_EN0_W {
        INTWAKE_EN0_W { w: self }
    }
    #[doc = "Bit 1 - Enables automatically clearing the RTC general purpose registers when an event occurs on channel 0."]
    #[inline(always)]
    pub fn gpclear_en0(&mut self) -> GPCLEAR_EN0_W {
        GPCLEAR_EN0_W { w: self }
    }
    #[doc = "Bit 2 - Selects the polarity of an event on input pin RTC_EV0."]
    #[inline(always)]
    pub fn pol0(&mut self) -> POL0_W {
        POL0_W { w: self }
    }
    #[doc = "Bit 3 - Event enable control for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn ev0_input_en(&mut self) -> EV0_INPUT_EN_W {
        EV0_INPUT_EN_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt and wakeup enable for channel 1."]
    #[inline(always)]
    pub fn intwake_en1(&mut self) -> INTWAKE_EN1_W {
        INTWAKE_EN1_W { w: self }
    }
    #[doc = "Bit 11 - Enables automatically clearing the RTC general purpose registers when an event occurs on channel 1."]
    #[inline(always)]
    pub fn gpclear_en1(&mut self) -> GPCLEAR_EN1_W {
        GPCLEAR_EN1_W { w: self }
    }
    #[doc = "Bit 12 - Selects the polarity of an event on input pin RTC_EV1."]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W {
        POL1_W { w: self }
    }
    #[doc = "Bit 13 - Event enable control for channel 1.\\[1\\]"]
    #[inline(always)]
    pub fn ev1_input_en(&mut self) -> EV1_INPUT_EN_W {
        EV1_INPUT_EN_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt and wakeup enable for channel 2."]
    #[inline(always)]
    pub fn intwake_en2(&mut self) -> INTWAKE_EN2_W {
        INTWAKE_EN2_W { w: self }
    }
    #[doc = "Bit 21 - Enables automatically clearing the RTC general purpose registers when an event occurs on channel 2."]
    #[inline(always)]
    pub fn gpclear_en2(&mut self) -> GPCLEAR_EN2_W {
        GPCLEAR_EN2_W { w: self }
    }
    #[doc = "Bit 22 - Selects the polarity of an event on input pin RTC_EV2."]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W {
        POL2_W { w: self }
    }
    #[doc = "Bit 23 - Event enable control for channel 2.\\[1\\]"]
    #[inline(always)]
    pub fn ev2_input_en(&mut self) -> EV2_INPUT_EN_W {
        EV2_INPUT_EN_W { w: self }
    }
    #[doc = "Bits 30:31 - Controls enabling the Event Monitor/Recorder and selecting its operating frequency.\\[2\\]"]
    #[inline(always)]
    pub fn ermode(&mut self) -> ERMODE_W {
        ERMODE_W { w: self }
    }
}