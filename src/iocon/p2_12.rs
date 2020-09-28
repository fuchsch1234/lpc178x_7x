#[doc = "Reader of register P2_12"]
pub type R = crate::R<u32, super::P2_12>;
#[doc = "Writer for register P2_12"]
pub type W = crate::W<u32, super::P2_12>;
#[doc = "Selects pin function for pin P2\\[12\\]"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: General purpose digital input/output pin. This\r\n                                            pin includes a 5 ns input glitch filter."]
    P2_12 = 0,
    #[doc = "1: External interrupt 2 input."]
    EINT2 = 1,
    #[doc = "2: Data line 2 for SD card interface."]
    SD_DAT_2 = 2,
    #[doc = "3: Transmit Word Select. It is driven by the master                                             and received by the slave. Corresponds to the signal WS                                             in the                                                 I2S-bus                                                 specification."]
    I2S_TX_WS = 3,
    #[doc = "4: LCD data."]
    LCD_VD_4 = 4,
    #[doc = "5: LCD data."]
    LCD_VD_3 = 5,
    #[doc = "6: LCD data."]
    LCD_VD_8 = 6,
    #[doc = "7: LCD data."]
    LCD_VD_18 = 7,
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
    pub fn variant(&self) -> FUNC_A {
        match self.bits {
            0 => FUNC_A::P2_12,
            1 => FUNC_A::EINT2,
            2 => FUNC_A::SD_DAT_2,
            3 => FUNC_A::I2S_TX_WS,
            4 => FUNC_A::LCD_VD_4,
            5 => FUNC_A::LCD_VD_3,
            6 => FUNC_A::LCD_VD_8,
            7 => FUNC_A::LCD_VD_18,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P2_12`"]
    #[inline(always)]
    pub fn is_p2_12(&self) -> bool {
        *self == FUNC_A::P2_12
    }
    #[doc = "Checks if the value of the field is `EINT2`"]
    #[inline(always)]
    pub fn is_eint2(&self) -> bool {
        *self == FUNC_A::EINT2
    }
    #[doc = "Checks if the value of the field is `SD_DAT_2`"]
    #[inline(always)]
    pub fn is_sd_dat_2(&self) -> bool {
        *self == FUNC_A::SD_DAT_2
    }
    #[doc = "Checks if the value of the field is `I2S_TX_WS`"]
    #[inline(always)]
    pub fn is_i2s_tx_ws(&self) -> bool {
        *self == FUNC_A::I2S_TX_WS
    }
    #[doc = "Checks if the value of the field is `LCD_VD_4`"]
    #[inline(always)]
    pub fn is_lcd_vd_4(&self) -> bool {
        *self == FUNC_A::LCD_VD_4
    }
    #[doc = "Checks if the value of the field is `LCD_VD_3`"]
    #[inline(always)]
    pub fn is_lcd_vd_3(&self) -> bool {
        *self == FUNC_A::LCD_VD_3
    }
    #[doc = "Checks if the value of the field is `LCD_VD_8`"]
    #[inline(always)]
    pub fn is_lcd_vd_8(&self) -> bool {
        *self == FUNC_A::LCD_VD_8
    }
    #[doc = "Checks if the value of the field is `LCD_VD_18`"]
    #[inline(always)]
    pub fn is_lcd_vd_18(&self) -> bool {
        *self == FUNC_A::LCD_VD_18
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline(always)]
    pub fn p2_12(self) -> &'a mut W {
        self.variant(FUNC_A::P2_12)
    }
    #[doc = "External interrupt 2 input."]
    #[inline(always)]
    pub fn eint2(self) -> &'a mut W {
        self.variant(FUNC_A::EINT2)
    }
    #[doc = "Data line 2 for SD card interface."]
    #[inline(always)]
    pub fn sd_dat_2(self) -> &'a mut W {
        self.variant(FUNC_A::SD_DAT_2)
    }
    #[doc = "Transmit Word Select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_tx_ws(self) -> &'a mut W {
        self.variant(FUNC_A::I2S_TX_WS)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_4(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_VD_4)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_3(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_VD_3)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_8(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_VD_8)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_18(self) -> &'a mut W {
        self.variant(FUNC_A::LCD_VD_18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[12\\]"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[12\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
}
