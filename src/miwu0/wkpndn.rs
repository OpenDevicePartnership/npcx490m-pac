#[doc = "Register `WKPNDn%s` reader"]
pub type R = crate::R<WkpndnSpec>;
#[doc = "Register `WKPNDn%s` writer"]
pub type W = crate::W<WkpndnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkpndnSpec;
impl crate::RegisterSpec for WkpndnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkpndn::R`](R) reader structure"]
impl crate::Readable for WkpndnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkpndn::W`](W) writer structure"]
impl crate::Writable for WkpndnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPNDn%s to value 0"]
impl crate::Resettable for WkpndnSpec {
    const RESET_VALUE: u8 = 0;
}
