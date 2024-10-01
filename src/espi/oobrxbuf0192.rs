#[doc = "Register `OOBRXBUF0-192` reader"]
pub type R = crate::R<Oobrxbuf0192Spec>;
#[doc = "Register `OOBRXBUF0-192` writer"]
pub type W = crate::W<Oobrxbuf0192Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0192::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0192::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oobrxbuf0192Spec;
impl crate::RegisterSpec for Oobrxbuf0192Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oobrxbuf0192::R`](R) reader structure"]
impl crate::Readable for Oobrxbuf0192Spec {}
#[doc = "`write(|w| ..)` method takes [`oobrxbuf0192::W`](W) writer structure"]
impl crate::Writable for Oobrxbuf0192Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOBRXBUF0-192 to value 0"]
impl crate::Resettable for Oobrxbuf0192Spec {
    const RESET_VALUE: u32 = 0;
}
