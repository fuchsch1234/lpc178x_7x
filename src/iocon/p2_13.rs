#[doc = "Reader of register P2_13"]
pub type R = crate::R<u32, super::P2_13>;
#[doc = "Writer for register P2_13"]
pub type W = crate::W<u32, super::P2_13>;
#[doc = "Selects pin function for pin P2\\[13\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin. This\r\n                                            pin includes a 5 ns input glitch filter."]
    P2_13 = 0,
    #[doc = "1: External interrupt 3 input."]
    EINT3 = 1,
    #[doc = "2: Data line 3 for SD card interface."]
    SD_DAT_3 = 2,
    #[doc = "3: Transmit data. It is driven by the transmitter                                             and read by the receiver. Corresponds to the signal SD                                             in the                                                 I2S-bus                                                 specification."]
    I2S_TX_SDA = 3,
    #[doc = "5: LCD data."]
    LCD_VD_5 = 5,
    #[doc = "6: LCD data."]
    LCD_VD_9 = 6,
    #[doc = "7: LCD data."]
    LCD_VD_19 = 7,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FUNC`"]
pub type FUNC_R = crate::R<u8, FUNC_A>;
impl FUNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FUNC_A::P2_13),
            1 => Val(FUNC_A::EINT3),
            2 => Val(FUNC_A::SD_DAT_3),
            3 => Val(FUNC_A::I2S_TX_SDA),
            5 => Val(FUNC_A::LCD_VD_5),
            6 => Val(FUNC_A::LCD_VD_9),
            7 => Val(FUNC_A::LCD_VD_19),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P2_13`"]
    #[inline(always)]
    pub fn is_p2_13(&self) -> bool {
        *self == FUNC_A::P2_13
    }
    #[doc = "Checks if the value of the field is `EINT3`"]
    #[inline(always)]
    pub fn is_eint3(&self) -> bool {
        *self == FUNC_A::EINT3
    }
    #[doc = "Checks if the value of the field is `SD_DAT_3`"]
    #[inline(always)]
    pub fn is_sd_dat_3(&self) -> bool {
        *self == FUNC_A::SD_DAT_3
    }
    #[doc = "Checks if the value of the field is `I2S_TX_SDA`"]
    #[inline(always)]
    pub fn is_i2s_tx_sda(&self) -> bool {
        *self == FUNC_A::I2S_TX_SDA
    }
    #[doc = "Checks if the value of the field is `LCD_VD_5`"]
    #[inline(always)]
    pub fn is_lcd_vd_5(&self) -> bool {
        *self == FUNC_A::LCD_VD_5
    }
    #[doc = "Checks if the value of the field is `LCD_VD_9`"]
    #[inline(always)]
    pub fn is_lcd_vd_9(&self) -> bool {
        *self == FUNC_A::LCD_VD_9
    }
    #[doc = "Checks if the value of the field is `LCD_VD_19`"]
    #[inline(always)]
    pub fn is_lcd_vd_19(&self) -> bool {
        *self == FUNC_A::LCD_VD_19
    }
}
#[doc = "Write proxy for field `FUNC`"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline(always)]
    pub fn p2_13(self) -> &'a mut W {
        self.variant(FUNC_A::P2_13)
    }
    #[doc = "External interrupt 3 input."]
    #[inline(always)]
    pub fn eint3(self) -> &'a mut W {
        self.variant(FUNC_A::EINT3)
    }
    #[doc = "Data line 3 for SD card interface."]
    #[inline(always)]
    pub fn sd_dat_3(self) -> &'a mut W {
        self.variant(FUNC_A::SD_DAT_3)
    }
    #[doc = "Transmit data. It is driven by the transmitter and read by the receiver. Corresponds to the signal SD in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_tx_sda(self) -> &'a mut W {
        self.variant(FUNC_A::I2S_TX_SDA)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_5(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_VD_5)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_9(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_VD_9)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_19(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_VD_19)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[13\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[13\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
