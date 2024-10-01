#[doc = "Register `PECI_ADDR` reader"]
pub type R = crate::R<PeciAddrSpec>;
#[doc = "Register `PECI_ADDR` writer"]
pub type W = crate::W<PeciAddrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PECI Address Register (PECI_ADDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciAddrSpec;
impl crate::RegisterSpec for PeciAddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_addr::R`](R) reader structure"]
impl crate::Readable for PeciAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_addr::W`](W) writer structure"]
impl crate::Writable for PeciAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_ADDR to value 0"]
impl crate::Resettable for PeciAddrSpec {
    const RESET_VALUE: u8 = 0;
}
