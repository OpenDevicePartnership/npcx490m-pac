#[doc = "Register `FLM_CMDEV` reader"]
pub type R = crate::R<FlmCmdevSpec>;
#[doc = "Register `FLM_CMDEV` writer"]
pub type W = crate::W<FlmCmdevSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FLM CMD Event Register (FLM_CMDEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmdevSpec;
impl crate::RegisterSpec for FlmCmdevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmdev::R`](R) reader structure"]
impl crate::Readable for FlmCmdevSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_cmdev::W`](W) writer structure"]
impl crate::Writable for FlmCmdevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CMDEV to value 0"]
impl crate::Resettable for FlmCmdevSpec {
    const RESET_VALUE: u32 = 0;
}
