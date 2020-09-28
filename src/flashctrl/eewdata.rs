#[doc = "Writer for register EEWDATA"]
pub type W = crate::W<u32, super::EEWDATA>;
#[doc = "Register EEWDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::EEWDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WDATA`"]
pub struct WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write data. In case of: 8-bit write operations: bits \\[7:0\\]
must contain valid write data. 16-bit write operations: bits \\[15:0\\]
must contain valid write data. 32-bit write operations: bits \\[31:0\\]
must contain valid write data."]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W { w: self }
    }
}
