#[doc = "Register `GDMA_DSTB` reader"]
pub type R = crate::R<GdmaDstbSpec>;
#[doc = "Register `GDMA_DSTB` writer"]
pub type W = crate::W<GdmaDstbSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Destination Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_dstb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_dstb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdmaDstbSpec;
impl crate::RegisterSpec for GdmaDstbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_dstb::R`](R) reader structure"]
impl crate::Readable for GdmaDstbSpec {}
#[doc = "`write(|w| ..)` method takes [`gdma_dstb::W`](W) writer structure"]
impl crate::Writable for GdmaDstbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_DSTB to value 0"]
impl crate::Resettable for GdmaDstbSpec {
    const RESET_VALUE: u32 = 0;
}
