#[doc = "Reader of register FLASHCFG"]
pub type R = crate::R<u32, super::FLASHCFG>;
#[doc = "Writer for register FLASHCFG"]
pub type W = crate::W<u32, super::FLASHCFG>;
#[doc = "Register FLASHCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_1 = 0,
    #[doc = "1: Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_2 = 1,
    #[doc = "2: Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_3 = 2,
    #[doc = "3: Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    FLASH_ACCESSES_USE_4 = 3,
    #[doc = "4: Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    FLASH_ACCESSES_USE_5 = 4,
    #[doc = "5: Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    FLASH_ACCESSES_USE_6 = 5,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASHTIM`"]
pub type FLASHTIM_R = crate::R<u8, FLASHTIM_A>;
impl FLASHTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLASHTIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLASHTIM_A::FLASH_ACCESSES_USE_1),
            1 => Val(FLASHTIM_A::FLASH_ACCESSES_USE_2),
            2 => Val(FLASHTIM_A::FLASH_ACCESSES_USE_3),
            3 => Val(FLASHTIM_A::FLASH_ACCESSES_USE_4),
            4 => Val(FLASHTIM_A::FLASH_ACCESSES_USE_5),
            5 => Val(FLASHTIM_A::FLASH_ACCESSES_USE_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_1`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_1(&self) -> bool {
        *self == FLASHTIM_A::FLASH_ACCESSES_USE_1
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_2`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_2(&self) -> bool {
        *self == FLASHTIM_A::FLASH_ACCESSES_USE_2
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_3`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_3(&self) -> bool {
        *self == FLASHTIM_A::FLASH_ACCESSES_USE_3
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_4`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_4(&self) -> bool {
        *self == FLASHTIM_A::FLASH_ACCESSES_USE_4
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_5`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_5(&self) -> bool {
        *self == FLASHTIM_A::FLASH_ACCESSES_USE_5
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSES_USE_6`"]
    #[inline(always)]
    pub fn is_flash_accesses_use_6(&self) -> bool {
        *self == FLASHTIM_A::FLASH_ACCESSES_USE_6
    }
}
#[doc = "Write proxy for field `FLASHTIM`"]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_1(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_1)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_2(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_2)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_3(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_3)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    #[inline(always)]
    pub fn flash_accesses_use_4(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_4)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_5(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_5)
    }
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    #[inline(always)]
    pub fn flash_accesses_use_6(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASH_ACCESSES_USE_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
}
