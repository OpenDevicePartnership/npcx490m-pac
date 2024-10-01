#[doc = "Register `WKPNDn7` reader"]
pub type R = crate::R<Wkpndn7Spec>;
#[doc = "Register `WKPNDn7` writer"]
pub type W = crate::W<Wkpndn7Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkpndn7Spec;
impl crate::RegisterSpec for Wkpndn7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkpndn7::R`](R) reader structure"]
impl crate::Readable for Wkpndn7Spec {}
#[doc = "`write(|w| ..)` method takes [`wkpndn7::W`](W) writer structure"]
impl crate::Writable for Wkpndn7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPNDn7 to value 0"]
impl crate::Resettable for Wkpndn7Spec {
    const RESET_VALUE: u8 = 0;
}
