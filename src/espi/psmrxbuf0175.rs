#[doc = "Register `PSMRXBUF0-175` reader"]
pub type R = crate::R<Psmrxbuf0175Spec>;
#[doc = "Register `PSMRXBUF0-175` writer"]
pub type W = crate::W<Psmrxbuf0175Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0175::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0175::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Psmrxbuf0175Spec;
impl crate::RegisterSpec for Psmrxbuf0175Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmrxbuf0175::R`](R) reader structure"]
impl crate::Readable for Psmrxbuf0175Spec {}
#[doc = "`write(|w| ..)` method takes [`psmrxbuf0175::W`](W) writer structure"]
impl crate::Writable for Psmrxbuf0175Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSMRXBUF0-175 to value 0"]
impl crate::Resettable for Psmrxbuf0175Spec {
    const RESET_VALUE: u32 = 0;
}
