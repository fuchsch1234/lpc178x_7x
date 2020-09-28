#[doc = "Reader of register DMACREQSEL"]
pub type R = crate::R<u32, super::DMACREQSEL>;
#[doc = "Writer for register DMACREQSEL"]
pub type W = crate::W<u32, super::DMACREQSEL>;
#[doc = "Register DMACREQSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACREQSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMASEL00`"]
pub type DMASEL00_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL00`"]
pub struct DMASEL00_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL00_W<'a> {
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
#[doc = "Reader of field `DMASEL01`"]
pub type DMASEL01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL01`"]
pub struct DMASEL01_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL01_W<'a> {
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
#[doc = "Reader of field `DMASEL02`"]
pub type DMASEL02_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL02`"]
pub struct DMASEL02_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL02_W<'a> {
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
#[doc = "Reader of field `DMASEL03`"]
pub type DMASEL03_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL03`"]
pub struct DMASEL03_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL03_W<'a> {
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
#[doc = "Reader of field `DMASEL04`"]
pub type DMASEL04_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL04`"]
pub struct DMASEL04_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL04_W<'a> {
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
#[doc = "Reader of field `DMASEL05`"]
pub type DMASEL05_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL05`"]
pub struct DMASEL05_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL05_W<'a> {
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
#[doc = "Reader of field `DMASEL06`"]
pub type DMASEL06_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL06`"]
pub struct DMASEL06_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL06_W<'a> {
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
#[doc = "Reader of field `DMASEL07`"]
pub type DMASEL07_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL07`"]
pub struct DMASEL07_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL07_W<'a> {
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
#[doc = "Reader of field `DMASEL10`"]
pub type DMASEL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL10`"]
pub struct DMASEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL10_W<'a> {
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
#[doc = "Reader of field `DMASEL11`"]
pub type DMASEL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL11`"]
pub struct DMASEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL11_W<'a> {
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
#[doc = "Reader of field `DMASEL12`"]
pub type DMASEL12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL12`"]
pub struct DMASEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL12_W<'a> {
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
#[doc = "Reader of field `DMASEL13`"]
pub type DMASEL13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL13`"]
pub struct DMASEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL13_W<'a> {
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
#[doc = "Reader of field `DMASEL14`"]
pub type DMASEL14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL14`"]
pub struct DMASEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DMASEL15`"]
pub type DMASEL15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEL15`"]
pub struct DMASEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel00(&self) -> DMASEL00_R {
        DMASEL00_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel01(&self) -> DMASEL01_R {
        DMASEL01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel02(&self) -> DMASEL02_R {
        DMASEL02_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel03(&self) -> DMASEL03_R {
        DMASEL03_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel04(&self) -> DMASEL04_R {
        DMASEL04_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel05(&self) -> DMASEL05_R {
        DMASEL05_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel06(&self) -> DMASEL06_R {
        DMASEL06_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel07(&self) -> DMASEL07_R {
        DMASEL07_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
    #[inline(always)]
    pub fn dmasel10(&self) -> DMASEL10_R {
        DMASEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
    #[inline(always)]
    pub fn dmasel11(&self) -> DMASEL11_R {
        DMASEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
    #[inline(always)]
    pub fn dmasel12(&self) -> DMASEL12_R {
        DMASEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
    #[inline(always)]
    pub fn dmasel13(&self) -> DMASEL13_R {
        DMASEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&self) -> DMASEL14_R {
        DMASEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&self) -> DMASEL15_R {
        DMASEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel00(&mut self) -> DMASEL00_W {
        DMASEL00_W { w: self }
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel01(&mut self) -> DMASEL01_W {
        DMASEL01_W { w: self }
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel02(&mut self) -> DMASEL02_W {
        DMASEL02_W { w: self }
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel03(&mut self) -> DMASEL03_W {
        DMASEL03_W { w: self }
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel04(&mut self) -> DMASEL04_W {
        DMASEL04_W { w: self }
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel05(&mut self) -> DMASEL05_W {
        DMASEL05_W { w: self }
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel06(&mut self) -> DMASEL06_W {
        DMASEL06_W { w: self }
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel07(&mut self) -> DMASEL07_W {
        DMASEL07_W { w: self }
    }
    #[doc = "Bit 10 - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
    #[inline(always)]
    pub fn dmasel10(&mut self) -> DMASEL10_W {
        DMASEL10_W { w: self }
    }
    #[doc = "Bit 11 - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
    #[inline(always)]
    pub fn dmasel11(&mut self) -> DMASEL11_W {
        DMASEL11_W { w: self }
    }
    #[doc = "Bit 12 - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
    #[inline(always)]
    pub fn dmasel12(&mut self) -> DMASEL12_W {
        DMASEL12_W { w: self }
    }
    #[doc = "Bit 13 - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
    #[inline(always)]
    pub fn dmasel13(&mut self) -> DMASEL13_W {
        DMASEL13_W { w: self }
    }
    #[doc = "Bit 14 - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&mut self) -> DMASEL14_W {
        DMASEL14_W { w: self }
    }
    #[doc = "Bit 15 - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&mut self) -> DMASEL15_W {
        DMASEL15_W { w: self }
    }
}
