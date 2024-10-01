#[doc = "Register `IBUF56` reader"]
pub type R = crate::R<Ibuf56Spec>;
#[doc = "Register `IBUF56` writer"]
pub type W = crate::W<Ibuf56Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf56Spec;
impl crate::RegisterSpec for Ibuf56Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf56::R`](R) reader structure"]
impl crate::Readable for Ibuf56Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf56::W`](W) writer structure"]
impl crate::Writable for Ibuf56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF56 to value 0"]
impl crate::Resettable for Ibuf56Spec {
    const RESET_VALUE: u8 = 0;
}
