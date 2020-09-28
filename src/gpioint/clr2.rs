#[doc = "Writer for register CLR2"]
pub type W = crate::W<u32, super::CLR2>;
#[doc = "Register CLR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `P2_0CI`"]
pub struct P2_0CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_0CI_W<'a> {
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
#[doc = "Write proxy for field `P2_1CI`"]
pub struct P2_1CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_1CI_W<'a> {
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
#[doc = "Write proxy for field `P2_2CI`"]
pub struct P2_2CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_2CI_W<'a> {
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
#[doc = "Write proxy for field `P2_3CI`"]
pub struct P2_3CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_3CI_W<'a> {
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
#[doc = "Write proxy for field `P2_4CI`"]
pub struct P2_4CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_4CI_W<'a> {
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
#[doc = "Write proxy for field `P2_5CI`"]
pub struct P2_5CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_5CI_W<'a> {
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
#[doc = "Write proxy for field `P2_6CI`"]
pub struct P2_6CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_6CI_W<'a> {
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
#[doc = "Write proxy for field `P2_7CI`"]
pub struct P2_7CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_7CI_W<'a> {
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
#[doc = "Write proxy for field `P2_8CI`"]
pub struct P2_8CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_8CI_W<'a> {
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
#[doc = "Write proxy for field `P2_9CI`"]
pub struct P2_9CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_9CI_W<'a> {
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
#[doc = "Write proxy for field `P2_10CI`"]
pub struct P2_10CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10CI_W<'a> {
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
#[doc = "Write proxy for field `P2_11CI`"]
pub struct P2_11CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11CI_W<'a> {
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
#[doc = "Write proxy for field `P2_12CI`"]
pub struct P2_12CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12CI_W<'a> {
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
#[doc = "Write proxy for field `P2_13CI`"]
pub struct P2_13CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13CI_W<'a> {
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
#[doc = "Write proxy for field `P2_14CI`"]
pub struct P2_14CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_14CI_W<'a> {
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
#[doc = "Write proxy for field `P2_15CI`"]
pub struct P2_15CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_15CI_W<'a> {
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
#[doc = "Write proxy for field `P2_16CI`"]
pub struct P2_16CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_16CI_W<'a> {
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
#[doc = "Write proxy for field `P2_17CI`"]
pub struct P2_17CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_17CI_W<'a> {
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
#[doc = "Write proxy for field `P2_18CI`"]
pub struct P2_18CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_18CI_W<'a> {
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
#[doc = "Write proxy for field `P2_19CI`"]
pub struct P2_19CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_19CI_W<'a> {
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
#[doc = "Write proxy for field `P2_20CI`"]
pub struct P2_20CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_20CI_W<'a> {
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
#[doc = "Write proxy for field `P2_21CI`"]
pub struct P2_21CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_21CI_W<'a> {
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
#[doc = "Write proxy for field `P2_22CI`"]
pub struct P2_22CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_22CI_W<'a> {
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
#[doc = "Write proxy for field `P2_23CI`"]
pub struct P2_23CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_23CI_W<'a> {
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
#[doc = "Write proxy for field `P2_24CI`"]
pub struct P2_24CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_24CI_W<'a> {
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
#[doc = "Write proxy for field `P2_25CI`"]
pub struct P2_25CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_25CI_W<'a> {
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
#[doc = "Write proxy for field `P2_26CI`"]
pub struct P2_26CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_26CI_W<'a> {
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
#[doc = "Write proxy for field `P2_27CI`"]
pub struct P2_27CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_27CI_W<'a> {
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
#[doc = "Write proxy for field `P2_28CI`"]
pub struct P2_28CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_28CI_W<'a> {
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
#[doc = "Write proxy for field `P2_29CI`"]
pub struct P2_29CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_29CI_W<'a> {
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
#[doc = "Write proxy for field `P2_30CI`"]
pub struct P2_30CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_30CI_W<'a> {
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
#[doc = "Write proxy for field `P2_31CI`"]
pub struct P2_31CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_31CI_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clear GPIO port Interrupts for P2\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_0ci(&mut self) -> P2_0CI_W {
        P2_0CI_W { w: self }
    }
    #[doc = "Bit 1 - Clear GPIO port Interrupts for P2\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_1ci(&mut self) -> P2_1CI_W {
        P2_1CI_W { w: self }
    }
    #[doc = "Bit 2 - Clear GPIO port Interrupts for P2\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_2ci(&mut self) -> P2_2CI_W {
        P2_2CI_W { w: self }
    }
    #[doc = "Bit 3 - Clear GPIO port Interrupts for P2\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_3ci(&mut self) -> P2_3CI_W {
        P2_3CI_W { w: self }
    }
    #[doc = "Bit 4 - Clear GPIO port Interrupts for P2\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_4ci(&mut self) -> P2_4CI_W {
        P2_4CI_W { w: self }
    }
    #[doc = "Bit 5 - Clear GPIO port Interrupts for P2\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_5ci(&mut self) -> P2_5CI_W {
        P2_5CI_W { w: self }
    }
    #[doc = "Bit 6 - Clear GPIO port Interrupts for P2\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_6ci(&mut self) -> P2_6CI_W {
        P2_6CI_W { w: self }
    }
    #[doc = "Bit 7 - Clear GPIO port Interrupts for P2\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_7ci(&mut self) -> P2_7CI_W {
        P2_7CI_W { w: self }
    }
    #[doc = "Bit 8 - Clear GPIO port Interrupts for P2\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_8ci(&mut self) -> P2_8CI_W {
        P2_8CI_W { w: self }
    }
    #[doc = "Bit 9 - Clear GPIO port Interrupts for P2\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_9ci(&mut self) -> P2_9CI_W {
        P2_9CI_W { w: self }
    }
    #[doc = "Bit 10 - Clear GPIO port Interrupts for P2\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_10ci(&mut self) -> P2_10CI_W {
        P2_10CI_W { w: self }
    }
    #[doc = "Bit 11 - Clear GPIO port Interrupts for P2\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_11ci(&mut self) -> P2_11CI_W {
        P2_11CI_W { w: self }
    }
    #[doc = "Bit 12 - Clear GPIO port Interrupts for P2\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_12ci(&mut self) -> P2_12CI_W {
        P2_12CI_W { w: self }
    }
    #[doc = "Bit 13 - Clear GPIO port Interrupts for P2\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_13ci(&mut self) -> P2_13CI_W {
        P2_13CI_W { w: self }
    }
    #[doc = "Bit 14 - Clear GPIO port Interrupts for P2\\[14\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_14ci(&mut self) -> P2_14CI_W {
        P2_14CI_W { w: self }
    }
    #[doc = "Bit 15 - Clear GPIO port Interrupts for P2\\[15\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_15ci(&mut self) -> P2_15CI_W {
        P2_15CI_W { w: self }
    }
    #[doc = "Bit 16 - Clear GPIO port Interrupts for P2\\[16\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_16ci(&mut self) -> P2_16CI_W {
        P2_16CI_W { w: self }
    }
    #[doc = "Bit 17 - Clear GPIO port Interrupts for P2\\[17\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_17ci(&mut self) -> P2_17CI_W {
        P2_17CI_W { w: self }
    }
    #[doc = "Bit 18 - Clear GPIO port Interrupts for P2\\[18\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_18ci(&mut self) -> P2_18CI_W {
        P2_18CI_W { w: self }
    }
    #[doc = "Bit 19 - Clear GPIO port Interrupts for P2\\[19\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_19ci(&mut self) -> P2_19CI_W {
        P2_19CI_W { w: self }
    }
    #[doc = "Bit 20 - Clear GPIO port Interrupts for P2\\[20\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_20ci(&mut self) -> P2_20CI_W {
        P2_20CI_W { w: self }
    }
    #[doc = "Bit 21 - Clear GPIO port Interrupts for P2\\[21\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_21ci(&mut self) -> P2_21CI_W {
        P2_21CI_W { w: self }
    }
    #[doc = "Bit 22 - Clear GPIO port Interrupts for P2\\[22\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_22ci(&mut self) -> P2_22CI_W {
        P2_22CI_W { w: self }
    }
    #[doc = "Bit 23 - Clear GPIO port Interrupts for P2\\[23\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_23ci(&mut self) -> P2_23CI_W {
        P2_23CI_W { w: self }
    }
    #[doc = "Bit 24 - Clear GPIO port Interrupts for P2\\[24\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_24ci(&mut self) -> P2_24CI_W {
        P2_24CI_W { w: self }
    }
    #[doc = "Bit 25 - Clear GPIO port Interrupts for P2\\[25\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_25ci(&mut self) -> P2_25CI_W {
        P2_25CI_W { w: self }
    }
    #[doc = "Bit 26 - Clear GPIO port Interrupts for P2\\[26\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_26ci(&mut self) -> P2_26CI_W {
        P2_26CI_W { w: self }
    }
    #[doc = "Bit 27 - Clear GPIO port Interrupts for P2\\[27\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_27ci(&mut self) -> P2_27CI_W {
        P2_27CI_W { w: self }
    }
    #[doc = "Bit 28 - Clear GPIO port Interrupts for P2\\[28\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_28ci(&mut self) -> P2_28CI_W {
        P2_28CI_W { w: self }
    }
    #[doc = "Bit 29 - Clear GPIO port Interrupts for P2\\[29\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_29ci(&mut self) -> P2_29CI_W {
        P2_29CI_W { w: self }
    }
    #[doc = "Bit 30 - Clear GPIO port Interrupts for P2\\[30\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_30ci(&mut self) -> P2_30CI_W {
        P2_30CI_W { w: self }
    }
    #[doc = "Bit 31 - Clear GPIO port Interrupts for P2\\[31\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_31ci(&mut self) -> P2_31CI_W {
        P2_31CI_W { w: self }
    }
}
