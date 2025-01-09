#[doc = "Register `WKINENn%s` reader"]
pub type R = crate::R<WkinennSpec>;
#[doc = "Register `WKINENn%s` writer"]
pub type W = crate::W<WkinennSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkinennSpec;
impl crate::RegisterSpec for WkinennSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkinenn::R`](R) reader structure"]
impl crate::Readable for WkinennSpec {}
#[doc = "`write(|w| ..)` method takes [`wkinenn::W`](W) writer structure"]
impl crate::Writable for WkinennSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKINENn%s to value 0"]
impl crate::Resettable for WkinennSpec {
    const RESET_VALUE: u8 = 0;
}
