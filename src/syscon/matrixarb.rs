#[doc = "Reader of register MATRIXARB"]
pub type R = crate::R<u32, super::MATRIXARB>;
#[doc = "Writer for register MATRIXARB"]
pub type W = crate::W<u32, super::MATRIXARB>;
#[doc = "Register MATRIXARB `reset()`'s with value 0"]
impl crate::ResetValue for super::MATRIXARB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_ICODE`"]
pub type PRI_ICODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_ICODE`"]
pub struct PRI_ICODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ICODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PRI_DCODE`"]
pub type PRI_DCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_DCODE`"]
pub struct PRI_DCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_DCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRI_SYS`"]
pub type PRI_SYS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SYS`"]
pub struct PRI_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PRI_GPDMA`"]
pub type PRI_GPDMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_GPDMA`"]
pub struct PRI_GPDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_GPDMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_ETH`"]
pub type PRI_ETH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_ETH`"]
pub struct PRI_ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ETH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_LCD`"]
pub type PRI_LCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_LCD`"]
pub struct PRI_LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_LCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PRI_USB`"]
pub type PRI_USB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_USB`"]
pub struct PRI_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ROM_LAT`"]
pub type ROM_LAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROM_LAT`"]
pub struct ROM_LAT_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_LAT_W<'a> {
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
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline(always)]
    pub fn pri_icode(&self) -> PRI_ICODE_R {
        PRI_ICODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&self) -> PRI_DCODE_R {
        PRI_DCODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&self) -> PRI_SYS_R {
        PRI_SYS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline(always)]
    pub fn pri_gpdma(&self) -> PRI_GPDMA_R {
        PRI_GPDMA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&self) -> PRI_ETH_R {
        PRI_ETH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&self) -> PRI_LCD_R {
        PRI_LCD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline(always)]
    pub fn pri_usb(&self) -> PRI_USB_R {
        PRI_USB_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline(always)]
    pub fn rom_lat(&self) -> ROM_LAT_R {
        ROM_LAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline(always)]
    pub fn pri_icode(&mut self) -> PRI_ICODE_W {
        PRI_ICODE_W { w: self }
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&mut self) -> PRI_DCODE_W {
        PRI_DCODE_W { w: self }
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&mut self) -> PRI_SYS_W {
        PRI_SYS_W { w: self }
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline(always)]
    pub fn pri_gpdma(&mut self) -> PRI_GPDMA_W {
        PRI_GPDMA_W { w: self }
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&mut self) -> PRI_ETH_W {
        PRI_ETH_W { w: self }
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&mut self) -> PRI_LCD_W {
        PRI_LCD_W { w: self }
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline(always)]
    pub fn pri_usb(&mut self) -> PRI_USB_W {
        PRI_USB_W { w: self }
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline(always)]
    pub fn rom_lat(&mut self) -> ROM_LAT_W {
        ROM_LAT_W { w: self }
    }
}
