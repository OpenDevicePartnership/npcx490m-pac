#[doc = "Register `UTBUFn` reader"]
pub type R = crate::R<UtbufnSpec>;
#[doc = "Register `UTBUFn` writer"]
pub type W = crate::W<UtbufnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Transmit Data Buffer Register (UTBUFn)\n\nYou can [`read`](crate::Reg::read) this register and get [`utbufn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utbufn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtbufnSpec;
impl crate::RegisterSpec for UtbufnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`utbufn::R`](R) reader structure"]
impl crate::Readable for UtbufnSpec {}
#[doc = "`write(|w| ..)` method takes [`utbufn::W`](W) writer structure"]
impl crate::Writable for UtbufnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UTBUFn to value 0"]
impl crate::Resettable for UtbufnSpec {
    const RESET_VALUE: u8 = 0;
}
