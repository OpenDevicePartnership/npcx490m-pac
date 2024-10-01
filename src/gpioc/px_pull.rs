#[doc = "Register `PxPULL` reader"]
pub type R = crate::R<PxPullSpec>;
#[doc = "Register `PxPULL` writer"]
pub type W = crate::W<PxPullSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port GPIOx Pull-Up or Pull-Down Enable Register (PxPULL)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_pull::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_pull::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxPullSpec;
impl crate::RegisterSpec for PxPullSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_pull::R`](R) reader structure"]
impl crate::Readable for PxPullSpec {}
#[doc = "`write(|w| ..)` method takes [`px_pull::W`](W) writer structure"]
impl crate::Writable for PxPullSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxPULL to value 0"]
impl crate::Resettable for PxPullSpec {
    const RESET_VALUE: u8 = 0;
}
