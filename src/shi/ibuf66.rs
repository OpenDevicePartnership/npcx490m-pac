#[doc = "Register `IBUF66` reader"]
pub type R = crate::R<Ibuf66Spec>;
#[doc = "Register `IBUF66` writer"]
pub type W = crate::W<Ibuf66Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibuf66::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibuf66::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ibuf66Spec;
impl crate::RegisterSpec for Ibuf66Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibuf66::R`](R) reader structure"]
impl crate::Readable for Ibuf66Spec {}
#[doc = "`write(|w| ..)` method takes [`ibuf66::W`](W) writer structure"]
impl crate::Writable for Ibuf66Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUF66 to value 0"]
impl crate::Resettable for Ibuf66Spec {
    const RESET_VALUE: u8 = 0;
}
