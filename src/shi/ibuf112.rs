#[doc = "Register `IBUF112` reader"]
pub type R = crate::R<Ibuf112Spec>;
#[doc = "Register `IBUF112` writer"]
pub type W = crate::W<Ibuf112Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf112::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf112::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf112Spec;
impl crate::RegisterSpec for Ibuf112Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf112::R`](R) reader structure"]
impl crate::Readable for Ibuf112Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf112::W`](W) writer structure"]
impl crate::Writable for Ibuf112Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF112 to value 0"]
impl crate::Resettable for Ibuf112Spec {
    const RESET_VALUE: u8 = 0;
}
