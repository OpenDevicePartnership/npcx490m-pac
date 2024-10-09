#[doc = "Register `GDMA_SRCB` reader"]
pub type R = crate::R<GdmaSrcbSpec>;
#[doc = "Register `GDMA_SRCB` writer"]
pub type W = crate::W<GdmaSrcbSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Source Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_srcb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_srcb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaSrcbSpec;
impl crate::RegisterSpec for GdmaSrcbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_srcb::R`](R) reader structure"]
impl crate::Readable for GdmaSrcbSpec {}
#[doc = "`write(|w| ..)` method takes [`gdma_srcb::W`](W) writer structure"]
impl crate::Writable for GdmaSrcbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_SRCB to value 0"]
impl crate::Resettable for GdmaSrcbSpec {
    const RESET_VALUE: u32 = 0;
}
