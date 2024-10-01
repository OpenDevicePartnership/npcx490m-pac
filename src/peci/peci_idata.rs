#[doc = "Register `PECI_IDATA` reader"]
pub type R = crate::R<PeciIdataSpec>;
#[doc = "Register `PECI_IDATA` writer"]
pub type W = crate::W<PeciIdataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Index Data Registers (PECI_IDATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_idata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_idata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciIdataSpec;
impl crate::RegisterSpec for PeciIdataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_idata::R`](R) reader structure"]
impl crate::Readable for PeciIdataSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_idata::W`](W) writer structure"]
impl crate::Writable for PeciIdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_IDATA to value 0"]
impl crate::Resettable for PeciIdataSpec {
    const RESET_VALUE: u8 = 0;
}
