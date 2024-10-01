#[doc = "Register `FLASHTXBUF0-165` reader"]
pub type R = crate::R<Flashtxbuf0165Spec>;
#[doc = "Register `FLASHTXBUF0-165` writer"]
pub type W = crate::W<Flashtxbuf0165Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0165::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0165::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashtxbuf0165Spec;
impl crate::RegisterSpec for Flashtxbuf0165Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashtxbuf0165::R`](R) reader structure"]
impl crate::Readable for Flashtxbuf0165Spec {}
#[doc = "`write(|w| ..)` method takes [`flashtxbuf0165::W`](W) writer structure"]
impl crate::Writable for Flashtxbuf0165Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHTXBUF0-165 to value 0"]
impl crate::Resettable for Flashtxbuf0165Spec {
    const RESET_VALUE: u32 = 0;
}
