#[doc = "Reader of register SCICTRL"]
pub type R = crate::R<u32, super::SCICTRL>;
#[doc = "Writer for register SCICTRL"]
pub type W = crate::W<u32, super::SCICTRL>;
#[doc = "Register SCICTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SCICTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Smart Card Interface Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCIEN_A {
    #[doc = "0: Smart card interface disabled."]
    SMART_CARD_INTERFACE = 0,
    #[doc = "1: Asynchronous half duplex smart card interface is enabled."]
    ASYNCHRONOUS_HALF_DU = 1,
}
impl From<SCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCIEN`"]
pub type SCIEN_R = crate::R<bool, SCIEN_A>;
impl SCIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCIEN_A {
        match self.bits {
            false => SCIEN_A::SMART_CARD_INTERFACE,
            true => SCIEN_A::ASYNCHRONOUS_HALF_DU,
        }
    }
    #[doc = "Checks if the value of the field is `SMART_CARD_INTERFACE`"]
    #[inline(always)]
    pub fn is_smart_card_interface(&self) -> bool {
        *self == SCIEN_A::SMART_CARD_INTERFACE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_HALF_DU`"]
    #[inline(always)]
    pub fn is_asynchronous_half_du(&self) -> bool {
        *self == SCIEN_A::ASYNCHRONOUS_HALF_DU
    }
}
#[doc = "Write proxy for field `SCIEN`"]
pub struct SCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Smart card interface disabled."]
    #[inline(always)]
    pub fn smart_card_interface(self) -> &'a mut W {
        self.variant(SCIEN_A::SMART_CARD_INTERFACE)
    }
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    #[inline(always)]
    pub fn asynchronous_half_du(self) -> &'a mut W {
        self.variant(SCIEN_A::ASYNCHRONOUS_HALF_DU)
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
#[doc = "NACK response disable. Only applicable in T=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKDIS_A {
    #[doc = "0: A NACK response is enabled."]
    A_NACK_RESPONSE_IS_E = 0,
    #[doc = "1: A NACK response is inhibited."]
    A_NACK_RESPONSE_IS_I = 1,
}
impl From<NACKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: NACKDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NACKDIS`"]
pub type NACKDIS_R = crate::R<bool, NACKDIS_A>;
impl NACKDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKDIS_A {
        match self.bits {
            false => NACKDIS_A::A_NACK_RESPONSE_IS_E,
            true => NACKDIS_A::A_NACK_RESPONSE_IS_I,
        }
    }
    #[doc = "Checks if the value of the field is `A_NACK_RESPONSE_IS_E`"]
    #[inline(always)]
    pub fn is_a_nack_response_is_e(&self) -> bool {
        *self == NACKDIS_A::A_NACK_RESPONSE_IS_E
    }
    #[doc = "Checks if the value of the field is `A_NACK_RESPONSE_IS_I`"]
    #[inline(always)]
    pub fn is_a_nack_response_is_i(&self) -> bool {
        *self == NACKDIS_A::A_NACK_RESPONSE_IS_I
    }
}
#[doc = "Write proxy for field `NACKDIS`"]
pub struct NACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A NACK response is enabled."]
    #[inline(always)]
    pub fn a_nack_response_is_e(self) -> &'a mut W {
        self.variant(NACKDIS_A::A_NACK_RESPONSE_IS_E)
    }
    #[doc = "A NACK response is inhibited."]
    #[inline(always)]
    pub fn a_nack_response_is_i(self) -> &'a mut W {
        self.variant(NACKDIS_A::A_NACK_RESPONSE_IS_I)
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
#[doc = "Protocol selection as defined in the ISO7816-3 standard.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTSEL_A {
    #[doc = "0: T = 0"]
    T_EQ_0 = 0,
    #[doc = "1: T = 1"]
    T_EQ_1 = 1,
}
impl From<PROTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PROTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROTSEL`"]
pub type PROTSEL_R = crate::R<bool, PROTSEL_A>;
impl PROTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTSEL_A {
        match self.bits {
            false => PROTSEL_A::T_EQ_0,
            true => PROTSEL_A::T_EQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `T_EQ_0`"]
    #[inline(always)]
    pub fn is_t_eq_0(&self) -> bool {
        *self == PROTSEL_A::T_EQ_0
    }
    #[doc = "Checks if the value of the field is `T_EQ_1`"]
    #[inline(always)]
    pub fn is_t_eq_1(&self) -> bool {
        *self == PROTSEL_A::T_EQ_1
    }
}
#[doc = "Write proxy for field `PROTSEL`"]
pub struct PROTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "T = 0"]
    #[inline(always)]
    pub fn t_eq_0(self) -> &'a mut W {
        self.variant(PROTSEL_A::T_EQ_0)
    }
    #[doc = "T = 1"]
    #[inline(always)]
    pub fn t_eq_1(self) -> &'a mut W {
        self.variant(PROTSEL_A::T_EQ_1)
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
#[doc = "Reader of field `TXRETRY`"]
pub type TXRETRY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXRETRY`"]
pub struct TXRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRETRY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `GUARDTIME`"]
pub type GUARDTIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GUARDTIME`"]
pub struct GUARDTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> GUARDTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
    #[inline(always)]
    pub fn guardtime(&self) -> GUARDTIME_R {
        GUARDTIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> SCIEN_W {
        SCIEN_W { w: self }
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> NACKDIS_W {
        NACKDIS_W { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> PROTSEL_W {
        PROTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
    #[inline(always)]
    pub fn txretry(&mut self) -> TXRETRY_W {
        TXRETRY_W { w: self }
    }
    #[doc = "Bits 8:15 - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
    #[inline(always)]
    pub fn guardtime(&mut self) -> GUARDTIME_W {
        GUARDTIME_W { w: self }
    }
}
