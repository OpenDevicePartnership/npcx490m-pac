#[doc = "Register `SID_CR` reader"]
pub type R = crate::R<SidCrSpec>;
#[doc = "Register `SID_CR` writer"]
pub type W = crate::W<SidCrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SID Core Access Register (SID_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`sid_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sid_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidCrSpec;
impl crate::RegisterSpec for SidCrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sid_cr::R`](R) reader structure"]
impl crate::Readable for SidCrSpec {}
#[doc = "`write(|w| ..)` method takes [`sid_cr::W`](W) writer structure"]
impl crate::Writable for SidCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SID_CR to value 0"]
impl crate::Resettable for SidCrSpec {
    const RESET_VALUE: u8 = 0;
}
