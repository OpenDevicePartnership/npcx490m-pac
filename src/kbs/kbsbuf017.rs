#[doc = "Register `KBSBUF0-17` reader"]
pub type R = crate::R<Kbsbuf017Spec>;
#[doc = "Register `KBSBUF0-17` writer"]
pub type W = crate::W<Kbsbuf017Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan Buffer Data 0-17 Register (KBSBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsbuf017::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsbuf017::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kbsbuf017Spec;
impl crate::RegisterSpec for Kbsbuf017Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbsbuf017::R`](R) reader structure"]
impl crate::Readable for Kbsbuf017Spec {}
#[doc = "`write(|w| ..)` method takes [`kbsbuf017::W`](W) writer structure"]
impl crate::Writable for Kbsbuf017Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBSBUF0-17 to value 0"]
impl crate::Resettable for Kbsbuf017Spec {
    const RESET_VALUE: u8 = 0;
}
