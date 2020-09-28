#[doc = "Reader of register RS485CTRL"]
pub type R = crate::R<u32, super::RS485CTRL>;
#[doc = "Writer for register RS485CTRL"]
pub type W = crate::W<u32, super::RS485CTRL>;
#[doc = "Register RS485CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RS485CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "NMM enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMMEN_A {
    #[doc = "0: RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED = 0,
    #[doc = "1: RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt. See Section 20.6.18 RS-485/EIA-485 modes of operation."]
    ENABLED = 1,
}
impl From<NMMEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMMEN`"]
pub type NMMEN_R = crate::R<bool, NMMEN_A>;
impl NMMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMMEN_A {
        match self.bits {
            false => NMMEN_A::DISABLED,
            true => NMMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NMMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NMMEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `NMMEN`"]
pub struct NMMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NMMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NMMEN_A::DISABLED)
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt. See Section 20.6.18 RS-485/EIA-485 modes of operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NMMEN_A::ENABLED)
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
#[doc = "Receiver enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDIS_A {
    #[doc = "0: Enabled."]
    ENABLED_ = 0,
    #[doc = "1: Disabled."]
    DISABLED_ = 1,
}
impl From<RXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDIS`"]
pub type RXDIS_R = crate::R<bool, RXDIS_A>;
impl RXDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDIS_A {
        match self.bits {
            false => RXDIS_A::ENABLED_,
            true => RXDIS_A::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == RXDIS_A::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == RXDIS_A::DISABLED_
    }
}
#[doc = "Write proxy for field `RXDIS`"]
pub struct RXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(RXDIS_A::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(RXDIS_A::DISABLED_)
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
#[doc = "AAD enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AADEN_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Enabled."]
    ENABLED_ = 1,
}
impl From<AADEN_A> for bool {
    #[inline(always)]
    fn from(variant: AADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AADEN`"]
pub type AADEN_R = crate::R<bool, AADEN_A>;
impl AADEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AADEN_A {
        match self.bits {
            false => AADEN_A::DISABLED_,
            true => AADEN_A::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == AADEN_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == AADEN_A::ENABLED_
    }
}
#[doc = "Write proxy for field `AADEN`"]
pub struct AADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AADEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(AADEN_A::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(AADEN_A::ENABLED_)
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
#[doc = "Direction control for DIR pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTRL_A {
    #[doc = "0: Disable Auto Direction Control."]
    DISABLE_AUTO_DIRECTI = 0,
    #[doc = "1: Enable Auto Direction Control."]
    ENABLE_AUTO_DIRECTIO = 1,
}
impl From<DCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCTRL`"]
pub type DCTRL_R = crate::R<bool, DCTRL_A>;
impl DCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTRL_A {
        match self.bits {
            false => DCTRL_A::DISABLE_AUTO_DIRECTI,
            true => DCTRL_A::ENABLE_AUTO_DIRECTIO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_DIRECTI`"]
    #[inline(always)]
    pub fn is_disable_auto_directi(&self) -> bool {
        *self == DCTRL_A::DISABLE_AUTO_DIRECTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_DIRECTIO`"]
    #[inline(always)]
    pub fn is_enable_auto_directio(&self) -> bool {
        *self == DCTRL_A::ENABLE_AUTO_DIRECTIO
    }
}
#[doc = "Write proxy for field `DCTRL`"]
pub struct DCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn disable_auto_directi(self) -> &'a mut W {
        self.variant(DCTRL_A::DISABLE_AUTO_DIRECTI)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enable_auto_directio(self) -> &'a mut W {
        self.variant(DCTRL_A::ENABLE_AUTO_DIRECTIO)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Direction control pin polarity. This bit reverses the polarity of the direction control signal on the DIR pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OINV_A {
    #[doc = "0: Low. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW_THE_DIRECTION_C = 0,
    #[doc = "1: High. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH_THE_DIRECTION_ = 1,
}
impl From<OINV_A> for bool {
    #[inline(always)]
    fn from(variant: OINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OINV`"]
pub type OINV_R = crate::R<bool, OINV_A>;
impl OINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OINV_A {
        match self.bits {
            false => OINV_A::LOW_THE_DIRECTION_C,
            true => OINV_A::HIGH_THE_DIRECTION_,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_THE_DIRECTION_C`"]
    #[inline(always)]
    pub fn is_low_the_direction_c(&self) -> bool {
        *self == OINV_A::LOW_THE_DIRECTION_C
    }
    #[doc = "Checks if the value of the field is `HIGH_THE_DIRECTION_`"]
    #[inline(always)]
    pub fn is_high_the_direction_(&self) -> bool {
        *self == OINV_A::HIGH_THE_DIRECTION_
    }
}
#[doc = "Write proxy for field `OINV`"]
pub struct OINV_W<'a> {
    w: &'a mut W,
}
impl<'a> OINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn low_the_direction_c(self) -> &'a mut W {
        self.variant(OINV_A::LOW_THE_DIRECTION_C)
    }
    #[doc = "High. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn high_the_direction_(self) -> &'a mut W {
        self.variant(OINV_A::HIGH_THE_DIRECTION_)
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
impl R {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&self) -> NMMEN_R {
        NMMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&self) -> RXDIS_R {
        RXDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AAD enable"]
    #[inline(always)]
    pub fn aaden(&self) -> AADEN_R {
        AADEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Direction control for DIR pin."]
    #[inline(always)]
    pub fn dctrl(&self) -> DCTRL_R {
        DCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the DIR pin."]
    #[inline(always)]
    pub fn oinv(&self) -> OINV_R {
        OINV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&mut self) -> NMMEN_W {
        NMMEN_W { w: self }
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W {
        RXDIS_W { w: self }
    }
    #[doc = "Bit 2 - AAD enable"]
    #[inline(always)]
    pub fn aaden(&mut self) -> AADEN_W {
        AADEN_W { w: self }
    }
    #[doc = "Bit 4 - Direction control for DIR pin."]
    #[inline(always)]
    pub fn dctrl(&mut self) -> DCTRL_W {
        DCTRL_W { w: self }
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the DIR pin."]
    #[inline(always)]
    pub fn oinv(&mut self) -> OINV_W {
        OINV_W { w: self }
    }
}
