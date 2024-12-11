#[doc = "Register `FLM_CMBEV` reader"]
pub type R = crate::R<FlmCmbevSpec>;
#[doc = "Register `FLM_CMBEV` writer"]
pub type W = crate::W<FlmCmbevSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FLM CMB Event Register (FLM_CMBEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmbev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmbev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmbevSpec;
impl crate::RegisterSpec for FlmCmbevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmbev::R`](R) reader structure"]
impl crate::Readable for FlmCmbevSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_cmbev::W`](W) writer structure"]
impl crate::Writable for FlmCmbevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CMBEV to value 0"]
impl crate::Resettable for FlmCmbevSpec {
    const RESET_VALUE: u32 = 0;
}
