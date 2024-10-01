#[doc = "Register `PECI_WR_FCS` reader"]
pub type R = crate::R<PeciWrFcsSpec>;
#[doc = "Register `PECI_WR_FCS` writer"]
pub type W = crate::W<PeciWrFcsSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Write FCS Register (PECI_WR_FCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_wr_fcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_wr_fcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciWrFcsSpec;
impl crate::RegisterSpec for PeciWrFcsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_wr_fcs::R`](R) reader structure"]
impl crate::Readable for PeciWrFcsSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_wr_fcs::W`](W) writer structure"]
impl crate::Writable for PeciWrFcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_WR_FCS to value 0"]
impl crate::Resettable for PeciWrFcsSpec {
    const RESET_VALUE: u8 = 0;
}
