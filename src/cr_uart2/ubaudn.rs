#[doc = "Register `UBAUDn` reader"]
pub type R = crate::R<UbaudnSpec>;
#[doc = "Register `UBAUDn` writer"]
pub type W = crate::W<UbaudnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Baud Rate Divisor Register (UBAUDn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ubaudn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubaudn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UbaudnSpec;
impl crate::RegisterSpec for UbaudnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ubaudn::R`](R) reader structure"]
impl crate::Readable for UbaudnSpec {}
#[doc = "`write(|w| ..)` method takes [`ubaudn::W`](W) writer structure"]
impl crate::Writable for UbaudnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UBAUDn to value 0"]
impl crate::Resettable for UbaudnSpec {
    const RESET_VALUE: u8 = 0;
}
