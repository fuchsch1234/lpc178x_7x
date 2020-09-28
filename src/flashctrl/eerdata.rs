#[doc = "Reader of register EERDATA"]
pub type R = crate::R<u32, super::EERDATA>;
#[doc = "Reader of field `RDATA`"]
pub type RDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read data. In case of: 8-bit read operations: bits \\[7:0\\]
contain read data, others are zero. 16-bit read operations: bits \\[15:0\\]
contain read data, others are zero. 32-bit read operations: bits \\[31:0\\]
contain read data."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
