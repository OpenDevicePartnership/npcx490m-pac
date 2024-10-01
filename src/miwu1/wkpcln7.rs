#[doc = "Register `WKPCLn7` reader"]
pub type R = crate::R<Wkpcln7Spec>;
#[doc = "Register `WKPCLn7` writer"]
pub type W = crate::W<Wkpcln7Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkpcln7Spec;
impl crate::RegisterSpec for Wkpcln7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkpcln7::R`](R) reader structure"]
impl crate::Readable for Wkpcln7Spec {}
#[doc = "`write(|w| ..)` method takes [`wkpcln7::W`](W) writer structure"]
impl crate::Writable for Wkpcln7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPCLn7 to value 0"]
impl crate::Resettable for Wkpcln7Spec {
    const RESET_VALUE: u8 = 0;
}
