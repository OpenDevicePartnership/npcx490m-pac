#[doc = "Register `WKENn4` reader"]
pub type R = crate::R<Wkenn4Spec>;
#[doc = "Register `WKENn4` writer"]
pub type W = crate::W<Wkenn4Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkenn4Spec;
impl crate::RegisterSpec for Wkenn4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkenn4::R`](R) reader structure"]
impl crate::Readable for Wkenn4Spec {}
#[doc = "`write(|w| ..)` method takes [`wkenn4::W`](W) writer structure"]
impl crate::Writable for Wkenn4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKENn4 to value 0"]
impl crate::Resettable for Wkenn4Spec {
    const RESET_VALUE: u8 = 0;
}
