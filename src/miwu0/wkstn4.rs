#[doc = "Register `WKSTn4` reader"]
pub type R = crate::R<Wkstn4Spec>;
#[doc = "Register `WKSTn4` writer"]
pub type W = crate::W<Wkstn4Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkstn4Spec;
impl crate::RegisterSpec for Wkstn4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkstn4::R`](R) reader structure"]
impl crate::Readable for Wkstn4Spec {}
#[doc = "`write(|w| ..)` method takes [`wkstn4::W`](W) writer structure"]
impl crate::Writable for Wkstn4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKSTn4 to value 0"]
impl crate::Resettable for Wkstn4Spec {
    const RESET_VALUE: u8 = 0;
}
