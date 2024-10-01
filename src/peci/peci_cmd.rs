#[doc = "Register `PECI_CMD` reader"]
pub type R = crate::R<PeciCmdSpec>;
#[doc = "Register `PECI_CMD` writer"]
pub type W = crate::W<PeciCmdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Command Register (PECI_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciCmdSpec;
impl crate::RegisterSpec for PeciCmdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_cmd::R`](R) reader structure"]
impl crate::Readable for PeciCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_cmd::W`](W) writer structure"]
impl crate::Writable for PeciCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_CMD to value 0"]
impl crate::Resettable for PeciCmdSpec {
    const RESET_VALUE: u8 = 0;
}
