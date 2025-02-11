#[doc = "Register `PBMTXBUF%s` reader"]
pub type R = crate::R<PbmtxbufSpec>;
#[doc = "Register `PBMTXBUF%s` writer"]
pub type W = crate::W<PbmtxbufSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbmtxbufSpec;
impl crate::RegisterSpec for PbmtxbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbmtxbuf::R`](R) reader structure"]
impl crate::Readable for PbmtxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`pbmtxbuf::W`](W) writer structure"]
impl crate::Writable for PbmtxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBMTXBUF%s to value 0"]
impl crate::Resettable for PbmtxbufSpec {
    const RESET_VALUE: u32 = 0;
}
