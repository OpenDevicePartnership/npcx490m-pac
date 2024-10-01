#[doc = "Register `OBUF61` reader"]
pub type R = crate::R<Obuf61Spec>;
#[doc = "Register `OBUF61` writer"]
pub type W = crate::W<Obuf61Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf61Spec;
impl crate::RegisterSpec for Obuf61Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf61::R`](R) reader structure"]
impl crate::Readable for Obuf61Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf61::W`](W) writer structure"]
impl crate::Writable for Obuf61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF61 to value 0"]
impl crate::Resettable for Obuf61Spec {
    const RESET_VALUE: u8 = 0;
}
