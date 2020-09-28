#[doc = "Reader of register CRSR_INTRAW"]
pub type R = crate::R<u32, super::CRSR_INTRAW>;
#[doc = "Reader of field `CRSRRIS`"]
pub type CRSRRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Cursor raw interrupt status. The cursor interrupt status is set immediately after the last data is read from the cursor image for the current frame. This bit is cleared by writing to the CrsrIC bit in the CRSR_INTCLR register."]
    #[inline(always)]
    pub fn crsrris(&self) -> CRSRRIS_R {
        CRSRRIS_R::new((self.bits & 0x01) != 0)
    }
}
