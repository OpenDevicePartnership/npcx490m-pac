#[doc = "Register `OBUF77` reader"]
pub type R = crate::R<Obuf77Spec>;
#[doc = "Register `OBUF77` writer"]
pub type W = crate::W<Obuf77Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf77::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf77::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf77Spec;
impl crate::RegisterSpec for Obuf77Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf77::R`](R) reader structure"]
impl crate::Readable for Obuf77Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf77::W`](W) writer structure"]
impl crate::Writable for Obuf77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF77 to value 0"]
impl crate::Resettable for Obuf77Spec {
    const RESET_VALUE: u8 = 0;
}
