#[doc = "Register `PECI_DATA_IN46` reader"]
pub type R = crate::R<PeciDataIn46Spec>;
#[doc = "Register `PECI_DATA_IN46` writer"]
pub type W = crate::W<PeciDataIn46Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciDataIn46Spec;
impl crate::RegisterSpec for PeciDataIn46Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_data_in46::R`](R) reader structure"]
impl crate::Readable for PeciDataIn46Spec {}
#[doc = "`write(|w| ..)` method takes [`peci_data_in46::W`](W) writer structure"]
impl crate::Writable for PeciDataIn46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_DATA_IN46 to value 0"]
impl crate::Resettable for PeciDataIn46Spec {
    const RESET_VALUE: u8 = 0;
}
