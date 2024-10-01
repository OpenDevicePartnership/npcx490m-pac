#[doc = "Register `PECI_DATA_OUT41` reader"]
pub type R = crate::R<PeciDataOut41Spec>;
#[doc = "Register `PECI_DATA_OUT41` writer"]
pub type W = crate::W<PeciDataOut41Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Data Out Register (PECI_DATA_OUT00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_out41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_out41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciDataOut41Spec;
impl crate::RegisterSpec for PeciDataOut41Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_data_out41::R`](R) reader structure"]
impl crate::Readable for PeciDataOut41Spec {}
#[doc = "`write(|w| ..)` method takes [`peci_data_out41::W`](W) writer structure"]
impl crate::Writable for PeciDataOut41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_DATA_OUT41 to value 0"]
impl crate::Resettable for PeciDataOut41Spec {
    const RESET_VALUE: u8 = 0;
}
