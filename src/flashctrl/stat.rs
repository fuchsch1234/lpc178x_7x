#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `SIG_DONE`"]
pub type SIG_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `END_OF_RDWR`"]
pub type END_OF_RDWR_R = crate::R<bool, bool>;
#[doc = "Reader of field `END_OF_PROG1`"]
pub type END_OF_PROG1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
    #[inline(always)]
    pub fn sig_done(&self) -> SIG_DONE_R {
        SIG_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 26 - EEPROM read/write operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written in the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
    #[inline(always)]
    pub fn end_of_rdwr(&self) -> END_OF_RDWR_R {
        END_OF_RDWR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EEPROM program operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written to the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
    #[inline(always)]
    pub fn end_of_prog1(&self) -> END_OF_PROG1_R {
        END_OF_PROG1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
