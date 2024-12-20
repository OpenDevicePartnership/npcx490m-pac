#[doc = "Register `WKMODn8` reader"]
pub type R = crate::R<Wkmodn8Spec>;
#[doc = "Register `WKMODn8` writer"]
pub type W = crate::W<Wkmodn8Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkmodn8Spec;
impl crate::RegisterSpec for Wkmodn8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkmodn8::R`](R) reader structure"]
impl crate::Readable for Wkmodn8Spec {}
#[doc = "`write(|w| ..)` method takes [`wkmodn8::W`](W) writer structure"]
impl crate::Writable for Wkmodn8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKMODn8 to value 0"]
impl crate::Resettable for Wkmodn8Spec {
    const RESET_VALUE: u8 = 0;
}
