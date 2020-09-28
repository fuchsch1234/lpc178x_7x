#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Busy. This bit is used to ensure that the memory controller enters the low-power or disabled mode cleanly by determining if the memory controller is busy or not.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B_A {
    #[doc = "0: EMC\r\nis idle (warm reset value)."]
    IDLE = 0,
    #[doc = "1: EMC\r\nis busy performing memory transactions, commands, auto-refresh cycles,\r\nor is in self-refresh mode (POR reset value)."]
    BUSY = 1,
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
            false => B_A::IDLE,
            true => B_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == B_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == B_A::BUSY
    }
}
#[doc = "Write buffer status.This bit enables the EMC to enter low-power mode or disabled mode cleanly.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_A {
    #[doc = "0: Write buffers\r\nempty (POR reset value)"]
    EMPTY = 0,
    #[doc = "1: Write\r\nbuffers contain data."]
    DATA = 1,
}
impl From<S_A> for bool {
    #[inline(always)]
    fn from(variant: S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S`"]
pub type S_R = crate::R<bool, S_A>;
impl S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_A {
        match self.bits {
            false => S_A::EMPTY,
            true => S_A::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == S_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == S_A::DATA
    }
}
#[doc = "Self-refresh acknowledge. This bit indicates the operating mode of the EMC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Self-refresh mode (POR reset value)."]
    SELFREFRESH = 1,
}
impl From<SA_A> for bool {
    #[inline(always)]
    fn from(variant: SA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<bool, SA_A>;
impl SA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SA_A {
        match self.bits {
            false => SA_A::NORMAL,
            true => SA_A::SELFREFRESH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SA_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SELFREFRESH`"]
    #[inline(always)]
    pub fn is_selfrefresh(&self) -> bool {
        *self == SA_A::SELFREFRESH
    }
}
impl R {
    #[doc = "Bit 0 - Busy. This bit is used to ensure that the memory controller enters the low-power or disabled mode cleanly by determining if the memory controller is busy or not."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write buffer status.This bit enables the EMC to enter low-power mode or disabled mode cleanly."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self-refresh acknowledge. This bit indicates the operating mode of the EMC."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
