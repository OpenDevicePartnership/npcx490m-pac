#[doc = "Register `WKPCLn5` reader"]
pub type R = crate::R<Wkpcln5Spec>;
#[doc = "Register `WKPCLn5` writer"]
pub type W = crate::W<Wkpcln5Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkpcln5Spec;
impl crate::RegisterSpec for Wkpcln5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkpcln5::R`](R) reader structure"]
impl crate::Readable for Wkpcln5Spec {}
#[doc = "`write(|w| ..)` method takes [`wkpcln5::W`](W) writer structure"]
impl crate::Writable for Wkpcln5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPCLn5 to value 0"]
impl crate::Resettable for Wkpcln5Spec {
    const RESET_VALUE: u8 = 0;
}
