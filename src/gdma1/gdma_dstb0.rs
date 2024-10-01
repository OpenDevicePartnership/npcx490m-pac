#[doc = "Register `GDMA_DSTB0` reader"]
pub type R = crate::R<GdmaDstb0Spec>;
#[doc = "Register `GDMA_DSTB0` writer"]
pub type W = crate::W<GdmaDstb0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_dstb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_dstb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaDstb0Spec;
impl crate::RegisterSpec for GdmaDstb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_dstb0::R`](R) reader structure"]
impl crate::Readable for GdmaDstb0Spec {}
#[doc = "`write(|w| ..)` method takes [`gdma_dstb0::W`](W) writer structure"]
impl crate::Writable for GdmaDstb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_DSTB0 to value 0"]
impl crate::Resettable for GdmaDstb0Spec {
    const RESET_VALUE: u32 = 0;
}
