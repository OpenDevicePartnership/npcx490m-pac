#[doc = "Register `WKEDGn1` reader"]
pub type R = crate::R<Wkedgn1Spec>;
#[doc = "Register `WKEDGn1` writer"]
pub type W = crate::W<Wkedgn1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkedgn1Spec;
impl crate::RegisterSpec for Wkedgn1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkedgn1::R`](R) reader structure"]
impl crate::Readable for Wkedgn1Spec {}
#[doc = "`write(|w| ..)` method takes [`wkedgn1::W`](W) writer structure"]
impl crate::Writable for Wkedgn1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKEDGn1 to value 0"]
impl crate::Resettable for Wkedgn1Spec {
    const RESET_VALUE: u8 = 0;
}
