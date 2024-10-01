#[doc = "Register `OBUF15` reader"]
pub type R = crate::R<Obuf15Spec>;
#[doc = "Register `OBUF15` writer"]
pub type W = crate::W<Obuf15Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf15Spec;
impl crate::RegisterSpec for Obuf15Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf15::R`](R) reader structure"]
impl crate::Readable for Obuf15Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf15::W`](W) writer structure"]
impl crate::Writable for Obuf15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF15 to value 0"]
impl crate::Resettable for Obuf15Spec {
    const RESET_VALUE: u8 = 0;
}
