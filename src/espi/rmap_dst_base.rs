#[doc = "Register `RMAP_DST_BASE` reader"]
pub type R = crate::R<RmapDstBaseSpec>;
#[doc = "Register `RMAP_DST_BASE` writer"]
pub type W = crate::W<RmapDstBaseSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Remapping Destination Base Register (RMAP_DST_BASE)\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap_dst_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap_dst_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmapDstBaseSpec;
impl crate::RegisterSpec for RmapDstBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmap_dst_base::R`](R) reader structure"]
impl crate::Readable for RmapDstBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`rmap_dst_base::W`](W) writer structure"]
impl crate::Writable for RmapDstBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMAP_DST_BASE to value 0"]
impl crate::Resettable for RmapDstBaseSpec {
    const RESET_VALUE: u32 = 0;
}
