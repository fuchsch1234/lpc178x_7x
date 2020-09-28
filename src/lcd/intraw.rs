#[doc = "Reader of register INTRAW"]
pub type R = crate::R<u32, super::INTRAW>;
#[doc = "Reader of field `FUFRIS`"]
pub type FUFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LNBURIS`"]
pub type LNBURIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VCOMPRIS`"]
pub type VCOMPRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BERRAW`"]
pub type BERRAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - FIFO underflow raw interrupt status. Set when either the upper or lower DMA FIFOs have been read accessed when empty causing an underflow condition to occur. Generates an interrupt if the FUFIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn fufris(&self) -> FUFRIS_R {
        FUFRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update raw interrupt status. Mode dependent. Set when the current base address registers have been successfully updated by the next address registers. Signifies that a new next address can be loaded if double buffering is in use. Generates an interrupt if the LNBUIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn lnburis(&self) -> LNBURIS_R {
        LNBURIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare raw interrupt status. Set when one of the four vertical regions is reached, as selected by the LcdVComp bits in the LCD_CTRL register. Generates an interrupt if the VCompIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn vcompris(&self) -> VCOMPRIS_R {
        VCOMPRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error raw interrupt status. Set when the AHB master interface receives a bus error response from a slave. Generates an interrupt if the BERIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn berraw(&self) -> BERRAW_R {
        BERRAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
