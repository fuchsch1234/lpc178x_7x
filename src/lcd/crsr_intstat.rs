#[doc = "Reader of register CRSR_INTSTAT"]
pub type R = crate::R<u32, super::CRSR_INTSTAT>;
#[doc = "Reader of field `CRSRMIS`"]
pub type CRSRMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Cursor masked interrupt status. The cursor interrupt status is set immediately after the last data read from the cursor image for the current frame, providing that the corresponding bit in the CRSR_INTMSK register is set. The bit remains clear if the CRSR_INTMSK register is clear. This bit is cleared by writing to the CRSR_INTCLR register."]
    #[inline(always)]
    pub fn crsrmis(&self) -> CRSRMIS_R {
        CRSRMIS_R::new((self.bits & 0x01) != 0)
    }
}
