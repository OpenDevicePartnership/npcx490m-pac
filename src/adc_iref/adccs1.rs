#[doc = "Register `ADCCS1` reader"]
pub type R = crate::R<Adccs1Spec>;
#[doc = "Register `ADCCS1` writer"]
pub type W = crate::W<Adccs1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC Scan Channels Select 1 Register (ADCCS1)\n\nYou can [`read`](crate::Reg::read) this register and get [`adccs1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccs1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adccs1Spec;
impl crate::RegisterSpec for Adccs1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adccs1::R`](R) reader structure"]
impl crate::Readable for Adccs1Spec {}
#[doc = "`write(|w| ..)` method takes [`adccs1::W`](W) writer structure"]
impl crate::Writable for Adccs1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADCCS1 to value 0"]
impl crate::Resettable for Adccs1Spec {
    const RESET_VALUE: u16 = 0;
}
