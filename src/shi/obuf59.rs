#[doc = "Register `OBUF59` reader"]
pub type R = crate::R<Obuf59Spec>;
#[doc = "Register `OBUF59` writer"]
pub type W = crate::W<Obuf59Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf59::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf59::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf59Spec;
impl crate::RegisterSpec for Obuf59Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf59::R`](R) reader structure"]
impl crate::Readable for Obuf59Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf59::W`](W) writer structure"]
impl crate::Writable for Obuf59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF59 to value 0"]
impl crate::Resettable for Obuf59Spec {
    const RESET_VALUE: u8 = 0;
}
