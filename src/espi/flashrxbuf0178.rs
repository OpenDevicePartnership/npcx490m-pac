#[doc = "Register `FLASHRXBUF0-178` reader"]
pub type R = crate::R<Flashrxbuf0178Spec>;
#[doc = "Register `FLASHRXBUF0-178` writer"]
pub type W = crate::W<Flashrxbuf0178Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashrxbuf0178Spec;
impl crate::RegisterSpec for Flashrxbuf0178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrxbuf0178::R`](R) reader structure"]
impl crate::Readable for Flashrxbuf0178Spec {}
#[doc = "`write(|w| ..)` method takes [`flashrxbuf0178::W`](W) writer structure"]
impl crate::Writable for Flashrxbuf0178Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHRXBUF0-178 to value 0"]
impl crate::Resettable for Flashrxbuf0178Spec {
    const RESET_VALUE: u32 = 0;
}
