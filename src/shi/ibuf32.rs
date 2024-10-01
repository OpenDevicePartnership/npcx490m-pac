#[doc = "Register `IBUF32` reader"]
pub type R = crate::R<Ibuf32Spec>;
#[doc = "Register `IBUF32` writer"]
pub type W = crate::W<Ibuf32Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf32Spec;
impl crate::RegisterSpec for Ibuf32Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf32::R`](R) reader structure"]
impl crate::Readable for Ibuf32Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf32::W`](W) writer structure"]
impl crate::Writable for Ibuf32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF32 to value 0"]
impl crate::Resettable for Ibuf32Spec {
    const RESET_VALUE: u8 = 0;
}
