#[doc = "Register `TnCRA` reader"]
pub type R = crate::R<TnCraSpec>;
#[doc = "Register `TnCRA` writer"]
pub type W = crate::W<TnCraSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reload/Capture A Register (TnCRA)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCraSpec;
impl crate::RegisterSpec for TnCraSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tn_cra::R`](R) reader structure"]
impl crate::Readable for TnCraSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_cra::W`](W) writer structure"]
impl crate::Writable for TnCraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TnCRA to value 0"]
impl crate::Resettable for TnCraSpec {
    const RESET_VALUE: u16 = 0;
}
