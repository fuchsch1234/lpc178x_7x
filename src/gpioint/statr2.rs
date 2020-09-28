#[doc = "Reader of register STATR2"]
pub type R = crate::R<u32, super::STATR2>;
#[doc = "Reader of field `P2_0REI`"]
pub type P2_0REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_1REI`"]
pub type P2_1REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_2REI`"]
pub type P2_2REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_3REI`"]
pub type P2_3REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_4REI`"]
pub type P2_4REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_5REI`"]
pub type P2_5REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_6REI`"]
pub type P2_6REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_7REI`"]
pub type P2_7REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_8REI`"]
pub type P2_8REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_9REI`"]
pub type P2_9REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_10REI`"]
pub type P2_10REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_11REI`"]
pub type P2_11REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_12REI`"]
pub type P2_12REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_13REI`"]
pub type P2_13REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_14REI`"]
pub type P2_14REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_15REI`"]
pub type P2_15REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_16REI`"]
pub type P2_16REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_17REI`"]
pub type P2_17REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_18REI`"]
pub type P2_18REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_19REI`"]
pub type P2_19REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_20REI`"]
pub type P2_20REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_21REI`"]
pub type P2_21REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_22REI`"]
pub type P2_22REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_23REI`"]
pub type P2_23REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_24REI`"]
pub type P2_24REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_25REI`"]
pub type P2_25REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_26REI`"]
pub type P2_26REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_27REI`"]
pub type P2_27REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_28REI`"]
pub type P2_28REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_29REI`"]
pub type P2_29REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_30REI`"]
pub type P2_30REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_31REI`"]
pub type P2_31REI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_0rei(&self) -> P2_0REI_R {
        P2_0REI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_1rei(&self) -> P2_1REI_R {
        P2_1REI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_2rei(&self) -> P2_2REI_R {
        P2_2REI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_3rei(&self) -> P2_3REI_R {
        P2_3REI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_4rei(&self) -> P2_4REI_R {
        P2_4REI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_5rei(&self) -> P2_5REI_R {
        P2_5REI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_6rei(&self) -> P2_6REI_R {
        P2_6REI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_7rei(&self) -> P2_7REI_R {
        P2_7REI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_8rei(&self) -> P2_8REI_R {
        P2_8REI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_9rei(&self) -> P2_9REI_R {
        P2_9REI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10rei(&self) -> P2_10REI_R {
        P2_10REI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11rei(&self) -> P2_11REI_R {
        P2_11REI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12rei(&self) -> P2_12REI_R {
        P2_12REI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13rei(&self) -> P2_13REI_R {
        P2_13REI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Status of Rising Edge Interrupt for P2\\[14\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_14rei(&self) -> P2_14REI_R {
        P2_14REI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Status of Rising Edge Interrupt for P2\\[15\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_15rei(&self) -> P2_15REI_R {
        P2_15REI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status of Rising Edge Interrupt for P2\\[16\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_16rei(&self) -> P2_16REI_R {
        P2_16REI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status of Rising Edge Interrupt for P2\\[17\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_17rei(&self) -> P2_17REI_R {
        P2_17REI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Status of Rising Edge Interrupt for P2\\[18\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_18rei(&self) -> P2_18REI_R {
        P2_18REI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Status of Rising Edge Interrupt for P2\\[19\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_19rei(&self) -> P2_19REI_R {
        P2_19REI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Status of Rising Edge Interrupt for P2\\[20\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_20rei(&self) -> P2_20REI_R {
        P2_20REI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Status of Rising Edge Interrupt for P2\\[21\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_21rei(&self) -> P2_21REI_R {
        P2_21REI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Status of Rising Edge Interrupt for P2\\[22\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_22rei(&self) -> P2_22REI_R {
        P2_22REI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Status of Rising Edge Interrupt for P2\\[23\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_23rei(&self) -> P2_23REI_R {
        P2_23REI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Status of Rising Edge Interrupt for P2\\[24\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_24rei(&self) -> P2_24REI_R {
        P2_24REI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Status of Rising Edge Interrupt for P2\\[25\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_25rei(&self) -> P2_25REI_R {
        P2_25REI_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Status of Rising Edge Interrupt for P2\\[26\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_26rei(&self) -> P2_26REI_R {
        P2_26REI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Status of Rising Edge Interrupt for P2\\[27\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_27rei(&self) -> P2_27REI_R {
        P2_27REI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Status of Rising Edge Interrupt for P2\\[28\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_28rei(&self) -> P2_28REI_R {
        P2_28REI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Status of Rising Edge Interrupt for P2\\[29\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_29rei(&self) -> P2_29REI_R {
        P2_29REI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Status of Rising Edge Interrupt for P2\\[30\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_30rei(&self) -> P2_30REI_R {
        P2_30REI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status of Rising Edge Interrupt for P2\\[31\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_31rei(&self) -> P2_31REI_R {
        P2_31REI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
