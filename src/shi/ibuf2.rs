#[doc = "Register `IBUF2` reader"]
pub type R = crate::R<Ibuf2Spec>;
#[doc = "Register `IBUF2` writer"]
pub type W = crate::W<Ibuf2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf2Spec;
impl crate::RegisterSpec for Ibuf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf2::R`](R) reader structure"]
impl crate::Readable for Ibuf2Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf2::W`](W) writer structure"]
impl crate::Writable for Ibuf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF2 to value 0"]
impl crate::Resettable for Ibuf2Spec {
    const RESET_VALUE: u8 = 0;
}
