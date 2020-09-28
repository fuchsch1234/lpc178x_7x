#[doc = "Reader of register ERLASTSTAMP%s"]
pub type R = crate::R<u32, super::ERLASTSTAMP>;
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, u8>;
#[doc = "Reader of field `DOY`"]
pub type DOY_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minutes value in the range of 0 to 59."]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hours value in the range of 0 to 23."]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:25 - Day of Year value in the range of 1 to 366."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
}
