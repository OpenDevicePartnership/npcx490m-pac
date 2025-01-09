#[doc = "Register `WKENn%s` reader"]
pub type R = crate::R<WkennSpec>;
#[doc = "Register `WKENn%s` writer"]
pub type W = crate::W<WkennSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkennSpec;
impl crate::RegisterSpec for WkennSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkenn::R`](R) reader structure"]
impl crate::Readable for WkennSpec {}
#[doc = "`write(|w| ..)` method takes [`wkenn::W`](W) writer structure"]
impl crate::Writable for WkennSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKENn%s to value 0"]
impl crate::Resettable for WkennSpec {
    const RESET_VALUE: u8 = 0;
}
