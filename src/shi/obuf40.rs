#[doc = "Register `OBUF40` reader"]
pub type R = crate::R<Obuf40Spec>;
#[doc = "Register `OBUF40` writer"]
pub type W = crate::W<Obuf40Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf40Spec;
impl crate::RegisterSpec for Obuf40Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf40::R`](R) reader structure"]
impl crate::Readable for Obuf40Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf40::W`](W) writer structure"]
impl crate::Writable for Obuf40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF40 to value 0"]
impl crate::Resettable for Obuf40Spec {
    const RESET_VALUE: u8 = 0;
}
