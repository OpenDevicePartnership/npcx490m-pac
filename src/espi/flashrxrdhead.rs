#[doc = "Register `FLASHRXRDHEAD` reader"]
pub type R = crate::R<FlashrxrdheadSpec>;
#[doc = "Register `FLASHRXRDHEAD` writer"]
pub type W = crate::W<FlashrxrdheadSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Receive Buffer Read Head Register (FLASHRXRDHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxrdhead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxrdhead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashrxrdheadSpec;
impl crate::RegisterSpec for FlashrxrdheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrxrdhead::R`](R) reader structure"]
impl crate::Readable for FlashrxrdheadSpec {}
#[doc = "`write(|w| ..)` method takes [`flashrxrdhead::W`](W) writer structure"]
impl crate::Writable for FlashrxrdheadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHRXRDHEAD to value 0"]
impl crate::Resettable for FlashrxrdheadSpec {
    const RESET_VALUE: u32 = 0;
}
