#[doc = "Register `GDMA_CDST1` reader"]
pub type R = crate::R<GdmaCdst1Spec>;
#[doc = "Register `GDMA_CDST1` writer"]
pub type W = crate::W<GdmaCdst1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_cdst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_cdst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaCdst1Spec;
impl crate::RegisterSpec for GdmaCdst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_cdst1::R`](R) reader structure"]
impl crate::Readable for GdmaCdst1Spec {}
#[doc = "`write(|w| ..)` method takes [`gdma_cdst1::W`](W) writer structure"]
impl crate::Writable for GdmaCdst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_CDST1 to value 0"]
impl crate::Resettable for GdmaCdst1Spec {
    const RESET_VALUE: u32 = 0;
}
