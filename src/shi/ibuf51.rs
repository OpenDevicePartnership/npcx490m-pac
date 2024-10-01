#[doc = "Register `IBUF51` reader"]
pub type R = crate::R<Ibuf51Spec>;
#[doc = "Register `IBUF51` writer"]
pub type W = crate::W<Ibuf51Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf51Spec;
impl crate::RegisterSpec for Ibuf51Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf51::R`](R) reader structure"]
impl crate::Readable for Ibuf51Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf51::W`](W) writer structure"]
impl crate::Writable for Ibuf51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF51 to value 0"]
impl crate::Resettable for Ibuf51Spec {
    const RESET_VALUE: u8 = 0;
}
