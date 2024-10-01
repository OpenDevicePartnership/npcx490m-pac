#[doc = "Register `WDCNT` reader"]
pub type R = crate::R<WdcntSpec>;
#[doc = "Register `WDCNT` writer"]
pub type W = crate::W<WdcntSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Watchdog Count Register (WDCNT)\n\nYou can [`read`](crate::Reg::read) this register and get [`wdcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdcntSpec;
impl crate::RegisterSpec for WdcntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdcnt::R`](R) reader structure"]
impl crate::Readable for WdcntSpec {}
#[doc = "`write(|w| ..)` method takes [`wdcnt::W`](W) writer structure"]
impl crate::Writable for WdcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDCNT to value 0"]
impl crate::Resettable for WdcntSpec {
    const RESET_VALUE: u8 = 0;
}
