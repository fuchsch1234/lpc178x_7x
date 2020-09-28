#[doc = "Reader of register ENF2"]
pub type R = crate::R<u32, super::ENF2>;
#[doc = "Writer for register ENF2"]
pub type W = crate::W<u32, super::ENF2>;
#[doc = "Register ENF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ENF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P2_0EF`"]
pub type P2_0EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_0EF`"]
pub struct P2_0EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_0EF_W<'a> {
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
#[doc = "Reader of field `P2_1EF`"]
pub type P2_1EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_1EF`"]
pub struct P2_1EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_1EF_W<'a> {
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
#[doc = "Reader of field `P2_2EF`"]
pub type P2_2EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_2EF`"]
pub struct P2_2EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_2EF_W<'a> {
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
#[doc = "Reader of field `P2_3EF`"]
pub type P2_3EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_3EF`"]
pub struct P2_3EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_3EF_W<'a> {
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
#[doc = "Reader of field `P2_4EF`"]
pub type P2_4EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_4EF`"]
pub struct P2_4EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_4EF_W<'a> {
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
#[doc = "Reader of field `P2_5EF`"]
pub type P2_5EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_5EF`"]
pub struct P2_5EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_5EF_W<'a> {
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
#[doc = "Reader of field `P2_6EF`"]
pub type P2_6EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_6EF`"]
pub struct P2_6EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_6EF_W<'a> {
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
#[doc = "Reader of field `P2_7EF`"]
pub type P2_7EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_7EF`"]
pub struct P2_7EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_7EF_W<'a> {
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
#[doc = "Reader of field `P2_8EF`"]
pub type P2_8EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_8EF`"]
pub struct P2_8EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_8EF_W<'a> {
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
#[doc = "Reader of field `P2_9EF`"]
pub type P2_9EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_9EF`"]
pub struct P2_9EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_9EF_W<'a> {
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
#[doc = "Reader of field `P2_10EF`"]
pub type P2_10EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_10EF`"]
pub struct P2_10EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10EF_W<'a> {
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
#[doc = "Reader of field `P2_11EF`"]
pub type P2_11EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_11EF`"]
pub struct P2_11EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11EF_W<'a> {
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
#[doc = "Reader of field `P2_12EF`"]
pub type P2_12EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_12EF`"]
pub struct P2_12EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12EF_W<'a> {
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
#[doc = "Reader of field `P2_13EF`"]
pub type P2_13EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_13EF`"]
pub struct P2_13EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13EF_W<'a> {
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
#[doc = "Reader of field `P2_14EF`"]
pub type P2_14EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_14EF`"]
pub struct P2_14EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_14EF_W<'a> {
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
#[doc = "Reader of field `P2_15EF`"]
pub type P2_15EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_15EF`"]
pub struct P2_15EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_15EF_W<'a> {
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
#[doc = "Reader of field `P2_16EF`"]
pub type P2_16EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_16EF`"]
pub struct P2_16EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_16EF_W<'a> {
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
#[doc = "Reader of field `P2_17EF`"]
pub type P2_17EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_17EF`"]
pub struct P2_17EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_17EF_W<'a> {
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
#[doc = "Reader of field `P2_18EF`"]
pub type P2_18EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_18EF`"]
pub struct P2_18EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_18EF_W<'a> {
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
#[doc = "Reader of field `P2_19EF`"]
pub type P2_19EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_19EF`"]
pub struct P2_19EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_19EF_W<'a> {
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
#[doc = "Reader of field `P2_20EF`"]
pub type P2_20EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_20EF`"]
pub struct P2_20EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_20EF_W<'a> {
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
#[doc = "Reader of field `P2_21EF`"]
pub type P2_21EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_21EF`"]
pub struct P2_21EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_21EF_W<'a> {
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
#[doc = "Reader of field `P2_22EF`"]
pub type P2_22EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_22EF`"]
pub struct P2_22EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_22EF_W<'a> {
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
#[doc = "Reader of field `P2_23EF`"]
pub type P2_23EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_23EF`"]
pub struct P2_23EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_23EF_W<'a> {
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
#[doc = "Reader of field `P2_24EF`"]
pub type P2_24EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_24EF`"]
pub struct P2_24EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_24EF_W<'a> {
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
#[doc = "Reader of field `P2_25EF`"]
pub type P2_25EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_25EF`"]
pub struct P2_25EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_25EF_W<'a> {
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
#[doc = "Reader of field `P2_26EF`"]
pub type P2_26EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_26EF`"]
pub struct P2_26EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_26EF_W<'a> {
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
#[doc = "Reader of field `P2_27EF`"]
pub type P2_27EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_27EF`"]
pub struct P2_27EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_27EF_W<'a> {
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
#[doc = "Reader of field `P2_28EF`"]
pub type P2_28EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_28EF`"]
pub struct P2_28EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_28EF_W<'a> {
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
#[doc = "Reader of field `P2_29EF`"]
pub type P2_29EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_29EF`"]
pub struct P2_29EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_29EF_W<'a> {
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
#[doc = "Reader of field `P2_30EF`"]
pub type P2_30EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_30EF`"]
pub struct P2_30EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_30EF_W<'a> {
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
#[doc = "Reader of field `P2_31EF`"]
pub type P2_31EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2_31EF`"]
pub struct P2_31EF_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_31EF_W<'a> {
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
    #[doc = "Bit 0 - Enable falling edge interrupt for P2\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_0ef(&self) -> P2_0EF_R {
        P2_0EF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_1ef(&self) -> P2_1EF_R {
        P2_1EF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_2ef(&self) -> P2_2EF_R {
        P2_2EF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_3ef(&self) -> P2_3EF_R {
        P2_3EF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_4ef(&self) -> P2_4EF_R {
        P2_4EF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_5ef(&self) -> P2_5EF_R {
        P2_5EF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_6ef(&self) -> P2_6EF_R {
        P2_6EF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_7ef(&self) -> P2_7EF_R {
        P2_7EF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_8ef(&self) -> P2_8EF_R {
        P2_8EF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_9ef(&self) -> P2_9EF_R {
        P2_9EF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_10ef(&self) -> P2_10EF_R {
        P2_10EF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_11ef(&self) -> P2_11EF_R {
        P2_11EF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_12ef(&self) -> P2_12EF_R {
        P2_12EF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_13ef(&self) -> P2_13EF_R {
        P2_13EF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable falling edge interrupt for P2\\[14\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_14ef(&self) -> P2_14EF_R {
        P2_14EF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable falling edge interrupt for P2\\[15\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_15ef(&self) -> P2_15EF_R {
        P2_15EF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable falling edge interrupt for P2\\[16\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_16ef(&self) -> P2_16EF_R {
        P2_16EF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable falling edge interrupt for P2\\[17\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_17ef(&self) -> P2_17EF_R {
        P2_17EF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable falling edge interrupt for P2\\[18\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_18ef(&self) -> P2_18EF_R {
        P2_18EF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable falling edge interrupt for P2\\[19\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_19ef(&self) -> P2_19EF_R {
        P2_19EF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable falling edge interrupt for P2\\[20\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_20ef(&self) -> P2_20EF_R {
        P2_20EF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable falling edge interrupt for P2\\[21\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_21ef(&self) -> P2_21EF_R {
        P2_21EF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable falling edge interrupt for P2\\[22\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_22ef(&self) -> P2_22EF_R {
        P2_22EF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable falling edge interrupt for P2\\[23\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_23ef(&self) -> P2_23EF_R {
        P2_23EF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable falling edge interrupt for P2\\[24\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_24ef(&self) -> P2_24EF_R {
        P2_24EF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable falling edge interrupt for P2\\[25\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_25ef(&self) -> P2_25EF_R {
        P2_25EF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable falling edge interrupt for P2\\[26\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_26ef(&self) -> P2_26EF_R {
        P2_26EF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable falling edge interrupt for P2\\[27\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_27ef(&self) -> P2_27EF_R {
        P2_27EF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable falling edge interrupt for P2\\[28\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_28ef(&self) -> P2_28EF_R {
        P2_28EF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable falling edge interrupt for P2\\[29\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_29ef(&self) -> P2_29EF_R {
        P2_29EF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable falling edge interrupt for P2\\[30\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_30ef(&self) -> P2_30EF_R {
        P2_30EF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable falling edge interrupt for P2\\[31\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_31ef(&self) -> P2_31EF_R {
        P2_31EF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable falling edge interrupt for P2\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_0ef(&mut self) -> P2_0EF_W {
        P2_0EF_W { w: self }
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_1ef(&mut self) -> P2_1EF_W {
        P2_1EF_W { w: self }
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_2ef(&mut self) -> P2_2EF_W {
        P2_2EF_W { w: self }
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_3ef(&mut self) -> P2_3EF_W {
        P2_3EF_W { w: self }
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_4ef(&mut self) -> P2_4EF_W {
        P2_4EF_W { w: self }
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_5ef(&mut self) -> P2_5EF_W {
        P2_5EF_W { w: self }
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_6ef(&mut self) -> P2_6EF_W {
        P2_6EF_W { w: self }
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_7ef(&mut self) -> P2_7EF_W {
        P2_7EF_W { w: self }
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_8ef(&mut self) -> P2_8EF_W {
        P2_8EF_W { w: self }
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_9ef(&mut self) -> P2_9EF_W {
        P2_9EF_W { w: self }
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_10ef(&mut self) -> P2_10EF_W {
        P2_10EF_W { w: self }
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_11ef(&mut self) -> P2_11EF_W {
        P2_11EF_W { w: self }
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_12ef(&mut self) -> P2_12EF_W {
        P2_12EF_W { w: self }
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_13ef(&mut self) -> P2_13EF_W {
        P2_13EF_W { w: self }
    }
    #[doc = "Bit 14 - Enable falling edge interrupt for P2\\[14\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_14ef(&mut self) -> P2_14EF_W {
        P2_14EF_W { w: self }
    }
    #[doc = "Bit 15 - Enable falling edge interrupt for P2\\[15\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_15ef(&mut self) -> P2_15EF_W {
        P2_15EF_W { w: self }
    }
    #[doc = "Bit 16 - Enable falling edge interrupt for P2\\[16\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_16ef(&mut self) -> P2_16EF_W {
        P2_16EF_W { w: self }
    }
    #[doc = "Bit 17 - Enable falling edge interrupt for P2\\[17\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_17ef(&mut self) -> P2_17EF_W {
        P2_17EF_W { w: self }
    }
    #[doc = "Bit 18 - Enable falling edge interrupt for P2\\[18\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_18ef(&mut self) -> P2_18EF_W {
        P2_18EF_W { w: self }
    }
    #[doc = "Bit 19 - Enable falling edge interrupt for P2\\[19\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_19ef(&mut self) -> P2_19EF_W {
        P2_19EF_W { w: self }
    }
    #[doc = "Bit 20 - Enable falling edge interrupt for P2\\[20\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_20ef(&mut self) -> P2_20EF_W {
        P2_20EF_W { w: self }
    }
    #[doc = "Bit 21 - Enable falling edge interrupt for P2\\[21\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_21ef(&mut self) -> P2_21EF_W {
        P2_21EF_W { w: self }
    }
    #[doc = "Bit 22 - Enable falling edge interrupt for P2\\[22\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_22ef(&mut self) -> P2_22EF_W {
        P2_22EF_W { w: self }
    }
    #[doc = "Bit 23 - Enable falling edge interrupt for P2\\[23\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_23ef(&mut self) -> P2_23EF_W {
        P2_23EF_W { w: self }
    }
    #[doc = "Bit 24 - Enable falling edge interrupt for P2\\[24\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_24ef(&mut self) -> P2_24EF_W {
        P2_24EF_W { w: self }
    }
    #[doc = "Bit 25 - Enable falling edge interrupt for P2\\[25\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_25ef(&mut self) -> P2_25EF_W {
        P2_25EF_W { w: self }
    }
    #[doc = "Bit 26 - Enable falling edge interrupt for P2\\[26\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_26ef(&mut self) -> P2_26EF_W {
        P2_26EF_W { w: self }
    }
    #[doc = "Bit 27 - Enable falling edge interrupt for P2\\[27\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_27ef(&mut self) -> P2_27EF_W {
        P2_27EF_W { w: self }
    }
    #[doc = "Bit 28 - Enable falling edge interrupt for P2\\[28\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_28ef(&mut self) -> P2_28EF_W {
        P2_28EF_W { w: self }
    }
    #[doc = "Bit 29 - Enable falling edge interrupt for P2\\[29\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_29ef(&mut self) -> P2_29EF_W {
        P2_29EF_W { w: self }
    }
    #[doc = "Bit 30 - Enable falling edge interrupt for P2\\[30\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_30ef(&mut self) -> P2_30EF_W {
        P2_30EF_W { w: self }
    }
    #[doc = "Bit 31 - Enable falling edge interrupt for P2\\[31\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline(always)]
    pub fn p2_31ef(&mut self) -> P2_31EF_W {
        P2_31EF_W { w: self }
    }
}
