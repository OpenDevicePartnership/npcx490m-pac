#[doc = "Register `PBMTXBUF0-188` reader"]
pub type R = crate::R<Pbmtxbuf0188Spec>;
#[doc = "Register `PBMTXBUF0-188` writer"]
pub type W = crate::W<Pbmtxbuf0188Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0188::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0188::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbmtxbuf0188Spec;
impl crate::RegisterSpec for Pbmtxbuf0188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbmtxbuf0188::R`](R) reader structure"]
impl crate::Readable for Pbmtxbuf0188Spec {}
#[doc = "`write(|w| ..)` method takes [`pbmtxbuf0188::W`](W) writer structure"]
impl crate::Writable for Pbmtxbuf0188Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBMTXBUF0-188 to value 0"]
impl crate::Resettable for Pbmtxbuf0188Spec {
    const RESET_VALUE: u32 = 0;
}
