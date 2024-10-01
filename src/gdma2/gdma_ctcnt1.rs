#[doc = "Register `GDMA_CTCNT1` reader"]
pub type R = crate::R<GdmaCtcnt1Spec>;
#[doc = "Register `GDMA_CTCNT1` writer"]
pub type W = crate::W<GdmaCtcnt1Spec>;
#[doc = "Field `CURENT_TFR_CNT` reader - Current Transfer Count"]
pub type CurentTfrCntR = crate::FieldReader<u32>;
#[doc = "Field `CURENT_TFR_CNT` writer - Current Transfer Count"]
pub type CurentTfrCntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
        f.debug_struct("GDMA_CTCNT1")
            .field("curent_tfr_cnt", &self.curent_tfr_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Current Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn curent_tfr_cnt(&mut self) -> CurentTfrCntW<GdmaCtcnt1Spec> {
        CurentTfrCntW::new(self, 0)
    }
}
#[doc = "Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaCtcnt1Spec;
impl crate::RegisterSpec for GdmaCtcnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_ctcnt1::R`](R) reader structure"]
impl crate::Readable for GdmaCtcnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`gdma_ctcnt1::W`](W) writer structure"]
impl crate::Writable for GdmaCtcnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_CTCNT1 to value 0"]
impl crate::Resettable for GdmaCtcnt1Spec {
    const RESET_VALUE: u32 = 0;
}
