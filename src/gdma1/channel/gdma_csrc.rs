#[doc = "Register `GDMA_CSRC` reader"]
pub type R = crate::R<GdmaCsrcSpec>;
#[doc = "Field `CURRENT_SRC_ADDR` reader - 32-bit Current Source Address"]
pub type CurrentSrcAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit Current Source Address"]
    #[inline(always)]
    pub fn current_src_addr(&self) -> CurrentSrcAddrR {
        CurrentSrcAddrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA_CSRC")
            .field("current_src_addr", &self.current_src_addr())
            .finish()
    }
}
#[doc = "Channel 0/1 Current Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_csrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaCsrcSpec;
impl crate::RegisterSpec for GdmaCsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_csrc::R`](R) reader structure"]
impl crate::Readable for GdmaCsrcSpec {}
#[doc = "`reset()` method sets GDMA_CSRC to value 0"]
impl crate::Resettable for GdmaCsrcSpec {
    const RESET_VALUE: u32 = 0;
}
