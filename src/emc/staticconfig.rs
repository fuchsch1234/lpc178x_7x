#[doc = "Reader of register STATICCONFIG%s"]
pub type R = crate::R<u32, super::STATICCONFIG>;
#[doc = "Writer for register STATICCONFIG%s"]
pub type W = crate::W<u32, super::STATICCONFIG>;
#[doc = "Register STATICCONFIG%s `reset()`'s with value 0"]
impl crate::ResetValue for super::STATICCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Memory width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MW_A {
    #[doc = "0: 8 bit (POR reset value)."]
    _8_BIT_POR_RESET_VAL = 0,
    #[doc = "1: 16 bit."]
    _16_BIT_ = 1,
    #[doc = "2: 32 bit."]
    _32_BIT_ = 2,
}
impl From<MW_A> for u8 {
    #[inline(always)]
    fn from(variant: MW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MW`"]
pub type MW_R = crate::R<u8, MW_A>;
impl MW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MW_A::_8_BIT_POR_RESET_VAL),
            1 => Val(MW_A::_16_BIT_),
            2 => Val(MW_A::_32_BIT_),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT_POR_RESET_VAL`"]
    #[inline(always)]
    pub fn is_8_bit_por_reset_val(&self) -> bool {
        *self == MW_A::_8_BIT_POR_RESET_VAL
    }
    #[doc = "Checks if the value of the field is `_16_BIT_`"]
    #[inline(always)]
    pub fn is_16_bit_(&self) -> bool {
        *self == MW_A::_16_BIT_
    }
    #[doc = "Checks if the value of the field is `_32_BIT_`"]
    #[inline(always)]
    pub fn is_32_bit_(&self) -> bool {
        *self == MW_A::_32_BIT_
    }
}
#[doc = "Write proxy for field `MW`"]
pub struct MW_W<'a> {
    w: &'a mut W,
}
impl<'a> MW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bit (POR reset value)."]
    #[inline(always)]
    pub fn _8_bit_por_reset_val(self) -> &'a mut W {
        self.variant(MW_A::_8_BIT_POR_RESET_VAL)
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn _16_bit_(self) -> &'a mut W {
        self.variant(MW_A::_16_BIT_)
    }
    #[doc = "32 bit."]
    #[inline(always)]
    pub fn _32_bit_(self) -> &'a mut W {
        self.variant(MW_A::_32_BIT_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    #[doc = "0: Disabled (POR reset value)."]
    DISABLED_POR_RESET_ = 0,
    #[doc = "1: Asynchronous page mode enabled (page length four)."]
    ASYNCHRONOUS_PAGE_MO = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<bool, PM_A>;
impl PM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::DISABLED_POR_RESET_,
            true => PM_A::ASYNCHRONOUS_PAGE_MO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_POR_RESET_`"]
    #[inline(always)]
    pub fn is_disabled_por_reset_(&self) -> bool {
        *self == PM_A::DISABLED_POR_RESET_
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_PAGE_MO`"]
    #[inline(always)]
    pub fn is_asynchronous_page_mo(&self) -> bool {
        *self == PM_A::ASYNCHRONOUS_PAGE_MO
    }
}
#[doc = "Write proxy for field `PM`"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled (POR reset value)."]
    #[inline(always)]
    pub fn disabled_por_reset_(self) -> &'a mut W {
        self.variant(PM_A::DISABLED_POR_RESET_)
    }
    #[doc = "Asynchronous page mode enabled (page length four)."]
    #[inline(always)]
    pub fn asynchronous_page_mo(self) -> &'a mut W {
        self.variant(PM_A::ASYNCHRONOUS_PAGE_MO)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Chip select polarity. The value of the chip select polarity on power-on reset is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC_A {
    #[doc = "0: Active LOW chip select."]
    ACTIVE_LOW_CHIP_SELE = 0,
    #[doc = "1: Active HIGH chip select."]
    ACTIVE_HIGH_CHIP_SEL = 1,
}
impl From<PC_A> for bool {
    #[inline(always)]
    fn from(variant: PC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<bool, PC_A>;
impl PC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PC_A {
        match self.bits {
            false => PC_A::ACTIVE_LOW_CHIP_SELE,
            true => PC_A::ACTIVE_HIGH_CHIP_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW_CHIP_SELE`"]
    #[inline(always)]
    pub fn is_active_low_chip_sele(&self) -> bool {
        *self == PC_A::ACTIVE_LOW_CHIP_SELE
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH_CHIP_SEL`"]
    #[inline(always)]
    pub fn is_active_high_chip_sel(&self) -> bool {
        *self == PC_A::ACTIVE_HIGH_CHIP_SEL
    }
}
#[doc = "Write proxy for field `PC`"]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active LOW chip select."]
    #[inline(always)]
    pub fn active_low_chip_sele(self) -> &'a mut W {
        self.variant(PC_A::ACTIVE_LOW_CHIP_SELE)
    }
    #[doc = "Active HIGH chip select."]
    #[inline(always)]
    pub fn active_high_chip_sel(self) -> &'a mut W {
        self.variant(PC_A::ACTIVE_HIGH_CHIP_SEL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PB_A {
    #[doc = "0: For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    BLSHIGH = 0,
    #[doc = "1: For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    BLSLOW = 1,
}
impl From<PB_A> for bool {
    #[inline(always)]
    fn from(variant: PB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PB`"]
pub type PB_R = crate::R<bool, PB_A>;
impl PB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB_A {
        match self.bits {
            false => PB_A::BLSHIGH,
            true => PB_A::BLSLOW,
        }
    }
    #[doc = "Checks if the value of the field is `BLSHIGH`"]
    #[inline(always)]
    pub fn is_blshigh(&self) -> bool {
        *self == PB_A::BLSHIGH
    }
    #[doc = "Checks if the value of the field is `BLSLOW`"]
    #[inline(always)]
    pub fn is_blslow(&self) -> bool {
        *self == PB_A::BLSLOW
    }
}
#[doc = "Write proxy for field `PB`"]
pub struct PB_W<'a> {
    w: &'a mut W,
}
impl<'a> PB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    #[inline(always)]
    pub fn blshigh(self) -> &'a mut W {
        self.variant(PB_A::BLSHIGH)
    }
    #[doc = "For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    #[inline(always)]
    pub fn blslow(self) -> &'a mut W {
        self.variant(PB_A::BLSLOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_A {
    #[doc = "0: Extended wait disabled (POR reset value)."]
    EXTENDED_WAIT_DISABL = 0,
    #[doc = "1: Extended wait enabled."]
    EXTENDED_WAIT_ENABLE = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EW`"]
pub type EW_R = crate::R<bool, EW_A>;
impl EW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::EXTENDED_WAIT_DISABL,
            true => EW_A::EXTENDED_WAIT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `EXTENDED_WAIT_DISABL`"]
    #[inline(always)]
    pub fn is_extended_wait_disabl(&self) -> bool {
        *self == EW_A::EXTENDED_WAIT_DISABL
    }
    #[doc = "Checks if the value of the field is `EXTENDED_WAIT_ENABLE`"]
    #[inline(always)]
    pub fn is_extended_wait_enable(&self) -> bool {
        *self == EW_A::EXTENDED_WAIT_ENABLE
    }
}
#[doc = "Write proxy for field `EW`"]
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Extended wait disabled (POR reset value)."]
    #[inline(always)]
    pub fn extended_wait_disabl(self) -> &'a mut W {
        self.variant(EW_A::EXTENDED_WAIT_DISABL)
    }
    #[doc = "Extended wait enabled."]
    #[inline(always)]
    pub fn extended_wait_enable(self) -> &'a mut W {
        self.variant(EW_A::EXTENDED_WAIT_ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Buffer enable \\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B_A {
    #[doc = "0: Buffer disabled (POR reset value)."]
    BUFFER_DISABLED_POR = 0,
    #[doc = "1: Buffer enabled."]
    BUFFER_ENABLED_ = 1,
}
impl From<B_A> for bool {
    #[inline(always)]
    fn from(variant: B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<bool, B_A>;
impl B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B_A {
        match self.bits {
            false => B_A::BUFFER_DISABLED_POR,
            true => B_A::BUFFER_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `BUFFER_DISABLED_POR`"]
    #[inline(always)]
    pub fn is_buffer_disabled_por(&self) -> bool {
        *self == B_A::BUFFER_DISABLED_POR
    }
    #[doc = "Checks if the value of the field is `BUFFER_ENABLED_`"]
    #[inline(always)]
    pub fn is_buffer_enabled_(&self) -> bool {
        *self == B_A::BUFFER_ENABLED_
    }
}
#[doc = "Write proxy for field `B`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffer disabled (POR reset value)."]
    #[inline(always)]
    pub fn buffer_disabled_por(self) -> &'a mut W {
        self.variant(B_A::BUFFER_DISABLED_POR)
    }
    #[doc = "Buffer enabled."]
    #[inline(always)]
    pub fn buffer_enabled_(self) -> &'a mut W {
        self.variant(B_A::BUFFER_ENABLED_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P_A {
    #[doc = "0: Writes not protected (POR reset value)."]
    WRITES_NOT_PROTECTED = 0,
    #[doc = "1: Write protected."]
    WRITE_PROTECTED_ = 1,
}
impl From<P_A> for bool {
    #[inline(always)]
    fn from(variant: P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, P_A>;
impl P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P_A {
        match self.bits {
            false => P_A::WRITES_NOT_PROTECTED,
            true => P_A::WRITE_PROTECTED_,
        }
    }
    #[doc = "Checks if the value of the field is `WRITES_NOT_PROTECTED`"]
    #[inline(always)]
    pub fn is_writes_not_protected(&self) -> bool {
        *self == P_A::WRITES_NOT_PROTECTED
    }
    #[doc = "Checks if the value of the field is `WRITE_PROTECTED_`"]
    #[inline(always)]
    pub fn is_write_protected_(&self) -> bool {
        *self == P_A::WRITE_PROTECTED_
    }
}
#[doc = "Write proxy for field `P`"]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes not protected (POR reset value)."]
    #[inline(always)]
    pub fn writes_not_protected(self) -> &'a mut W {
        self.variant(P_A::WRITES_NOT_PROTECTED)
    }
    #[doc = "Write protected."]
    #[inline(always)]
    pub fn write_protected_(self) -> &'a mut W {
        self.variant(P_A::WRITE_PROTECTED_)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W {
        MW_W { w: self }
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W {
        PB_W { w: self }
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
}
