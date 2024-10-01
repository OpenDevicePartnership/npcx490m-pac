#[doc = "Register `WKEDGn7` reader"]
pub type R = crate::R<Wkedgn7Spec>;
#[doc = "Register `WKEDGn7` writer"]
pub type W = crate::W<Wkedgn7Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkedgn7Spec;
impl crate::RegisterSpec for Wkedgn7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkedgn7::R`](R) reader structure"]
impl crate::Readable for Wkedgn7Spec {}
#[doc = "`write(|w| ..)` method takes [`wkedgn7::W`](W) writer structure"]
impl crate::Writable for Wkedgn7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKEDGn7 to value 0"]
impl crate::Resettable for Wkedgn7Spec {
    const RESET_VALUE: u8 = 0;
}
