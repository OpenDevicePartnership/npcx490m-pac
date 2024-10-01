#[doc = "Register `GDMA_SRCB0` reader"]
pub type R = crate::R<GdmaSrcb0Spec>;
#[doc = "Register `GDMA_SRCB0` writer"]
pub type W = crate::W<GdmaSrcb0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_srcb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_srcb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaSrcb0Spec;
impl crate::RegisterSpec for GdmaSrcb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_srcb0::R`](R) reader structure"]
impl crate::Readable for GdmaSrcb0Spec {}
#[doc = "`write(|w| ..)` method takes [`gdma_srcb0::W`](W) writer structure"]
impl crate::Writable for GdmaSrcb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_SRCB0 to value 0"]
impl crate::Resettable for GdmaSrcb0Spec {
    const RESET_VALUE: u32 = 0;
}
