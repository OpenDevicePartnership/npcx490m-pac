#[doc = "Register `WKPNDn0` reader"]
pub type R = crate::R<Wkpndn0Spec>;
#[doc = "Register `WKPNDn0` writer"]
pub type W = crate::W<Wkpndn0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkpndn0Spec;
impl crate::RegisterSpec for Wkpndn0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkpndn0::R`](R) reader structure"]
impl crate::Readable for Wkpndn0Spec {}
#[doc = "`write(|w| ..)` method takes [`wkpndn0::W`](W) writer structure"]
impl crate::Writable for Wkpndn0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPNDn0 to value 0"]
impl crate::Resettable for Wkpndn0Spec {
    const RESET_VALUE: u8 = 0;
}
