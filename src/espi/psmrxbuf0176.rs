#[doc = "Register `PSMRXBUF0-176` reader"]
pub type R = crate::R<Psmrxbuf0176Spec>;
#[doc = "Register `PSMRXBUF0-176` writer"]
pub type W = crate::W<Psmrxbuf0176Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0176::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0176::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Psmrxbuf0176Spec;
impl crate::RegisterSpec for Psmrxbuf0176Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmrxbuf0176::R`](R) reader structure"]
impl crate::Readable for Psmrxbuf0176Spec {}
#[doc = "`write(|w| ..)` method takes [`psmrxbuf0176::W`](W) writer structure"]
impl crate::Writable for Psmrxbuf0176Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSMRXBUF0-176 to value 0"]
impl crate::Resettable for Psmrxbuf0176Spec {
    const RESET_VALUE: u32 = 0;
}
