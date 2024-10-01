#[doc = "Register `IBUF102` reader"]
pub type R = crate::R<Ibuf102Spec>;
#[doc = "Register `IBUF102` writer"]
pub type W = crate::W<Ibuf102Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf102::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf102::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf102Spec;
impl crate::RegisterSpec for Ibuf102Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf102::R`](R) reader structure"]
impl crate::Readable for Ibuf102Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf102::W`](W) writer structure"]
impl crate::Writable for Ibuf102Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF102 to value 0"]
impl crate::Resettable for Ibuf102Spec {
    const RESET_VALUE: u8 = 0;
}
