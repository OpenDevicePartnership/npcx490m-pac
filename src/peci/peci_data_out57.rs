#[doc = "Register `PECI_DATA_OUT57` reader"]
pub type R = crate::R<PeciDataOut57Spec>;
#[doc = "Register `PECI_DATA_OUT57` writer"]
pub type W = crate::W<PeciDataOut57Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out57::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out57::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciDataOut57Spec;
impl crate::RegisterSpec for PeciDataOut57Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_data_out57::R`](R) reader structure"]
impl crate::Readable for PeciDataOut57Spec {}
#[doc = "`write(|w| ..)` method takes [`peci_data_out57::W`](W) writer structure"]
impl crate::Writable for PeciDataOut57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_DATA_OUT57 to value 0"]
impl crate::Resettable for PeciDataOut57Spec {
    const RESET_VALUE: u8 = 0;
}
