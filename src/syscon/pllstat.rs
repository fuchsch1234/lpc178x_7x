#[doc = "Reader of register PLL%sSTAT"]
pub type R = crate::R<u32, super::PLLSTAT>;
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `PLLE_STAT`"]
pub type PLLE_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLOCK`"]
pub type PLOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Read-back for the PLL Multiplier value. This is the value currently used by the related PLL."]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Read-back for the PLL Divider value. This is the value currently used by the related PLL."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Read-back for the PLL Enable bit. When one, the related PLL is currently activated. When zero, the related PLL is turned off. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn plle_stat(&self) -> PLLE_STAT_R {
        PLLE_STAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reflects the PLL Lock status. When zero, the related PLL is not locked. When one, the related PLL is locked onto the requested frequency."]
    #[inline(always)]
    pub fn plock(&self) -> PLOCK_R {
        PLOCK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
