#[doc = "Register `SDPD1` reader"]
pub type R = crate::R<Sdpd1Spec>;
#[doc = "Register `SDPD1` writer"]
pub type W = crate::W<Sdpd1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Simple Debug Port Data 1 Register (SDPD1)\n\nYou can [`read`](crate::Reg::read) this register and get [`sdpd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdpd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdpd1Spec;
impl crate::RegisterSpec for Sdpd1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdpd1::R`](R) reader structure"]
impl crate::Readable for Sdpd1Spec {}
#[doc = "`write(|w| ..)` method takes [`sdpd1::W`](W) writer structure"]
impl crate::Writable for Sdpd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDPD1 to value 0"]
impl crate::Resettable for Sdpd1Spec {
    const RESET_VALUE: u8 = 0;
}
