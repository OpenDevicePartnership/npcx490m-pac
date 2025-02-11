#[doc = "Register `PBMRXRDHEAD` reader"]
pub type R = crate::R<PbmrxrdheadSpec>;
#[doc = "Register `PBMRXRDHEAD` writer"]
pub type W = crate::W<PbmrxrdheadSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Bus Master Receive Buffer Read Head Register (PBMRXRDHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxrdhead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxrdhead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbmrxrdheadSpec;
impl crate::RegisterSpec for PbmrxrdheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbmrxrdhead::R`](R) reader structure"]
impl crate::Readable for PbmrxrdheadSpec {}
#[doc = "`write(|w| ..)` method takes [`pbmrxrdhead::W`](W) writer structure"]
impl crate::Writable for PbmrxrdheadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBMRXRDHEAD to value 0"]
impl crate::Resettable for PbmrxrdheadSpec {
    const RESET_VALUE: u32 = 0;
}
