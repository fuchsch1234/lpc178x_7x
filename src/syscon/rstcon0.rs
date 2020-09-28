#[doc = "Reader of register RSTCON0"]
pub type R = crate::R<u32, super::RSTCON0>;
#[doc = "Writer for register RSTCON0"]
pub type W = crate::W<u32, super::RSTCON0>;
#[doc = "Register RSTCON0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCON0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSTLCD`"]
pub type RSTLCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTLCD`"]
pub struct RSTLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTLCD_W<'a> {
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
#[doc = "Reader of field `RSTTIM0`"]
pub type RSTTIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTTIM0`"]
pub struct RSTTIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM0_W<'a> {
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
#[doc = "Reader of field `RSTTIM1`"]
pub type RSTTIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTTIM1`"]
pub struct RSTTIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM1_W<'a> {
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
#[doc = "Reader of field `RSTUART0`"]
pub type RSTUART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTUART0`"]
pub struct RSTUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART0_W<'a> {
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
#[doc = "Reader of field `RSTUART1`"]
pub type RSTUART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTUART1`"]
pub struct RSTUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART1_W<'a> {
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
#[doc = "Reader of field `RSTPWM0`"]
pub type RSTPWM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTPWM0`"]
pub struct RSTPWM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPWM0_W<'a> {
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
#[doc = "Reader of field `RSTPWM1`"]
pub type RSTPWM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTPWM1`"]
pub struct RSTPWM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPWM1_W<'a> {
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
#[doc = "Reader of field `RSTI2C0`"]
pub type RSTI2C0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTI2C0`"]
pub struct RSTI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2C0_W<'a> {
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
#[doc = "Reader of field `RSTUART4`"]
pub type RSTUART4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTUART4`"]
pub struct RSTUART4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART4_W<'a> {
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
#[doc = "Reader of field `RSTRTC`"]
pub type RSTRTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTRTC`"]
pub struct RSTRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RSTSSP1`"]
pub type RSTSSP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTSSP1`"]
pub struct RSTSSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSSP1_W<'a> {
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
#[doc = "Reader of field `RSTEMC`"]
pub type RSTEMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTEMC`"]
pub struct RSTEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTEMC_W<'a> {
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
#[doc = "Reader of field `RSTADC`"]
pub type RSTADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTADC`"]
pub struct RSTADC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTADC_W<'a> {
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
#[doc = "Reader of field `RSTCAN1`"]
pub type RSTCAN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTCAN1`"]
pub struct RSTCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCAN1_W<'a> {
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
#[doc = "Reader of field `RSTCAN2`"]
pub type RSTCAN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTCAN2`"]
pub struct RSTCAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTCAN2_W<'a> {
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
#[doc = "Reader of field `RSTGPIO`"]
pub type RSTGPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTGPIO`"]
pub struct RSTGPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTGPIO_W<'a> {
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
#[doc = "Reader of field `RSTSPIFI`"]
pub type RSTSPIFI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTSPIFI`"]
pub struct RSTSPIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSPIFI_W<'a> {
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
#[doc = "Reader of field `RSTMCPWM`"]
pub type RSTMCPWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTMCPWM`"]
pub struct RSTMCPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTMCPWM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RSTQEI`"]
pub type RSTQEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTQEI`"]
pub struct RSTQEI_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTQEI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RSTI2C1`"]
pub type RSTI2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTI2C1`"]
pub struct RSTI2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2C1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RSTSSP2`"]
pub type RSTSSP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTSSP2`"]
pub struct RSTSSP2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSSP2_W<'a> {
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
#[doc = "Reader of field `RSTSSP0`"]
pub type RSTSSP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTSSP0`"]
pub struct RSTSSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSSP0_W<'a> {
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
#[doc = "Reader of field `RSTTIM2`"]
pub type RSTTIM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTTIM2`"]
pub struct RSTTIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM2_W<'a> {
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
#[doc = "Reader of field `RSTTIM3`"]
pub type RSTTIM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTTIM3`"]
pub struct RSTTIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTIM3_W<'a> {
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
#[doc = "Reader of field `RSTUART2`"]
pub type RSTUART2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTUART2`"]
pub struct RSTUART2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RSTUART3`"]
pub type RSTUART3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTUART3`"]
pub struct RSTUART3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUART3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RSTI2C2`"]
pub type RSTI2C2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTI2C2`"]
pub struct RSTI2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2C2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RSTI2S`"]
pub type RSTI2S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTI2S`"]
pub struct RSTI2S_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTI2S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RSTSDC`"]
pub type RSTSDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTSDC`"]
pub struct RSTSDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RSTGPDMA`"]
pub type RSTGPDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTGPDMA`"]
pub struct RSTGPDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTGPDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RSTENET`"]
pub type RSTENET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTENET`"]
pub struct RSTENET_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTENET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RSTUSB`"]
pub type RSTUSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTUSB`"]
pub struct RSTUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTUSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD controller reset control bit."]
    #[inline(always)]
    pub fn rstlcd(&self) -> RSTLCD_R {
        RSTLCD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter 0 reset control bit."]
    #[inline(always)]
    pub fn rsttim0(&self) -> RSTTIM0_R {
        RSTTIM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter 1 reset control bit."]
    #[inline(always)]
    pub fn rsttim1(&self) -> RSTTIM1_R {
        RSTTIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UART0 reset control bit."]
    #[inline(always)]
    pub fn rstuart0(&self) -> RSTUART0_R {
        RSTUART0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART1 reset control bit."]
    #[inline(always)]
    pub fn rstuart1(&self) -> RSTUART1_R {
        RSTUART1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM0 reset control bit."]
    #[inline(always)]
    pub fn rstpwm0(&self) -> RSTPWM0_R {
        RSTPWM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM1 reset control bit."]
    #[inline(always)]
    pub fn rstpwm1(&self) -> RSTPWM1_R {
        RSTPWM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The I2C0 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c0(&self) -> RSTI2C0_R {
        RSTI2C0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART4 reset control bit."]
    #[inline(always)]
    pub fn rstuart4(&self) -> RSTUART4_R {
        RSTUART4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC and Event Monitor/Recorder reset control bit. RTC reset is limited, see Table 628 Real-Time Clock register map for details."]
    #[inline(always)]
    pub fn rstrtc(&self) -> RSTRTC_R {
        RSTRTC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The SSP 1 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp1(&self) -> RSTSSP1_R {
        RSTSSP1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Memory Controller reset control bit."]
    #[inline(always)]
    pub fn rstemc(&self) -> RSTEMC_R {
        RSTEMC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - A/D converter (ADC) reset control bit."]
    #[inline(always)]
    pub fn rstadc(&self) -> RSTADC_R {
        RSTADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CAN Controller 1 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan1(&self) -> RSTCAN1_R {
        RSTCAN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CAN Controller 2 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan2(&self) -> RSTCAN2_R {
        RSTCAN2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reset control bit for GPIO, and GPIO interrupts. Note: IOCON may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstgpio(&self) -> RSTGPIO_R {
        RSTGPIO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Flash Interface reset control bit (LPC1773 only)."]
    #[inline(always)]
    pub fn rstspifi(&self) -> RSTSPIFI_R {
        RSTSPIFI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Motor Control PWM reset control bit."]
    #[inline(always)]
    pub fn rstmcpwm(&self) -> RSTMCPWM_R {
        RSTMCPWM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface reset control bit."]
    #[inline(always)]
    pub fn rstqei(&self) -> RSTQEI_R {
        RSTQEI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The I2C1 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c1(&self) -> RSTI2C1_R {
        RSTI2C1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The SSP2 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp2(&self) -> RSTSSP2_R {
        RSTSSP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The SSP0 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp0(&self) -> RSTSSP0_R {
        RSTSSP0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 2 reset control bit."]
    #[inline(always)]
    pub fn rsttim2(&self) -> RSTTIM2_R {
        RSTTIM2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timer 3 reset control bit."]
    #[inline(always)]
    pub fn rsttim3(&self) -> RSTTIM3_R {
        RSTTIM3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UART 2 reset control bit."]
    #[inline(always)]
    pub fn rstuart2(&self) -> RSTUART2_R {
        RSTUART2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UART 3 reset control bit."]
    #[inline(always)]
    pub fn rstuart3(&self) -> RSTUART3_R {
        RSTUART3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - I2C interface 2 reset control bit."]
    #[inline(always)]
    pub fn rsti2c2(&self) -> RSTI2C2_R {
        RSTI2C2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - I2S interface reset control bit."]
    #[inline(always)]
    pub fn rsti2s(&self) -> RSTI2S_R {
        RSTI2S_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SD Card interface reset control bit."]
    #[inline(always)]
    pub fn rstsdc(&self) -> RSTSDC_R {
        RSTSDC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPDMA function reset control bit."]
    #[inline(always)]
    pub fn rstgpdma(&self) -> RSTGPDMA_R {
        RSTGPDMA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ethernet block reset control bit."]
    #[inline(always)]
    pub fn rstenet(&self) -> RSTENET_R {
        RSTENET_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - USB interface reset control bit."]
    #[inline(always)]
    pub fn rstusb(&self) -> RSTUSB_R {
        RSTUSB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD controller reset control bit."]
    #[inline(always)]
    pub fn rstlcd(&mut self) -> RSTLCD_W {
        RSTLCD_W { w: self }
    }
    #[doc = "Bit 1 - Timer/Counter 0 reset control bit."]
    #[inline(always)]
    pub fn rsttim0(&mut self) -> RSTTIM0_W {
        RSTTIM0_W { w: self }
    }
    #[doc = "Bit 2 - Timer/Counter 1 reset control bit."]
    #[inline(always)]
    pub fn rsttim1(&mut self) -> RSTTIM1_W {
        RSTTIM1_W { w: self }
    }
    #[doc = "Bit 3 - UART0 reset control bit."]
    #[inline(always)]
    pub fn rstuart0(&mut self) -> RSTUART0_W {
        RSTUART0_W { w: self }
    }
    #[doc = "Bit 4 - UART1 reset control bit."]
    #[inline(always)]
    pub fn rstuart1(&mut self) -> RSTUART1_W {
        RSTUART1_W { w: self }
    }
    #[doc = "Bit 5 - PWM0 reset control bit."]
    #[inline(always)]
    pub fn rstpwm0(&mut self) -> RSTPWM0_W {
        RSTPWM0_W { w: self }
    }
    #[doc = "Bit 6 - PWM1 reset control bit."]
    #[inline(always)]
    pub fn rstpwm1(&mut self) -> RSTPWM1_W {
        RSTPWM1_W { w: self }
    }
    #[doc = "Bit 7 - The I2C0 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c0(&mut self) -> RSTI2C0_W {
        RSTI2C0_W { w: self }
    }
    #[doc = "Bit 8 - UART4 reset control bit."]
    #[inline(always)]
    pub fn rstuart4(&mut self) -> RSTUART4_W {
        RSTUART4_W { w: self }
    }
    #[doc = "Bit 9 - RTC and Event Monitor/Recorder reset control bit. RTC reset is limited, see Table 628 Real-Time Clock register map for details."]
    #[inline(always)]
    pub fn rstrtc(&mut self) -> RSTRTC_W {
        RSTRTC_W { w: self }
    }
    #[doc = "Bit 10 - The SSP 1 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp1(&mut self) -> RSTSSP1_W {
        RSTSSP1_W { w: self }
    }
    #[doc = "Bit 11 - External Memory Controller reset control bit."]
    #[inline(always)]
    pub fn rstemc(&mut self) -> RSTEMC_W {
        RSTEMC_W { w: self }
    }
    #[doc = "Bit 12 - A/D converter (ADC) reset control bit."]
    #[inline(always)]
    pub fn rstadc(&mut self) -> RSTADC_W {
        RSTADC_W { w: self }
    }
    #[doc = "Bit 13 - CAN Controller 1 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan1(&mut self) -> RSTCAN1_W {
        RSTCAN1_W { w: self }
    }
    #[doc = "Bit 14 - CAN Controller 2 reset control bit. Note: The CAN acceptance filter may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstcan2(&mut self) -> RSTCAN2_W {
        RSTCAN2_W { w: self }
    }
    #[doc = "Bit 15 - Reset control bit for GPIO, and GPIO interrupts. Note: IOCON may be reset by a separate bit in the RSTCON1 register."]
    #[inline(always)]
    pub fn rstgpio(&mut self) -> RSTGPIO_W {
        RSTGPIO_W { w: self }
    }
    #[doc = "Bit 16 - SPI Flash Interface reset control bit (LPC1773 only)."]
    #[inline(always)]
    pub fn rstspifi(&mut self) -> RSTSPIFI_W {
        RSTSPIFI_W { w: self }
    }
    #[doc = "Bit 17 - Motor Control PWM reset control bit."]
    #[inline(always)]
    pub fn rstmcpwm(&mut self) -> RSTMCPWM_W {
        RSTMCPWM_W { w: self }
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface reset control bit."]
    #[inline(always)]
    pub fn rstqei(&mut self) -> RSTQEI_W {
        RSTQEI_W { w: self }
    }
    #[doc = "Bit 19 - The I2C1 interface reset control bit."]
    #[inline(always)]
    pub fn rsti2c1(&mut self) -> RSTI2C1_W {
        RSTI2C1_W { w: self }
    }
    #[doc = "Bit 20 - The SSP2 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp2(&mut self) -> RSTSSP2_W {
        RSTSSP2_W { w: self }
    }
    #[doc = "Bit 21 - The SSP0 interface reset control bit."]
    #[inline(always)]
    pub fn rstssp0(&mut self) -> RSTSSP0_W {
        RSTSSP0_W { w: self }
    }
    #[doc = "Bit 22 - Timer 2 reset control bit."]
    #[inline(always)]
    pub fn rsttim2(&mut self) -> RSTTIM2_W {
        RSTTIM2_W { w: self }
    }
    #[doc = "Bit 23 - Timer 3 reset control bit."]
    #[inline(always)]
    pub fn rsttim3(&mut self) -> RSTTIM3_W {
        RSTTIM3_W { w: self }
    }
    #[doc = "Bit 24 - UART 2 reset control bit."]
    #[inline(always)]
    pub fn rstuart2(&mut self) -> RSTUART2_W {
        RSTUART2_W { w: self }
    }
    #[doc = "Bit 25 - UART 3 reset control bit."]
    #[inline(always)]
    pub fn rstuart3(&mut self) -> RSTUART3_W {
        RSTUART3_W { w: self }
    }
    #[doc = "Bit 26 - I2C interface 2 reset control bit."]
    #[inline(always)]
    pub fn rsti2c2(&mut self) -> RSTI2C2_W {
        RSTI2C2_W { w: self }
    }
    #[doc = "Bit 27 - I2S interface reset control bit."]
    #[inline(always)]
    pub fn rsti2s(&mut self) -> RSTI2S_W {
        RSTI2S_W { w: self }
    }
    #[doc = "Bit 28 - SD Card interface reset control bit."]
    #[inline(always)]
    pub fn rstsdc(&mut self) -> RSTSDC_W {
        RSTSDC_W { w: self }
    }
    #[doc = "Bit 29 - GPDMA function reset control bit."]
    #[inline(always)]
    pub fn rstgpdma(&mut self) -> RSTGPDMA_W {
        RSTGPDMA_W { w: self }
    }
    #[doc = "Bit 30 - Ethernet block reset control bit."]
    #[inline(always)]
    pub fn rstenet(&mut self) -> RSTENET_W {
        RSTENET_W { w: self }
    }
    #[doc = "Bit 31 - USB interface reset control bit."]
    #[inline(always)]
    pub fn rstusb(&mut self) -> RSTUSB_W {
        RSTUSB_W { w: self }
    }
}
