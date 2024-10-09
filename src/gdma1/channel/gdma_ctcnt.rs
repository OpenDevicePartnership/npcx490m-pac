#[doc = "Register `GDMA_CTCNT` reader"]
pub type R = crate::R<GdmaCtcntSpec>;
#[doc = "Field `CURENT_TFR_CNT` reader - Current Transfer Count"]
pub type CurentTfrCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Current Transfer Count"]
    #[inline(always)]
    pub fn curent_tfr_cnt(&self) -> CurentTfrCntR {
        CurentTfrCntR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA_CTCNT")
            .field("curent_tfr_cnt", &self.curent_tfr_cnt())
            .finish()
    }
}
#[doc = "Channel 0/1 Current Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaCtcntSpec;
impl crate::RegisterSpec for GdmaCtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_ctcnt::R`](R) reader structure"]
impl crate::Readable for GdmaCtcntSpec {}
#[doc = "`reset()` method sets GDMA_CTCNT to value 0"]
impl crate::Resettable for GdmaCtcntSpec {
    const RESET_VALUE: u32 = 0;
}
