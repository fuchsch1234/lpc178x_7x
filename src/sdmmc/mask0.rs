#[doc = "Reader of register MASK0"]
pub type R = crate::R<u32, super::MASK0>;
#[doc = "Writer for register MASK0"]
pub type W = crate::W<u32, super::MASK0>;
#[doc = "Register MASK0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK0`"]
pub type MASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK0`"]
pub struct MASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK0_W<'a> {
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
#[doc = "Reader of field `MASK1`"]
pub type MASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK1`"]
pub struct MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK1_W<'a> {
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
#[doc = "Reader of field `MASK2`"]
pub type MASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK2`"]
pub struct MASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK2_W<'a> {
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
#[doc = "Reader of field `MASK3`"]
pub type MASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK3`"]
pub struct MASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK3_W<'a> {
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
#[doc = "Reader of field `MASK4`"]
pub type MASK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK4`"]
pub struct MASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK4_W<'a> {
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
#[doc = "Reader of field `MASK5`"]
pub type MASK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK5`"]
pub struct MASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK5_W<'a> {
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
#[doc = "Reader of field `MASK6`"]
pub type MASK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK6`"]
pub struct MASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK6_W<'a> {
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
#[doc = "Reader of field `MASK7`"]
pub type MASK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK7`"]
pub struct MASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK7_W<'a> {
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
#[doc = "Reader of field `MASK8`"]
pub type MASK8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK8`"]
pub struct MASK8_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK8_W<'a> {
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
#[doc = "Reader of field `MASK9`"]
pub type MASK9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK9`"]
pub struct MASK9_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK9_W<'a> {
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
#[doc = "Reader of field `MASK10`"]
pub type MASK10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK10`"]
pub struct MASK10_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK10_W<'a> {
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
#[doc = "Reader of field `MASK11`"]
pub type MASK11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK11`"]
pub struct MASK11_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK11_W<'a> {
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
#[doc = "Reader of field `MASK12`"]
pub type MASK12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK12`"]
pub struct MASK12_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK12_W<'a> {
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
#[doc = "Reader of field `MASK13`"]
pub type MASK13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK13`"]
pub struct MASK13_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK13_W<'a> {
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
#[doc = "Reader of field `MASK14`"]
pub type MASK14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK14`"]
pub struct MASK14_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK14_W<'a> {
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
#[doc = "Reader of field `MASK15`"]
pub type MASK15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK15`"]
pub struct MASK15_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK15_W<'a> {
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
#[doc = "Reader of field `MASK16`"]
pub type MASK16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK16`"]
pub struct MASK16_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK16_W<'a> {
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
#[doc = "Reader of field `MASK17`"]
pub type MASK17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK17`"]
pub struct MASK17_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK17_W<'a> {
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
#[doc = "Reader of field `MASK18`"]
pub type MASK18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK18`"]
pub struct MASK18_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK18_W<'a> {
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
#[doc = "Reader of field `MASK19`"]
pub type MASK19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK19`"]
pub struct MASK19_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK19_W<'a> {
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
#[doc = "Reader of field `MASK20`"]
pub type MASK20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK20`"]
pub struct MASK20_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK20_W<'a> {
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
#[doc = "Reader of field `MASK21`"]
pub type MASK21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK21`"]
pub struct MASK21_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK21_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Mask CmdCrcFail flag."]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask DataCrcFail flag."]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask CmdTimeOut flag."]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask DataTimeOut flag."]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask TxUnderrun flag."]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask RxOverrun flag."]
    #[inline(always)]
    pub fn mask5(&self) -> MASK5_R {
        MASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask CmdRespEnd flag."]
    #[inline(always)]
    pub fn mask6(&self) -> MASK6_R {
        MASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask CmdSent flag."]
    #[inline(always)]
    pub fn mask7(&self) -> MASK7_R {
        MASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask DataEnd flag."]
    #[inline(always)]
    pub fn mask8(&self) -> MASK8_R {
        MASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask StartBitErr flag."]
    #[inline(always)]
    pub fn mask9(&self) -> MASK9_R {
        MASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mask DataBlockEnd flag."]
    #[inline(always)]
    pub fn mask10(&self) -> MASK10_R {
        MASK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask CmdActive flag."]
    #[inline(always)]
    pub fn mask11(&self) -> MASK11_R {
        MASK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mask TxActive flag."]
    #[inline(always)]
    pub fn mask12(&self) -> MASK12_R {
        MASK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Mask RxActive flag."]
    #[inline(always)]
    pub fn mask13(&self) -> MASK13_R {
        MASK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mask TxFifoHalfEmpty flag."]
    #[inline(always)]
    pub fn mask14(&self) -> MASK14_R {
        MASK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask RxFifoHalfFull flag."]
    #[inline(always)]
    pub fn mask15(&self) -> MASK15_R {
        MASK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mask TxFifoFull flag."]
    #[inline(always)]
    pub fn mask16(&self) -> MASK16_R {
        MASK16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mask RxFifoFull flag."]
    #[inline(always)]
    pub fn mask17(&self) -> MASK17_R {
        MASK17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mask TxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask18(&self) -> MASK18_R {
        MASK18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Mask RxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask19(&self) -> MASK19_R {
        MASK19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Mask TxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask20(&self) -> MASK20_R {
        MASK20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Mask RxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask21(&self) -> MASK21_R {
        MASK21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask CmdCrcFail flag."]
    #[inline(always)]
    pub fn mask0(&mut self) -> MASK0_W {
        MASK0_W { w: self }
    }
    #[doc = "Bit 1 - Mask DataCrcFail flag."]
    #[inline(always)]
    pub fn mask1(&mut self) -> MASK1_W {
        MASK1_W { w: self }
    }
    #[doc = "Bit 2 - Mask CmdTimeOut flag."]
    #[inline(always)]
    pub fn mask2(&mut self) -> MASK2_W {
        MASK2_W { w: self }
    }
    #[doc = "Bit 3 - Mask DataTimeOut flag."]
    #[inline(always)]
    pub fn mask3(&mut self) -> MASK3_W {
        MASK3_W { w: self }
    }
    #[doc = "Bit 4 - Mask TxUnderrun flag."]
    #[inline(always)]
    pub fn mask4(&mut self) -> MASK4_W {
        MASK4_W { w: self }
    }
    #[doc = "Bit 5 - Mask RxOverrun flag."]
    #[inline(always)]
    pub fn mask5(&mut self) -> MASK5_W {
        MASK5_W { w: self }
    }
    #[doc = "Bit 6 - Mask CmdRespEnd flag."]
    #[inline(always)]
    pub fn mask6(&mut self) -> MASK6_W {
        MASK6_W { w: self }
    }
    #[doc = "Bit 7 - Mask CmdSent flag."]
    #[inline(always)]
    pub fn mask7(&mut self) -> MASK7_W {
        MASK7_W { w: self }
    }
    #[doc = "Bit 8 - Mask DataEnd flag."]
    #[inline(always)]
    pub fn mask8(&mut self) -> MASK8_W {
        MASK8_W { w: self }
    }
    #[doc = "Bit 9 - Mask StartBitErr flag."]
    #[inline(always)]
    pub fn mask9(&mut self) -> MASK9_W {
        MASK9_W { w: self }
    }
    #[doc = "Bit 10 - Mask DataBlockEnd flag."]
    #[inline(always)]
    pub fn mask10(&mut self) -> MASK10_W {
        MASK10_W { w: self }
    }
    #[doc = "Bit 11 - Mask CmdActive flag."]
    #[inline(always)]
    pub fn mask11(&mut self) -> MASK11_W {
        MASK11_W { w: self }
    }
    #[doc = "Bit 12 - Mask TxActive flag."]
    #[inline(always)]
    pub fn mask12(&mut self) -> MASK12_W {
        MASK12_W { w: self }
    }
    #[doc = "Bit 13 - Mask RxActive flag."]
    #[inline(always)]
    pub fn mask13(&mut self) -> MASK13_W {
        MASK13_W { w: self }
    }
    #[doc = "Bit 14 - Mask TxFifoHalfEmpty flag."]
    #[inline(always)]
    pub fn mask14(&mut self) -> MASK14_W {
        MASK14_W { w: self }
    }
    #[doc = "Bit 15 - Mask RxFifoHalfFull flag."]
    #[inline(always)]
    pub fn mask15(&mut self) -> MASK15_W {
        MASK15_W { w: self }
    }
    #[doc = "Bit 16 - Mask TxFifoFull flag."]
    #[inline(always)]
    pub fn mask16(&mut self) -> MASK16_W {
        MASK16_W { w: self }
    }
    #[doc = "Bit 17 - Mask RxFifoFull flag."]
    #[inline(always)]
    pub fn mask17(&mut self) -> MASK17_W {
        MASK17_W { w: self }
    }
    #[doc = "Bit 18 - Mask TxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask18(&mut self) -> MASK18_W {
        MASK18_W { w: self }
    }
    #[doc = "Bit 19 - Mask RxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask19(&mut self) -> MASK19_W {
        MASK19_W { w: self }
    }
    #[doc = "Bit 20 - Mask TxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask20(&mut self) -> MASK20_W {
        MASK20_W { w: self }
    }
    #[doc = "Bit 21 - Mask RxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask21(&mut self) -> MASK21_W {
        MASK21_W { w: self }
    }
}
