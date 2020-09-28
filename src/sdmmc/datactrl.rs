#[doc = "Reader of register DATACTRL"]
pub type R = crate::R<u32, super::DATACTRL>;
#[doc = "Writer for register DATACTRL"]
pub type W = crate::W<u32, super::DATACTRL>;
#[doc = "Register DATACTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DATACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTION_A {
    #[doc = "0: From controller to card."]
    FROM_CONTROLLER_TO_C = 0,
    #[doc = "1: From card to controller."]
    FROM_CARD_TO_CONTROL = 1,
}
impl From<DIRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRECTION`"]
pub type DIRECTION_R = crate::R<bool, DIRECTION_A>;
impl DIRECTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECTION_A {
        match self.bits {
            false => DIRECTION_A::FROM_CONTROLLER_TO_C,
            true => DIRECTION_A::FROM_CARD_TO_CONTROL,
        }
    }
    #[doc = "Checks if the value of the field is `FROM_CONTROLLER_TO_C`"]
    #[inline(always)]
    pub fn is_from_controller_to_c(&self) -> bool {
        *self == DIRECTION_A::FROM_CONTROLLER_TO_C
    }
    #[doc = "Checks if the value of the field is `FROM_CARD_TO_CONTROL`"]
    #[inline(always)]
    pub fn is_from_card_to_control(&self) -> bool {
        *self == DIRECTION_A::FROM_CARD_TO_CONTROL
    }
}
#[doc = "Write proxy for field `DIRECTION`"]
pub struct DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "From controller to card."]
    #[inline(always)]
    pub fn from_controller_to_c(self) -> &'a mut W {
        self.variant(DIRECTION_A::FROM_CONTROLLER_TO_C)
    }
    #[doc = "From card to controller."]
    #[inline(always)]
    pub fn from_card_to_control(self) -> &'a mut W {
        self.variant(DIRECTION_A::FROM_CARD_TO_CONTROL)
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
#[doc = "Data transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Block data transfer."]
    BLOCK_DATA_TRANSFER_ = 0,
    #[doc = "1: Stream data transfer."]
    STREAM_DATA_TRANSFER = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::BLOCK_DATA_TRANSFER_,
            true => MODE_A::STREAM_DATA_TRANSFER,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK_DATA_TRANSFER_`"]
    #[inline(always)]
    pub fn is_block_data_transfer_(&self) -> bool {
        *self == MODE_A::BLOCK_DATA_TRANSFER_
    }
    #[doc = "Checks if the value of the field is `STREAM_DATA_TRANSFER`"]
    #[inline(always)]
    pub fn is_stream_data_transfer(&self) -> bool {
        *self == MODE_A::STREAM_DATA_TRANSFER
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
            self.bit(variant.into())
        }
    }
    #[doc = "Block data transfer."]
    #[inline(always)]
    pub fn block_data_transfer_(self) -> &'a mut W {
        self.variant(MODE_A::BLOCK_DATA_TRANSFER_)
    }
    #[doc = "Stream data transfer."]
    #[inline(always)]
    pub fn stream_data_transfer(self) -> &'a mut W {
        self.variant(MODE_A::STREAM_DATA_TRANSFER)
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
#[doc = "Enable DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENABLE_A {
    #[doc = "0: DMA disabled."]
    DMA_DISABLED_ = 0,
    #[doc = "1: DMA enabled."]
    DMA_ENABLED_ = 1,
}
impl From<DMAENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAENABLE`"]
pub type DMAENABLE_R = crate::R<bool, DMAENABLE_A>;
impl DMAENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAENABLE_A {
        match self.bits {
            false => DMAENABLE_A::DMA_DISABLED_,
            true => DMAENABLE_A::DMA_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DISABLED_`"]
    #[inline(always)]
    pub fn is_dma_disabled_(&self) -> bool {
        *self == DMAENABLE_A::DMA_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DMA_ENABLED_`"]
    #[inline(always)]
    pub fn is_dma_enabled_(&self) -> bool {
        *self == DMAENABLE_A::DMA_ENABLED_
    }
}
#[doc = "Write proxy for field `DMAENABLE`"]
pub struct DMAENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn dma_disabled_(self) -> &'a mut W {
        self.variant(DMAENABLE_A::DMA_DISABLED_)
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn dma_enabled_(self) -> &'a mut W {
        self.variant(DMAENABLE_A::DMA_ENABLED_)
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
#[doc = "Reader of field `BLOCKSIZE`"]
pub type BLOCKSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLOCKSIZE`"]
pub struct BLOCKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DMAENABLE_R {
        DMAENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&mut self) -> DIRECTION_W {
        DIRECTION_W { w: self }
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline(always)]
    pub fn dmaenable(&mut self) -> DMAENABLE_W {
        DMAENABLE_W { w: self }
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W {
        BLOCKSIZE_W { w: self }
    }
}
