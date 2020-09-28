#[doc = "Reader of register STATICWAITPAGE%s"]
pub type R = crate::R<u32, super::STATICWAITPAGE>;
#[doc = "Writer for register STATICWAITPAGE%s"]
pub type W = crate::W<u32, super::STATICWAITPAGE>;
#[doc = "Register STATICWAITPAGE%s `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::STATICWAITPAGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `WAITPAGE`"]
pub type WAITPAGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITPAGE`"]
pub struct WAITPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITPAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Asynchronous page mode read after the first read wait states. Number of wait states for asynchronous page mode read accesses after the first read: 0x0 - 0x1E = (n+ 1) CCLK cycle read access time. For asynchronous page mode read for sequential reads, the wait state time for page mode accesses after the first read is (WAITPAGE + 1) x tCCLK. 0x1F = 32 CCLK cycle read access time (POR reset value)."]
    #[inline(always)]
    pub fn waitpage(&self) -> WAITPAGE_R {
        WAITPAGE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Asynchronous page mode read after the first read wait states. Number of wait states for asynchronous page mode read accesses after the first read: 0x0 - 0x1E = (n+ 1) CCLK cycle read access time. For asynchronous page mode read for sequential reads, the wait state time for page mode accesses after the first read is (WAITPAGE + 1) x tCCLK. 0x1F = 32 CCLK cycle read access time (POR reset value)."]
    #[inline(always)]
    pub fn waitpage(&mut self) -> WAITPAGE_W {
        WAITPAGE_W { w: self }
    }
}
