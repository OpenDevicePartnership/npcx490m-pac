#[doc = "Register `FLASHTXBUF0-1612` reader"]
pub type R = crate::R<Flashtxbuf01612Spec>;
#[doc = "Register `FLASHTXBUF0-1612` writer"]
pub type W = crate::W<Flashtxbuf01612Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01612::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01612::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashtxbuf01612Spec;
impl crate::RegisterSpec for Flashtxbuf01612Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashtxbuf01612::R`](R) reader structure"]
impl crate::Readable for Flashtxbuf01612Spec {}
#[doc = "`write(|w| ..)` method takes [`flashtxbuf01612::W`](W) writer structure"]
impl crate::Writable for Flashtxbuf01612Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHTXBUF0-1612 to value 0"]
impl crate::Resettable for Flashtxbuf01612Spec {
    const RESET_VALUE: u32 = 0;
}
