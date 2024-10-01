#[doc = "Register `WKEDGn6` reader"]
pub type R = crate::R<Wkedgn6Spec>;
#[doc = "Register `WKEDGn6` writer"]
pub type W = crate::W<Wkedgn6Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkedgn6Spec;
impl crate::RegisterSpec for Wkedgn6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkedgn6::R`](R) reader structure"]
impl crate::Readable for Wkedgn6Spec {}
#[doc = "`write(|w| ..)` method takes [`wkedgn6::W`](W) writer structure"]
impl crate::Writable for Wkedgn6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKEDGn6 to value 0"]
impl crate::Resettable for Wkedgn6Spec {
    const RESET_VALUE: u8 = 0;
}
