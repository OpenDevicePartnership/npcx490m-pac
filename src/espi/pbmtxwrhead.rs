#[doc = "Register `PBMTXWRHEAD` reader"]
pub type R = crate::R<PbmtxwrheadSpec>;
#[doc = "Register `PBMTXWRHEAD` writer"]
pub type W = crate::W<PbmtxwrheadSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Bus Master Transmit Buffer Write Head Register (PBMTXWRHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxwrhead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxwrhead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbmtxwrheadSpec;
impl crate::RegisterSpec for PbmtxwrheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbmtxwrhead::R`](R) reader structure"]
impl crate::Readable for PbmtxwrheadSpec {}
#[doc = "`write(|w| ..)` method takes [`pbmtxwrhead::W`](W) writer structure"]
impl crate::Writable for PbmtxwrheadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBMTXWRHEAD to value 0"]
impl crate::Resettable for PbmtxwrheadSpec {
    const RESET_VALUE: u32 = 0;
}
