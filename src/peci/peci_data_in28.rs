#[doc = "Register `PECI_DATA_IN28` reader"]
pub type R = crate::R<PeciDataIn28Spec>;
#[doc = "Register `PECI_DATA_IN28` writer"]
pub type W = crate::W<PeciDataIn28Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Data In Register (PECI_DATA_IN00-3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_data_in28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_data_in28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciDataIn28Spec;
impl crate::RegisterSpec for PeciDataIn28Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_data_in28::R`](R) reader structure"]
impl crate::Readable for PeciDataIn28Spec {}
#[doc = "`write(|w| ..)` method takes [`peci_data_in28::W`](W) writer structure"]
impl crate::Writable for PeciDataIn28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_DATA_IN28 to value 0"]
impl crate::Resettable for PeciDataIn28Spec {
    const RESET_VALUE: u8 = 0;
}
