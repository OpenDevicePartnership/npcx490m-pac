#[doc = "Register `IBUF17` reader"]
pub type R = crate::R<Ibuf17Spec>;
#[doc = "Register `IBUF17` writer"]
pub type W = crate::W<Ibuf17Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf17Spec;
impl crate::RegisterSpec for Ibuf17Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf17::R`](R) reader structure"]
impl crate::Readable for Ibuf17Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf17::W`](W) writer structure"]
impl crate::Writable for Ibuf17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF17 to value 0"]
impl crate::Resettable for Ibuf17Spec {
    const RESET_VALUE: u8 = 0;
}
