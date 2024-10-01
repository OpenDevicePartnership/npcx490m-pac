#[doc = "Register `WKMODn6` reader"]
pub type R = crate::R<Wkmodn6Spec>;
#[doc = "Register `WKMODn6` writer"]
pub type W = crate::W<Wkmodn6Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkmodn6Spec;
impl crate::RegisterSpec for Wkmodn6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkmodn6::R`](R) reader structure"]
impl crate::Readable for Wkmodn6Spec {}
#[doc = "`write(|w| ..)` method takes [`wkmodn6::W`](W) writer structure"]
impl crate::Writable for Wkmodn6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKMODn6 to value 0"]
impl crate::Resettable for Wkmodn6Spec {
    const RESET_VALUE: u8 = 0;
}
