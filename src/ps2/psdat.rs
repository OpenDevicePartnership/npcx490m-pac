#[doc = "Register `PSDAT` reader"]
pub type R = crate::R<PsdatSpec>;
#[doc = "Register `PSDAT` writer"]
pub type W = crate::W<PsdatSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PS/2 Data Register (PSDAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`psdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsdatSpec;
impl crate::RegisterSpec for PsdatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psdat::R`](R) reader structure"]
impl crate::Readable for PsdatSpec {}
#[doc = "`write(|w| ..)` method takes [`psdat::W`](W) writer structure"]
impl crate::Writable for PsdatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSDAT to value 0"]
impl crate::Resettable for PsdatSpec {
    const RESET_VALUE: u8 = 0;
}
