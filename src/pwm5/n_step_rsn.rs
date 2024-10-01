#[doc = "Register `N_STEP_RSn` reader"]
pub type R = crate::R<NStepRsnSpec>;
#[doc = "Register `N_STEP_RSn` writer"]
pub type W = crate::W<NStepRsnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Number of Steps for Rise Duty Cycle Register (N_STEP_RSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`n_step_rsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_step_rsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NStepRsnSpec;
impl crate::RegisterSpec for NStepRsnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`n_step_rsn::R`](R) reader structure"]
impl crate::Readable for NStepRsnSpec {}
#[doc = "`write(|w| ..)` method takes [`n_step_rsn::W`](W) writer structure"]
impl crate::Writable for NStepRsnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets N_STEP_RSn to value 0"]
impl crate::Resettable for NStepRsnSpec {
    const RESET_VALUE: u8 = 0;
}
