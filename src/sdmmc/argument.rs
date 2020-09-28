#[doc = "Reader of register ARGUMENT"]
pub type R = crate::R<u32, super::ARGUMENT>;
#[doc = "Writer for register ARGUMENT"]
pub type W = crate::W<u32, super::ARGUMENT>;
#[doc = "Register ARGUMENT `reset()`'s with value 0"]
impl crate::ResetValue for super::ARGUMENT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CmdArg`"]
pub type CMDARG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CmdArg`"]
pub struct CMDARG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmd_arg(&self) -> CMDARG_R {
        CMDARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmd_arg(&mut self) -> CMDARG_W {
        CMDARG_W { w: self }
    }
}
