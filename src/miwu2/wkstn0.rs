#[doc = "Register `WKSTn0` reader"]
pub type R = crate::R<Wkstn0Spec>;
#[doc = "Register `WKSTn0` writer"]
pub type W = crate::W<Wkstn0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkstn0Spec;
impl crate::RegisterSpec for Wkstn0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkstn0::R`](R) reader structure"]
impl crate::Readable for Wkstn0Spec {}
#[doc = "`write(|w| ..)` method takes [`wkstn0::W`](W) writer structure"]
impl crate::Writable for Wkstn0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKSTn0 to value 0"]
impl crate::Resettable for Wkstn0Spec {
    const RESET_VALUE: u8 = 0;
}
