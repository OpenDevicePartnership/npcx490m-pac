#[doc = "Register `WDSDM` reader"]
pub type R = crate::R<WdsdmSpec>;
#[doc = "Register `WDSDM` writer"]
pub type W = crate::W<WdsdmSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Watchdog Service Data Match Register (WDSDM)\n\nYou can [`read`](crate::Reg::read) this register and get [`wdsdm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdsdm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdsdmSpec;
impl crate::RegisterSpec for WdsdmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdsdm::R`](R) reader structure"]
impl crate::Readable for WdsdmSpec {}
#[doc = "`write(|w| ..)` method takes [`wdsdm::W`](W) writer structure"]
impl crate::Writable for WdsdmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDSDM to value 0"]
impl crate::Resettable for WdsdmSpec {
    const RESET_VALUE: u8 = 0;
}
