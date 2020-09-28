#[doc = "Reader of register RESPONSE%s"]
pub type R = crate::R<u32, super::RESPONSE>;
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Card status"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
