#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `CMDCRCFAIL`"]
pub type CMDCRCFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATACRCFAIL`"]
pub type DATACRCFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDTIMEOUT`"]
pub type CMDTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATATIMEOUT`"]
pub type DATATIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUNDERRUN`"]
pub type TXUNDERRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOVERRUN`"]
pub type RXOVERRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDRESPEND`"]
pub type CMDRESPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDSENT`"]
pub type CMDSENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATAEND`"]
pub type DATAEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTBITERR`"]
pub type STARTBITERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATABLOCKEND`"]
pub type DATABLOCKEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDACTIVE`"]
pub type CMDACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXACTIVE`"]
pub type TXACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXACTIVE`"]
pub type RXACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFOHALFEMPTY`"]
pub type TXFIFOHALFEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFOHALFFULL`"]
pub type RXFIFOHALFFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFOFULL`"]
pub type TXFIFOFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFOFULL`"]
pub type RXFIFOFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFOEMPTY`"]
pub type TXFIFOEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFOEMPTY`"]
pub type RXFIFOEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDATAAVLBL`"]
pub type TXDATAAVLBL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAAVLBL`"]
pub type RXDATAAVLBL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Command response received (CRC check failed)."]
    #[inline(always)]
    pub fn cmdcrcfail(&self) -> CMDCRCFAIL_R {
        CMDCRCFAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)."]
    #[inline(always)]
    pub fn datacrcfail(&self) -> DATACRCFAIL_R {
        DATACRCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command response timeout."]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CMDTIMEOUT_R {
        CMDTIMEOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout."]
    #[inline(always)]
    pub fn datatimeout(&self) -> DATATIMEOUT_R {
        DATATIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrun(&self) -> TXUNDERRUN_R {
        TXUNDERRUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RXOVERRUN_R {
        RXOVERRUN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)."]
    #[inline(always)]
    pub fn cmdrespend(&self) -> CMDRESPEND_R {
        CMDRESPEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)."]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter is zero)."]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode."]
    #[inline(always)]
    pub fn startbiterr(&self) -> STARTBITERR_R {
        STARTBITERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)."]
    #[inline(always)]
    pub fn datablockend(&self) -> DATABLOCKEND_R {
        DATABLOCKEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress."]
    #[inline(always)]
    pub fn cmdactive(&self) -> CMDACTIVE_R {
        CMDACTIVE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress."]
    #[inline(always)]
    pub fn txactive(&self) -> TXACTIVE_R {
        TXACTIVE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress."]
    #[inline(always)]
    pub fn rxactive(&self) -> RXACTIVE_R {
        RXACTIVE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty."]
    #[inline(always)]
    pub fn txfifohalfempty(&self) -> TXFIFOHALFEMPTY_R {
        TXFIFOHALFEMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full."]
    #[inline(always)]
    pub fn rxfifohalffull(&self) -> RXFIFOHALFFULL_R {
        RXFIFOHALFFULL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full."]
    #[inline(always)]
    pub fn txfifofull(&self) -> TXFIFOFULL_R {
        TXFIFOFULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full."]
    #[inline(always)]
    pub fn rxfifofull(&self) -> RXFIFOFULL_R {
        RXFIFOFULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txfifoempty(&self) -> TXFIFOEMPTY_R {
        TXFIFOEMPTY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty."]
    #[inline(always)]
    pub fn rxfifoempty(&self) -> RXFIFOEMPTY_R {
        RXFIFOEMPTY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO."]
    #[inline(always)]
    pub fn txdataavlbl(&self) -> TXDATAAVLBL_R {
        TXDATAAVLBL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO."]
    #[inline(always)]
    pub fn rxdataavlbl(&self) -> RXDATAAVLBL_R {
        RXDATAAVLBL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
