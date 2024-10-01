#[doc = "Register `PBMRXBUF0-178` reader"]
pub type R = crate::R<Pbmrxbuf0178Spec>;
#[doc = "Register `PBMRXBUF0-178` writer"]
pub type W = crate::W<Pbmrxbuf0178Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbmrxbuf0178Spec;
impl crate::RegisterSpec for Pbmrxbuf0178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbmrxbuf0178::R`](R) reader structure"]
impl crate::Readable for Pbmrxbuf0178Spec {}
#[doc = "`write(|w| ..)` method takes [`pbmrxbuf0178::W`](W) writer structure"]
impl crate::Writable for Pbmrxbuf0178Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBMRXBUF0-178 to value 0"]
impl crate::Resettable for Pbmrxbuf0178Spec {
    const RESET_VALUE: u32 = 0;
}
