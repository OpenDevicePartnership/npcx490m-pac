#[doc = "Register `UMA_CODE` reader"]
pub type R = crate::R<UmaCodeSpec>;
#[doc = "Register `UMA_CODE` writer"]
pub type W = crate::W<UmaCodeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UMA Code Byte Register (UMA_CODE)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_code::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_code::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaCodeSpec;
impl crate::RegisterSpec for UmaCodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uma_code::R`](R) reader structure"]
impl crate::Readable for UmaCodeSpec {}
#[doc = "`write(|w| ..)` method takes [`uma_code::W`](W) writer structure"]
impl crate::Writable for UmaCodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UMA_CODE to value 0"]
impl crate::Resettable for UmaCodeSpec {
    const RESET_VALUE: u8 = 0;
}
