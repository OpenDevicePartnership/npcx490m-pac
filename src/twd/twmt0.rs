#[doc = "Register `TWMT0` reader"]
pub type R = crate::R<Twmt0Spec>;
#[doc = "Register `TWMT0` writer"]
pub type W = crate::W<Twmt0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TWD Timer 0 Counter Register (TWMT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`twmt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twmt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Twmt0Spec;
impl crate::RegisterSpec for Twmt0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`twmt0::R`](R) reader structure"]
impl crate::Readable for Twmt0Spec {}
#[doc = "`write(|w| ..)` method takes [`twmt0::W`](W) writer structure"]
impl crate::Writable for Twmt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TWMT0 to value 0"]
impl crate::Resettable for Twmt0Spec {
    const RESET_VALUE: u16 = 0;
}
