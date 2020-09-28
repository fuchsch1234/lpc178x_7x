#[doc = "Reader of register DYNAMICRASCAS%s"]
pub type R = crate::R<u32, super::DYNAMICRASCAS>;
#[doc = "Writer for register DYNAMICRASCAS%s"]
pub type W = crate::W<u32, super::DYNAMICRASCAS>;
#[doc = "Register DYNAMICRASCAS%s `reset()`'s with value 0x0303"]
impl crate::ResetValue for super::DYNAMICRASCAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0303
    }
}
#[doc = "RAS latency (active to read/write delay).\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAS_A {
    #[doc = "1: One CCLK cycle."]
    ONE_CCLK_CYCLE_ = 1,
    #[doc = "2: Two CCLK cycles."]
    TWO_CCLK_CYCLES_ = 2,
    #[doc = "3: Three CCLK cycles (POR reset value)."]
    THREE_CCLK_CYCLES_P = 3,
}
impl From<RAS_A> for u8 {
    #[inline(always)]
    fn from(variant: RAS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAS`"]
pub type RAS_R = crate::R<u8, RAS_A>;
impl RAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RAS_A::ONE_CCLK_CYCLE_),
            2 => Val(RAS_A::TWO_CCLK_CYCLES_),
            3 => Val(RAS_A::THREE_CCLK_CYCLES_P),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_CCLK_CYCLE_`"]
    #[inline(always)]
    pub fn is_one_cclk_cycle_(&self) -> bool {
        *self == RAS_A::ONE_CCLK_CYCLE_
    }
    #[doc = "Checks if the value of the field is `TWO_CCLK_CYCLES_`"]
    #[inline(always)]
    pub fn is_two_cclk_cycles_(&self) -> bool {
        *self == RAS_A::TWO_CCLK_CYCLES_
    }
    #[doc = "Checks if the value of the field is `THREE_CCLK_CYCLES_P`"]
    #[inline(always)]
    pub fn is_three_cclk_cycles_p(&self) -> bool {
        *self == RAS_A::THREE_CCLK_CYCLES_P
    }
}
#[doc = "Write proxy for field `RAS`"]
pub struct RAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn one_cclk_cycle_(self) -> &'a mut W {
        self.variant(RAS_A::ONE_CCLK_CYCLE_)
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn two_cclk_cycles_(self) -> &'a mut W {
        self.variant(RAS_A::TWO_CCLK_CYCLES_)
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn three_cclk_cycles_p(self) -> &'a mut W {
        self.variant(RAS_A::THREE_CCLK_CYCLES_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "CAS latency.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAS_A {
    #[doc = "1: One CCLK cycle."]
    ONE_CCLK_CYCLE_ = 1,
    #[doc = "2: Two CCLK cycles."]
    TWO_CCLK_CYCLES_ = 2,
    #[doc = "3: Three CCLK cycles (POR reset value)."]
    THREE_CCLK_CYCLES_P = 3,
}
impl From<CAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CAS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAS`"]
pub type CAS_R = crate::R<u8, CAS_A>;
impl CAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CAS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CAS_A::ONE_CCLK_CYCLE_),
            2 => Val(CAS_A::TWO_CCLK_CYCLES_),
            3 => Val(CAS_A::THREE_CCLK_CYCLES_P),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_CCLK_CYCLE_`"]
    #[inline(always)]
    pub fn is_one_cclk_cycle_(&self) -> bool {
        *self == CAS_A::ONE_CCLK_CYCLE_
    }
    #[doc = "Checks if the value of the field is `TWO_CCLK_CYCLES_`"]
    #[inline(always)]
    pub fn is_two_cclk_cycles_(&self) -> bool {
        *self == CAS_A::TWO_CCLK_CYCLES_
    }
    #[doc = "Checks if the value of the field is `THREE_CCLK_CYCLES_P`"]
    #[inline(always)]
    pub fn is_three_cclk_cycles_p(&self) -> bool {
        *self == CAS_A::THREE_CCLK_CYCLES_P
    }
}
#[doc = "Write proxy for field `CAS`"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn one_cclk_cycle_(self) -> &'a mut W {
        self.variant(CAS_A::ONE_CCLK_CYCLE_)
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn two_cclk_cycles_(self) -> &'a mut W {
        self.variant(CAS_A::TWO_CCLK_CYCLES_)
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn three_cclk_cycles_p(self) -> &'a mut W {
        self.variant(CAS_A::THREE_CCLK_CYCLES_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&mut self) -> RAS_W {
        RAS_W { w: self }
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
}
