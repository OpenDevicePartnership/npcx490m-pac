#[doc = "Register `UMA_AB0-21` reader"]
pub type R = crate::R<UmaAb021Spec>;
#[doc = "Register `UMA_AB0-21` writer"]
pub type W = crate::W<UmaAb021Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UMA Address Byte 0-2 Register (UMA_AB0-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ab021::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ab021::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaAb021Spec;
impl crate::RegisterSpec for UmaAb021Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uma_ab021::R`](R) reader structure"]
impl crate::Readable for UmaAb021Spec {}
#[doc = "`write(|w| ..)` method takes [`uma_ab021::W`](W) writer structure"]
impl crate::Writable for UmaAb021Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UMA_AB0-21 to value 0"]
impl crate::Resettable for UmaAb021Spec {
    const RESET_VALUE: u8 = 0;
}
