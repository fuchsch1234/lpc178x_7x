#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Reader of field `EE_RW_DONE`"]
pub type EE_RW_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EE_PROG_DONE`"]
pub type EE_PROG_DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 26 - EEPROM read/write operation finished interrupt enable bit. Bit is: - set when 1 is written to the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
    #[inline(always)]
    pub fn ee_rw_done(&self) -> EE_RW_DONE_R {
        EE_RW_DONE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EEPROM program operation finished interrupt enable bit. Bit is: - set when 1 is written in the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
    #[inline(always)]
    pub fn ee_prog_done(&self) -> EE_PROG_DONE_R {
        EE_PROG_DONE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
