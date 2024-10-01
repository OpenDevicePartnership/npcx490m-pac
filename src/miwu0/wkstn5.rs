#[doc = "Register `WKSTn5` reader"]
pub type R = crate::R<Wkstn5Spec>;
#[doc = "Register `WKSTn5` writer"]
pub type W = crate::W<Wkstn5Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkstn5Spec;
impl crate::RegisterSpec for Wkstn5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkstn5::R`](R) reader structure"]
impl crate::Readable for Wkstn5Spec {}
#[doc = "`write(|w| ..)` method takes [`wkstn5::W`](W) writer structure"]
impl crate::Writable for Wkstn5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKSTn5 to value 0"]
impl crate::Resettable for Wkstn5Spec {
    const RESET_VALUE: u8 = 0;
}
