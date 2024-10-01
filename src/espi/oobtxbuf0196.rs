#[doc = "Register `OOBTXBUF0-196` reader"]
pub type R = crate::R<Oobtxbuf0196Spec>;
#[doc = "Register `OOBTXBUF0-196` writer"]
pub type W = crate::W<Oobtxbuf0196Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0196::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0196::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oobtxbuf0196Spec;
impl crate::RegisterSpec for Oobtxbuf0196Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oobtxbuf0196::R`](R) reader structure"]
impl crate::Readable for Oobtxbuf0196Spec {}
#[doc = "`write(|w| ..)` method takes [`oobtxbuf0196::W`](W) writer structure"]
impl crate::Writable for Oobtxbuf0196Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOBTXBUF0-196 to value 0"]
impl crate::Resettable for Oobtxbuf0196Spec {
    const RESET_VALUE: u32 = 0;
}
