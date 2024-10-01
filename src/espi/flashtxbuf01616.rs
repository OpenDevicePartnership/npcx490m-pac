#[doc = "Register `FLASHTXBUF0-1616` reader"]
pub type R = crate::R<Flashtxbuf01616Spec>;
#[doc = "Register `FLASHTXBUF0-1616` writer"]
pub type W = crate::W<Flashtxbuf01616Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01616::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01616::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashtxbuf01616Spec;
impl crate::RegisterSpec for Flashtxbuf01616Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashtxbuf01616::R`](R) reader structure"]
impl crate::Readable for Flashtxbuf01616Spec {}
#[doc = "`write(|w| ..)` method takes [`flashtxbuf01616::W`](W) writer structure"]
impl crate::Writable for Flashtxbuf01616Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHTXBUF0-1616 to value 0"]
impl crate::Resettable for Flashtxbuf01616Spec {
    const RESET_VALUE: u32 = 0;
}
