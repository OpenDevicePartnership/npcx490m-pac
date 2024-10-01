#[doc = "Register `PxOTYPE` reader"]
pub type R = crate::R<PxOtypeSpec>;
#[doc = "Register `PxOTYPE` writer"]
pub type W = crate::W<PxOtypeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port GPIOx Output Type Register (PxOTYPE)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_otype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_otype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxOtypeSpec;
impl crate::RegisterSpec for PxOtypeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_otype::R`](R) reader structure"]
impl crate::Readable for PxOtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`px_otype::W`](W) writer structure"]
impl crate::Writable for PxOtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxOTYPE to value 0"]
impl crate::Resettable for PxOtypeSpec {
    const RESET_VALUE: u8 = 0;
}
