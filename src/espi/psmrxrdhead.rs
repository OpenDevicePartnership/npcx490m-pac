#[doc = "Register `PSMRXRDHEAD` reader"]
pub type R = crate::R<PsmrxrdheadSpec>;
#[doc = "Register `PSMRXRDHEAD` writer"]
pub type W = crate::W<PsmrxrdheadSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Target Message Receive Buffer Read Head Register (PSMRXRDHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxrdhead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxrdhead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsmrxrdheadSpec;
impl crate::RegisterSpec for PsmrxrdheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmrxrdhead::R`](R) reader structure"]
impl crate::Readable for PsmrxrdheadSpec {}
#[doc = "`write(|w| ..)` method takes [`psmrxrdhead::W`](W) writer structure"]
impl crate::Writable for PsmrxrdheadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSMRXRDHEAD to value 0"]
impl crate::Resettable for PsmrxrdheadSpec {
    const RESET_VALUE: u32 = 0;
}
