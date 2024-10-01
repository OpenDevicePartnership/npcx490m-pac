#[doc = "Register `IBUF98` reader"]
pub type R = crate::R<Ibuf98Spec>;
#[doc = "Register `IBUF98` writer"]
pub type W = crate::W<Ibuf98Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf98Spec;
impl crate::RegisterSpec for Ibuf98Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf98::R`](R) reader structure"]
impl crate::Readable for Ibuf98Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf98::W`](W) writer structure"]
impl crate::Writable for Ibuf98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF98 to value 0"]
impl crate::Resettable for Ibuf98Spec {
    const RESET_VALUE: u8 = 0;
}
