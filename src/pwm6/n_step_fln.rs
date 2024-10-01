#[doc = "Register `N_STEP_FLn` reader"]
pub type R = crate::R<NStepFlnSpec>;
#[doc = "Register `N_STEP_FLn` writer"]
pub type W = crate::W<NStepFlnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Number of Steps for Fall Duty Cycle Register (N_STEP_FLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`n_step_fln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_step_fln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NStepFlnSpec;
impl crate::RegisterSpec for NStepFlnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`n_step_fln::R`](R) reader structure"]
impl crate::Readable for NStepFlnSpec {}
#[doc = "`write(|w| ..)` method takes [`n_step_fln::W`](W) writer structure"]
impl crate::Writable for NStepFlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets N_STEP_FLn to value 0"]
impl crate::Resettable for NStepFlnSpec {
    const RESET_VALUE: u8 = 0;
}
