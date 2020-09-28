#[doc = "Reader of register EEWSTATE"]
pub type R = crate::R<u32, super::EEWSTATE>;
#[doc = "Writer for register EEWSTATE"]
pub type W = crate::W<u32, super::EEWSTATE>;
#[doc = "Register EEWSTATE `reset()`'s with value 0"]
impl crate::ResetValue for super::EEWSTATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE3`"]
pub type PHASE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE3`"]
pub struct PHASE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PHASE2`"]
pub type PHASE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE2`"]
pub struct PHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PHASE1`"]
pub type PHASE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE1`"]
pub struct PHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait states 3 (minus 1 encoded). The number of system clock periods required to give a minimum time of 15 ns."]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE3_R {
        PHASE3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states 2 (minus 1 encoded). The number of system clock periods required to give a minimum time of 55 ns."]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Wait states 1 (minus 1 encoded). The number of system clock periods required to give a minimum time of 35 ns."]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states 3 (minus 1 encoded). The number of system clock periods required to give a minimum time of 15 ns."]
    #[inline(always)]
    pub fn phase3(&mut self) -> PHASE3_W {
        PHASE3_W { w: self }
    }
    #[doc = "Bits 8:15 - Wait states 2 (minus 1 encoded). The number of system clock periods required to give a minimum time of 55 ns."]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W {
        PHASE2_W { w: self }
    }
    #[doc = "Bits 16:23 - Wait states 1 (minus 1 encoded). The number of system clock periods required to give a minimum time of 35 ns."]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W {
        PHASE1_W { w: self }
    }
}
