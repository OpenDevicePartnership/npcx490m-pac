#[doc = "Register `FLM_CMBEN` reader"]
pub type R = crate::R<FlmCmbenSpec>;
#[doc = "Register `FLM_CMBEN` writer"]
pub type W = crate::W<FlmCmbenSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FLM Command Byte Enable (FLM_CMBEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmben::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmben::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmbenSpec;
impl crate::RegisterSpec for FlmCmbenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmben::R`](R) reader structure"]
impl crate::Readable for FlmCmbenSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_cmben::W`](W) writer structure"]
impl crate::Writable for FlmCmbenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CMBEN to value 0"]
impl crate::Resettable for FlmCmbenSpec {
    const RESET_VALUE: u32 = 0;
}
