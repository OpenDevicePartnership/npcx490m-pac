#[doc = "Register `FLM_CMDEN` reader"]
pub type R = crate::R<FlmCmdenSpec>;
#[doc = "Register `FLM_CMDEN` writer"]
pub type W = crate::W<FlmCmdenSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FLM Command Enable (FLM_CMDEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmden::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmden::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmdenSpec;
impl crate::RegisterSpec for FlmCmdenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmden::R`](R) reader structure"]
impl crate::Readable for FlmCmdenSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_cmden::W`](W) writer structure"]
impl crate::Writable for FlmCmdenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CMDEN to value 0"]
impl crate::Resettable for FlmCmdenSpec {
    const RESET_VALUE: u32 = 0;
}
