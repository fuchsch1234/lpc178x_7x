#[doc = "Reader of register TIMV"]
pub type R = crate::R<u32, super::TIMV>;
#[doc = "Writer for register TIMV"]
pub type W = crate::W<u32, super::TIMV>;
#[doc = "Register TIMV `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPP`"]
pub type LPP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LPP`"]
pub struct LPP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `VSW`"]
pub type VSW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSW`"]
pub struct VSW_W<'a> {
    w: &'a mut W,
}
impl<'a> VSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `VFP`"]
pub type VFP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VFP`"]
pub struct VFP_W<'a> {
    w: &'a mut W,
}
impl<'a> VFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VBP`"]
pub type VBP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBP`"]
pub struct VBP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Lines per panel. This is the number of active lines per screen. The LPP field specifies the total number of lines or rows on the LCD panel being controlled. LPP is a 10-bit value allowing between 1 and 1024 lines. Program the register with the number of lines per LCD panel, minus 1. For dual panel displays, program the register with the number of lines on each of the upper and lower panels."]
    #[inline(always)]
    pub fn lpp(&self) -> LPP_R {
        LPP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width. This is the number of horizontal synchronization lines. The 6-bit VSW field specifies the pulse width of the vertical synchronization pulse. Program the register with the number of lines required, minus one. The number of horizontal synchronization lines must be small (for example, program to zero) for passive STN LCDs. The higher the value the worse the contrast on STN LCDs."]
    #[inline(always)]
    pub fn vsw(&self) -> VSW_R {
        VSW_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Vertical front porch. This is the number of inactive lines at the end of a frame, before the vertical synchronization period. The 8-bit VFP field specifies the number of line clocks to insert at the end of each frame. When a complete frame of pixels is transmitted to the LCD display, the value in VFP is used to count the number of line clock periods to wait. After the count has elapsed, the vertical synchronization signal, LCD_FP, is asserted in active mode, or extra line clocks are inserted as specified by the VSW bit-field in passive mode. VFP generates 0-255 line clock cycles. Program to zero on passive displays for improved contrast."]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical back porch. This is the number of inactive lines at the start of a frame, after the vertical synchronization period. The 8-bit VBP field specifies the number of line clocks inserted at the beginning of each frame. The VBP count starts immediately after the vertical synchronization signal for the previous frame has been negated for active mode, or the extra line clocks have been inserted as specified by the VSW bit field in passive mode. After this has occurred, the count value in VBP sets the number of line clock periods inserted before the next frame. VBP generates 0 to 255 extra line clock cycles. Program to zero on passive displays for improved contrast."]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lines per panel. This is the number of active lines per screen. The LPP field specifies the total number of lines or rows on the LCD panel being controlled. LPP is a 10-bit value allowing between 1 and 1024 lines. Program the register with the number of lines per LCD panel, minus 1. For dual panel displays, program the register with the number of lines on each of the upper and lower panels."]
    #[inline(always)]
    pub fn lpp(&mut self) -> LPP_W {
        LPP_W { w: self }
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width. This is the number of horizontal synchronization lines. The 6-bit VSW field specifies the pulse width of the vertical synchronization pulse. Program the register with the number of lines required, minus one. The number of horizontal synchronization lines must be small (for example, program to zero) for passive STN LCDs. The higher the value the worse the contrast on STN LCDs."]
    #[inline(always)]
    pub fn vsw(&mut self) -> VSW_W {
        VSW_W { w: self }
    }
    #[doc = "Bits 16:23 - Vertical front porch. This is the number of inactive lines at the end of a frame, before the vertical synchronization period. The 8-bit VFP field specifies the number of line clocks to insert at the end of each frame. When a complete frame of pixels is transmitted to the LCD display, the value in VFP is used to count the number of line clock periods to wait. After the count has elapsed, the vertical synchronization signal, LCD_FP, is asserted in active mode, or extra line clocks are inserted as specified by the VSW bit-field in passive mode. VFP generates 0-255 line clock cycles. Program to zero on passive displays for improved contrast."]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W {
        VFP_W { w: self }
    }
    #[doc = "Bits 24:31 - Vertical back porch. This is the number of inactive lines at the start of a frame, after the vertical synchronization period. The 8-bit VBP field specifies the number of line clocks inserted at the beginning of each frame. The VBP count starts immediately after the vertical synchronization signal for the previous frame has been negated for active mode, or the extra line clocks have been inserted as specified by the VSW bit field in passive mode. After this has occurred, the count value in VBP sets the number of line clock periods inserted before the next frame. VBP generates 0 to 255 extra line clock cycles. Program to zero on passive displays for improved contrast."]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W {
        VBP_W { w: self }
    }
}
