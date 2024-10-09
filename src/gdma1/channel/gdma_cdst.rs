#[doc = "Register `GDMA_CDST` reader"]
pub type R = crate::R<GdmaCdstSpec>;
#[doc = "Field `CURRENT_DST_ADDR` reader - 32-bit Current Destination Address"]
pub type CurrentDstAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit Current Destination Address"]
    #[inline(always)]
    pub fn current_dst_addr(&self) -> CurrentDstAddrR {
        CurrentDstAddrR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA_CDST")
            .field("current_dst_addr", &self.current_dst_addr())
            .finish()
    }
}
#[doc = "Channel 0/1 Current Destination Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_cdst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaCdstSpec;
impl crate::RegisterSpec for GdmaCdstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_cdst::R`](R) reader structure"]
impl crate::Readable for GdmaCdstSpec {}
#[doc = "`reset()` method sets GDMA_CDST to value 0"]
impl crate::Resettable for GdmaCdstSpec {
    const RESET_VALUE: u32 = 0;
}
