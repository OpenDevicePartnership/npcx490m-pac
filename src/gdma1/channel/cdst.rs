#[doc = "Register `CDST` reader"]
pub type R = crate::R<CdstSpec>;
#[doc = "Register `CDST` writer"]
pub type W = crate::W<CdstSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)\n\nYou can [`read`](crate::Reg::read) this register and get [`cdst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdstSpec;
impl crate::RegisterSpec for CdstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdst::R`](R) reader structure"]
impl crate::Readable for CdstSpec {}
#[doc = "`write(|w| ..)` method takes [`cdst::W`](W) writer structure"]
impl crate::Writable for CdstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDST to value 0"]
impl crate::Resettable for CdstSpec {
    const RESET_VALUE: u32 = 0;
}
