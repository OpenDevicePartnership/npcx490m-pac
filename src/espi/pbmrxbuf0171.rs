#[doc = "Register `PBMRXBUF0-171` reader"]
pub type R = crate::R<Pbmrxbuf0171Spec>;
#[doc = "Register `PBMRXBUF0-171` writer"]
pub type W = crate::W<Pbmrxbuf0171Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0171::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0171::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbmrxbuf0171Spec;
impl crate::RegisterSpec for Pbmrxbuf0171Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbmrxbuf0171::R`](R) reader structure"]
impl crate::Readable for Pbmrxbuf0171Spec {}
#[doc = "`write(|w| ..)` method takes [`pbmrxbuf0171::W`](W) writer structure"]
impl crate::Writable for Pbmrxbuf0171Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBMRXBUF0-171 to value 0"]
impl crate::Resettable for Pbmrxbuf0171Spec {
    const RESET_VALUE: u32 = 0;
}
