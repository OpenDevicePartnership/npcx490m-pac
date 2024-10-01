#[doc = "Register `SRID_CR` reader"]
pub type R = crate::R<SridCrSpec>;
#[doc = "Register `SRID_CR` writer"]
pub type W = crate::W<SridCrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SRID Core Access Register (SRID_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`srid_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srid_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SridCrSpec;
impl crate::RegisterSpec for SridCrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`srid_cr::R`](R) reader structure"]
impl crate::Readable for SridCrSpec {}
#[doc = "`write(|w| ..)` method takes [`srid_cr::W`](W) writer structure"]
impl crate::Writable for SridCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SRID_CR to value 0"]
impl crate::Resettable for SridCrSpec {
    const RESET_VALUE: u8 = 0;
}
