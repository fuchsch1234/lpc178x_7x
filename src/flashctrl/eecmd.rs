#[doc = "Reader of register EECMD"]
pub type R = crate::R<u32, super::EECMD>;
#[doc = "Writer for register EECMD"]
pub type W = crate::W<u32, super::EECMD>;
#[doc = "Register EECMD `reset()`'s with value 0"]
impl crate::ResetValue for super::EECMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RDPREFETCH`"]
pub type RDPREFETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDPREFETCH`"]
pub struct RDPREFETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RDPREFETCH_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Command. 000: 8-bit read 001: 16-bit read 010: 32-bit read 011: 8-bit write 100: 16-bit write 101: 32-bit write 110: erase/program page 111: reserved"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Read data prefetch bit. 0: do not prefetch next read data as result of reading from the read data register. 1: prefetch read data as result of reading from the read data register. When this bit is set multiple consecutive data elements can be read without the need of programming new address values in the address register. The address post-increment and the automatic read data prefetch (if enabled) allow only reading from the read data register to be done to read the data."]
    #[inline(always)]
    pub fn rdprefetch(&self) -> RDPREFETCH_R {
        RDPREFETCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command. 000: 8-bit read 001: 16-bit read 010: 32-bit read 011: 8-bit write 100: 16-bit write 101: 32-bit write 110: erase/program page 111: reserved"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bit 3 - Read data prefetch bit. 0: do not prefetch next read data as result of reading from the read data register. 1: prefetch read data as result of reading from the read data register. When this bit is set multiple consecutive data elements can be read without the need of programming new address values in the address register. The address post-increment and the automatic read data prefetch (if enabled) allow only reading from the read data register to be done to read the data."]
    #[inline(always)]
    pub fn rdprefetch(&mut self) -> RDPREFETCH_W {
        RDPREFETCH_W { w: self }
    }
}
