#[doc = "Register `OBUF49` reader"]
pub type R = crate::R<Obuf49Spec>;
#[doc = "Register `OBUF49` writer"]
pub type W = crate::W<Obuf49Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf49Spec;
impl crate::RegisterSpec for Obuf49Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf49::R`](R) reader structure"]
impl crate::Readable for Obuf49Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf49::W`](W) writer structure"]
impl crate::Writable for Obuf49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF49 to value 0"]
impl crate::Resettable for Obuf49Spec {
    const RESET_VALUE: u8 = 0;
}
