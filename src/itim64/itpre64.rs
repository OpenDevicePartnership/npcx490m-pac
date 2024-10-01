#[doc = "Register `ITPRE64` reader"]
pub type R = crate::R<Itpre64Spec>;
#[doc = "Register `ITPRE64` writer"]
pub type W = crate::W<Itpre64Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal Timer Prescaler Register (ITPRE64)\n\nYou can [`read`](crate::Reg::read) this register and get [`itpre64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itpre64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itpre64Spec;
impl crate::RegisterSpec for Itpre64Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itpre64::R`](R) reader structure"]
impl crate::Readable for Itpre64Spec {}
#[doc = "`write(|w| ..)` method takes [`itpre64::W`](W) writer structure"]
impl crate::Writable for Itpre64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITPRE64 to value 0"]
impl crate::Resettable for Itpre64Spec {
    const RESET_VALUE: u8 = 0;
}
