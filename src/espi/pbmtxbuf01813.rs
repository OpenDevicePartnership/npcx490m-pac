#[doc = "Register `PBMTXBUF0-1813` reader"]
pub type R = crate::R<Pbmtxbuf01813Spec>;
#[doc = "Register `PBMTXBUF0-1813` writer"]
pub type W = crate::W<Pbmtxbuf01813Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01813::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01813::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbmtxbuf01813Spec;
impl crate::RegisterSpec for Pbmtxbuf01813Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbmtxbuf01813::R`](R) reader structure"]
impl crate::Readable for Pbmtxbuf01813Spec {}
#[doc = "`write(|w| ..)` method takes [`pbmtxbuf01813::W`](W) writer structure"]
impl crate::Writable for Pbmtxbuf01813Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBMTXBUF0-1813 to value 0"]
impl crate::Resettable for Pbmtxbuf01813Spec {
    const RESET_VALUE: u32 = 0;
}
