#[doc = "Register `KBSOUT0` reader"]
pub type R = crate::R<Kbsout0Spec>;
#[doc = "Register `KBSOUT0` writer"]
pub type W = crate::W<Kbsout0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan Out 0 Register (KBSOUT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsout0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsout0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kbsout0Spec;
impl crate::RegisterSpec for Kbsout0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`kbsout0::R`](R) reader structure"]
impl crate::Readable for Kbsout0Spec {}
#[doc = "`write(|w| ..)` method takes [`kbsout0::W`](W) writer structure"]
impl crate::Writable for Kbsout0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets KBSOUT0 to value 0"]
impl crate::Resettable for Kbsout0Spec {
    const RESET_VALUE: u16 = 0;
}
