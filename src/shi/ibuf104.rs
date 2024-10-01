#[doc = "Register `IBUF104` reader"]
pub type R = crate::R<Ibuf104Spec>;
#[doc = "Register `IBUF104` writer"]
pub type W = crate::W<Ibuf104Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf104Spec;
impl crate::RegisterSpec for Ibuf104Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf104::R`](R) reader structure"]
impl crate::Readable for Ibuf104Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf104::W`](W) writer structure"]
impl crate::Writable for Ibuf104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF104 to value 0"]
impl crate::Resettable for Ibuf104Spec {
    const RESET_VALUE: u8 = 0;
}
