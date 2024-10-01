#[doc = "Register `TWMWD` reader"]
pub type R = crate::R<TwmwdSpec>;
#[doc = "Register `TWMWD` writer"]
pub type W = crate::W<TwmwdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Watchdog Counter Register (TWMWD)\n\nYou can [`read`](crate::Reg::read) this register and get [`twmwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twmwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwmwdSpec;
impl crate::RegisterSpec for TwmwdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twmwd::R`](R) reader structure"]
impl crate::Readable for TwmwdSpec {}
#[doc = "`write(|w| ..)` method takes [`twmwd::W`](W) writer structure"]
impl crate::Writable for TwmwdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TWMWD to value 0"]
impl crate::Resettable for TwmwdSpec {
    const RESET_VALUE: u8 = 0;
}
