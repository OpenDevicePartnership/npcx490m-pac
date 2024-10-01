#[doc = "Register `OBUF60` reader"]
pub type R = crate::R<Obuf60Spec>;
#[doc = "Register `OBUF60` writer"]
pub type W = crate::W<Obuf60Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf60Spec;
impl crate::RegisterSpec for Obuf60Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf60::R`](R) reader structure"]
impl crate::Readable for Obuf60Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf60::W`](W) writer structure"]
impl crate::Writable for Obuf60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF60 to value 0"]
impl crate::Resettable for Obuf60Spec {
    const RESET_VALUE: u8 = 0;
}
