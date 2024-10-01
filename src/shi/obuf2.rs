#[doc = "Register `OBUF2` reader"]
pub type R = crate::R<Obuf2Spec>;
#[doc = "Register `OBUF2` writer"]
pub type W = crate::W<Obuf2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf2Spec;
impl crate::RegisterSpec for Obuf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf2::R`](R) reader structure"]
impl crate::Readable for Obuf2Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf2::W`](W) writer structure"]
impl crate::Writable for Obuf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF2 to value 0"]
impl crate::Resettable for Obuf2Spec {
    const RESET_VALUE: u8 = 0;
}
