#[doc = "Register `OOBRXBUF0-1914` reader"]
pub type R = crate::R<Oobrxbuf01914Spec>;
#[doc = "Register `OOBRXBUF0-1914` writer"]
pub type W = crate::W<Oobrxbuf01914Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01914::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01914::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oobrxbuf01914Spec;
impl crate::RegisterSpec for Oobrxbuf01914Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oobrxbuf01914::R`](R) reader structure"]
impl crate::Readable for Oobrxbuf01914Spec {}
#[doc = "`write(|w| ..)` method takes [`oobrxbuf01914::W`](W) writer structure"]
impl crate::Writable for Oobrxbuf01914Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOBRXBUF0-1914 to value 0"]
impl crate::Resettable for Oobrxbuf01914Spec {
    const RESET_VALUE: u32 = 0;
}
