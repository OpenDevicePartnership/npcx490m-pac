#[doc = "Register `TnCRB` reader"]
pub type R = crate::R<TnCrbSpec>;
#[doc = "Register `TnCRB` writer"]
pub type W = crate::W<TnCrbSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reload/Capture B Register (TnCRB)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_crb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_crb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCrbSpec;
impl crate::RegisterSpec for TnCrbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tn_crb::R`](R) reader structure"]
impl crate::Readable for TnCrbSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_crb::W`](W) writer structure"]
impl crate::Writable for TnCrbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TnCRB to value 0"]
impl crate::Resettable for TnCrbSpec {
    const RESET_VALUE: u16 = 0;
}
