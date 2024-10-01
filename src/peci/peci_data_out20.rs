#[doc = "Register `PECI_DATA_OUT20` reader"]
pub type R = crate::R<PeciDataOut20Spec>;
#[doc = "Register `PECI_DATA_OUT20` writer"]
pub type W = crate::W<PeciDataOut20Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciDataOut20Spec;
impl crate::RegisterSpec for PeciDataOut20Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_data_out20::R`](R) reader structure"]
impl crate::Readable for PeciDataOut20Spec {}
#[doc = "`write(|w| ..)` method takes [`peci_data_out20::W`](W) writer structure"]
impl crate::Writable for PeciDataOut20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_DATA_OUT20 to value 0"]
impl crate::Resettable for PeciDataOut20Spec {
    const RESET_VALUE: u8 = 0;
}
