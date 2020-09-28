#[doc = "Reader of register DYNAMICCONTROL"]
pub type R = crate::R<u32, super::DYNAMICCONTROL>;
#[doc = "Writer for register DYNAMICCONTROL"]
pub type W = crate::W<u32, super::DYNAMICCONTROL>;
#[doc = "Register DYNAMICCONTROL `reset()`'s with value 0x06"]
impl crate::ResetValue for super::DYNAMICCONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Dynamic memory clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CE_A {
    #[doc = "0: Clock enable of idle devices are deasserted to save power (POR reset value)."]
    POWERSAVE = 0,
    #[doc = "1: All clock enables are driven HIGH continuously.\\[1\\]"]
    HIGH = 1,
}
impl From<CE_A> for bool {
    #[inline(always)]
    fn from(variant: CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CE`"]
pub type CE_R = crate::R<bool, CE_A>;
impl CE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CE_A {
        match self.bits {
            false => CE_A::POWERSAVE,
            true => CE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `POWERSAVE`"]
    #[inline(always)]
    pub fn is_powersave(&self) -> bool {
        *self == CE_A::POWERSAVE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CE_A::HIGH
    }
}
#[doc = "Write proxy for field `CE`"]
pub struct CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock enable of idle devices are deasserted to save power (POR reset value)."]
    #[inline(always)]
    pub fn powersave(self) -> &'a mut W {
        self.variant(CE_A::POWERSAVE)
    }
    #[doc = "All clock enables are driven HIGH continuously.\\[1\\]"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CE_A::HIGH)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
    #[doc = "0: CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    STOP = 0,
    #[doc = "1: CLKOUT runs continuously (POR reset value)."]
    RUN = 1,
}
impl From<CS_A> for bool {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<bool, CS_A>;
impl CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            false => CS_A::STOP,
            true => CS_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CS_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == CS_A::RUN
    }
}
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CS_A::STOP)
    }
    #[doc = "CLKOUT runs continuously (POR reset value)."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CS_A::RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: Normal mode."]
    NORMAL_MODE_ = 0,
    #[doc = "1: Enter self-refresh mode (POR reset value)."]
    ENTER_SELF_REFRESH_M = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, SR_A>;
impl SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::NORMAL_MODE_,
            true => SR_A::ENTER_SELF_REFRESH_M,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE_`"]
    #[inline(always)]
    pub fn is_normal_mode_(&self) -> bool {
        *self == SR_A::NORMAL_MODE_
    }
    #[doc = "Checks if the value of the field is `ENTER_SELF_REFRESH_M`"]
    #[inline(always)]
    pub fn is_enter_self_refresh_m(&self) -> bool {
        *self == SR_A::ENTER_SELF_REFRESH_M
    }
}
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal_mode_(self) -> &'a mut W {
        self.variant(SR_A::NORMAL_MODE_)
    }
    #[doc = "Enter self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn enter_self_refresh_m(self) -> &'a mut W {
        self.variant(SR_A::ENTER_SELF_REFRESH_M)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Memory clock control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMC_A {
    #[doc = "0: CLKOUT enabled (POR reset value)."]
    CLKOUT_ENABLED_POR_ = 0,
    #[doc = "1: CLKOUT disabled.\\[3\\]"]
    CLKOUT_DISABLED = 1,
}
impl From<MMC_A> for bool {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MMC`"]
pub type MMC_R = crate::R<bool, MMC_A>;
impl MMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMC_A {
        match self.bits {
            false => MMC_A::CLKOUT_ENABLED_POR_,
            true => MMC_A::CLKOUT_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CLKOUT_ENABLED_POR_`"]
    #[inline(always)]
    pub fn is_clkout_enabled_por_(&self) -> bool {
        *self == MMC_A::CLKOUT_ENABLED_POR_
    }
    #[doc = "Checks if the value of the field is `CLKOUT_DISABLED`"]
    #[inline(always)]
    pub fn is_clkout_disabled(&self) -> bool {
        *self == MMC_A::CLKOUT_DISABLED
    }
}
#[doc = "Write proxy for field `MMC`"]
pub struct MMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CLKOUT enabled (POR reset value)."]
    #[inline(always)]
    pub fn clkout_enabled_por_(self) -> &'a mut W {
        self.variant(MMC_A::CLKOUT_ENABLED_POR_)
    }
    #[doc = "CLKOUT disabled.\\[3\\]"]
    #[inline(always)]
    pub fn clkout_disabled(self) -> &'a mut W {
        self.variant(MMC_A::CLKOUT_DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "SDRAM initialization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I_A {
    #[doc = "0: Issue SDRAM NORMAL operation command (POR reset value)."]
    NORMAL = 0,
    #[doc = "1: Issue SDRAM MODE command."]
    MODE = 1,
    #[doc = "2: Issue SDRAM PALL (precharge all) command."]
    PALL = 2,
    #[doc = "3: Issue SDRAM NOP (no operation) command)"]
    NOP = 3,
}
impl From<I_A> for u8 {
    #[inline(always)]
    fn from(variant: I_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I`"]
pub type I_R = crate::R<u8, I_A>;
impl I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I_A {
        match self.bits {
            0 => I_A::NORMAL,
            1 => I_A::MODE,
            2 => I_A::PALL,
            3 => I_A::NOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == I_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MODE`"]
    #[inline(always)]
    pub fn is_mode(&self) -> bool {
        *self == I_A::MODE
    }
    #[doc = "Checks if the value of the field is `PALL`"]
    #[inline(always)]
    pub fn is_pall(&self) -> bool {
        *self == I_A::PALL
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == I_A::NOP
    }
}
#[doc = "Write proxy for field `I`"]
pub struct I_W<'a> {
    w: &'a mut W,
}
impl<'a> I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Issue SDRAM NORMAL operation command (POR reset value)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(I_A::NORMAL)
    }
    #[doc = "Issue SDRAM MODE command."]
    #[inline(always)]
    pub fn mode(self) -> &'a mut W {
        self.variant(I_A::MODE)
    }
    #[doc = "Issue SDRAM PALL (precharge all) command."]
    #[inline(always)]
    pub fn pall(self) -> &'a mut W {
        self.variant(I_A::PALL)
    }
    #[doc = "Issue SDRAM NOP (no operation) command)"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(I_A::NOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W {
        CE_W { w: self }
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W { w: self }
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&mut self) -> I_W {
        I_W { w: self }
    }
}
