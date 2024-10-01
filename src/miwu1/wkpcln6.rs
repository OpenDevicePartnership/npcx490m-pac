#[doc = "Register `WKPCLn6` reader"]
pub type R = crate::R<Wkpcln6Spec>;
#[doc = "Register `WKPCLn6` writer"]
pub type W = crate::W<Wkpcln6Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkpcln6Spec;
impl crate::RegisterSpec for Wkpcln6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkpcln6::R`](R) reader structure"]
impl crate::Readable for Wkpcln6Spec {}
#[doc = "`write(|w| ..)` method takes [`wkpcln6::W`](W) writer structure"]
impl crate::Writable for Wkpcln6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPCLn6 to value 0"]
impl crate::Resettable for Wkpcln6Spec {
    const RESET_VALUE: u8 = 0;
}
