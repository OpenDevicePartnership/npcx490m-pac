#[doc = "Register `WKINENn0` reader"]
pub type R = crate::R<Wkinenn0Spec>;
#[doc = "Register `WKINENn0` writer"]
pub type W = crate::W<Wkinenn0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkinenn0Spec;
impl crate::RegisterSpec for Wkinenn0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkinenn0::R`](R) reader structure"]
impl crate::Readable for Wkinenn0Spec {}
#[doc = "`write(|w| ..)` method takes [`wkinenn0::W`](W) writer structure"]
impl crate::Writable for Wkinenn0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKINENn0 to value 0"]
impl crate::Resettable for Wkinenn0Spec {
    const RESET_VALUE: u8 = 0;
}
