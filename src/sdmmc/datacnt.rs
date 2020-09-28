#[doc = "Reader of register DATACNT"]
pub type R = crate::R<u32, super::DATACNT>;
#[doc = "Reader of field `DATACOUNT`"]
pub type DATACOUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Remaining data"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
