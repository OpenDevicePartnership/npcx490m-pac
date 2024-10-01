#[doc = "Register `OBUF31` reader"]
pub type R = crate::R<Obuf31Spec>;
#[doc = "Register `OBUF31` writer"]
pub type W = crate::W<Obuf31Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf31Spec;
impl crate::RegisterSpec for Obuf31Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf31::R`](R) reader structure"]
impl crate::Readable for Obuf31Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf31::W`](W) writer structure"]
impl crate::Writable for Obuf31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF31 to value 0"]
impl crate::Resettable for Obuf31Spec {
    const RESET_VALUE: u8 = 0;
}
