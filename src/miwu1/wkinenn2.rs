#[doc = "Register `WKINENn2` reader"]
pub type R = crate::R<Wkinenn2Spec>;
#[doc = "Register `WKINENn2` writer"]
pub type W = crate::W<Wkinenn2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkinenn2Spec;
impl crate::RegisterSpec for Wkinenn2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkinenn2::R`](R) reader structure"]
impl crate::Readable for Wkinenn2Spec {}
#[doc = "`write(|w| ..)` method takes [`wkinenn2::W`](W) writer structure"]
impl crate::Writable for Wkinenn2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKINENn2 to value 0"]
impl crate::Resettable for Wkinenn2Spec {
    const RESET_VALUE: u8 = 0;
}
