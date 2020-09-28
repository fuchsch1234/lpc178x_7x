#[doc = "Reader of register ERCOUNTERS"]
pub type R = crate::R<u32, super::ERCOUNTERS>;
#[doc = "Reader of field `COUNTER0`"]
pub type COUNTER0_R = crate::R<u8, u8>;
#[doc = "Reader of field `COUNTER1`"]
pub type COUNTER1_R = crate::R<u8, u8>;
#[doc = "Reader of field `COUNTER2`"]
pub type COUNTER2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Value of the counter for event 0. If the counter reaches full count (the value 7), it remains there if additional events occur. This counter is cleared when the corresponding EVx bit in the ERSTATUS register is cleared by software."]
    #[inline(always)]
    pub fn counter0(&self) -> COUNTER0_R {
        COUNTER0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Value of the counter for event 1. See description for COUNTER0."]
    #[inline(always)]
    pub fn counter1(&self) -> COUNTER1_R {
        COUNTER1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Value of the counter for event 2. See description for COUNTER0."]
    #[inline(always)]
    pub fn counter2(&self) -> COUNTER2_R {
        COUNTER2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
