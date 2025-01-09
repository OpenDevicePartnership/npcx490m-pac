#[doc = "Register `WKMODn%s` reader"]
pub type R = crate::R<WkmodnSpec>;
#[doc = "Register `WKMODn%s` writer"]
pub type W = crate::W<WkmodnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkmodnSpec;
impl crate::RegisterSpec for WkmodnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkmodn::R`](R) reader structure"]
impl crate::Readable for WkmodnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkmodn::W`](W) writer structure"]
impl crate::Writable for WkmodnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKMODn%s to value 0"]
impl crate::Resettable for WkmodnSpec {
    const RESET_VALUE: u8 = 0;
}
