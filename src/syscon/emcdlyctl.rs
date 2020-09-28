#[doc = "Reader of register EMCDLYCTL"]
pub type R = crate::R<u32, super::EMCDLYCTL>;
#[doc = "Writer for register EMCDLYCTL"]
pub type W = crate::W<u32, super::EMCDLYCTL>;
#[doc = "Register EMCDLYCTL `reset()`'s with value 0x0210"]
impl crate::ResetValue for super::EMCDLYCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0210
    }
}
#[doc = "Reader of field `CMDDLY`"]
pub type CMDDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDDLY`"]
pub struct CMDDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `FBCLKDLY`"]
pub type FBCLKDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FBCLKDLY`"]
pub struct FBCLKDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCLKDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKOUT0DLY`"]
pub type CLKOUT0DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKOUT0DLY`"]
pub struct CLKOUT0DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT0DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLKOUT1DLY`"]
pub type CLKOUT1DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKOUT1DLY`"]
pub struct CLKOUT1DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT1DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode. See Section 10.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn cmddly(&self) -> CMDDLY_R {
        CMDDLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling. See Section 10.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn fbclkdly(&self) -> FBCLKDLY_R {
        FBCLKDLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn clkout0dly(&self) -> CLKOUT0DLY_R {
        CLKOUT0DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn clkout1dly(&self) -> CLKOUT1DLY_R {
        CLKOUT1DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode. See Section 10.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn cmddly(&mut self) -> CMDDLY_W {
        CMDDLY_W { w: self }
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling. See Section 10.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn fbclkdly(&mut self) -> FBCLKDLY_W {
        FBCLKDLY_W { w: self }
    }
    #[doc = "Bits 16:20 - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn clkout0dly(&mut self) -> CLKOUT0DLY_W {
        CLKOUT0DLY_W { w: self }
    }
    #[doc = "Bits 24:28 - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn clkout1dly(&mut self) -> CLKOUT1DLY_W {
        CLKOUT1DLY_W { w: self }
    }
}
