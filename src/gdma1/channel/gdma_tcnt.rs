#[doc = "Register `GDMA_TCNT` reader"]
pub type R = crate::R<GdmaTcntSpec>;
#[doc = "Register `GDMA_TCNT` writer"]
pub type W = crate::W<GdmaTcntSpec>;
#[doc = "Field `TFR_CNT` reader - 24-bit Transfer Count"]
pub type TfrCntR = crate::FieldReader<u32>;
#[doc = "Field `TFR_CNT` writer - 24-bit Transfer Count"]
pub type TfrCntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 24-bit Transfer Count"]
    #[inline(always)]
    pub fn tfr_cnt(&self) -> TfrCntR {
        TfrCntR::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA_TCNT")
            .field("tfr_cnt", &self.tfr_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - 24-bit Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn tfr_cnt(&mut self) -> TfrCntW<GdmaTcntSpec> {
        TfrCntW::new(self, 0)
    }
}
#[doc = "Channel 0/1 Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_tcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_tcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaTcntSpec;
impl crate::RegisterSpec for GdmaTcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_tcnt::R`](R) reader structure"]
impl crate::Readable for GdmaTcntSpec {}
#[doc = "`write(|w| ..)` method takes [`gdma_tcnt::W`](W) writer structure"]
impl crate::Writable for GdmaTcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_TCNT to value 0"]
impl crate::Resettable for GdmaTcntSpec {
    const RESET_VALUE: u32 = 0;
}
