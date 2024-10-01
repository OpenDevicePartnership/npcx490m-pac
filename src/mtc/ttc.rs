#[doc = "Register `TTC` reader"]
pub type R = crate::R<TtcSpec>;
#[doc = "Register `TTC` writer"]
pub type W = crate::W<TtcSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timing Ticks Count Register (TTC)\n\nYou can [`read`](crate::Reg::read) this register and get [`ttc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtcSpec;
impl crate::RegisterSpec for TtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttc::R`](R) reader structure"]
impl crate::Readable for TtcSpec {}
#[doc = "`write(|w| ..)` method takes [`ttc::W`](W) writer structure"]
impl crate::Writable for TtcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTC to value 0"]
impl crate::Resettable for TtcSpec {
    const RESET_VALUE: u32 = 0;
}
