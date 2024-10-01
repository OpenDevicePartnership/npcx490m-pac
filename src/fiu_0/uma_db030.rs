#[doc = "Register `UMA_DB0-30` reader"]
pub type R = crate::R<UmaDb030Spec>;
#[doc = "Register `UMA_DB0-30` writer"]
pub type W = crate::W<UmaDb030Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UMA Data Byte 0-3 Register (UMA_DB0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_db030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_db030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaDb030Spec;
impl crate::RegisterSpec for UmaDb030Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uma_db030::R`](R) reader structure"]
impl crate::Readable for UmaDb030Spec {}
#[doc = "`write(|w| ..)` method takes [`uma_db030::W`](W) writer structure"]
impl crate::Writable for UmaDb030Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UMA_DB0-30 to value 0"]
impl crate::Resettable for UmaDb030Spec {
    const RESET_VALUE: u8 = 0;
}
