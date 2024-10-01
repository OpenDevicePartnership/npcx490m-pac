#[doc = "Register `CHPREV_CR` reader"]
pub type R = crate::R<ChprevCrSpec>;
#[doc = "Register `CHPREV_CR` writer"]
pub type W = crate::W<ChprevCrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Chip Revision Core Access Register (CHPREV_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`chprev_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chprev_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChprevCrSpec;
impl crate::RegisterSpec for ChprevCrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chprev_cr::R`](R) reader structure"]
impl crate::Readable for ChprevCrSpec {}
#[doc = "`write(|w| ..)` method takes [`chprev_cr::W`](W) writer structure"]
impl crate::Writable for ChprevCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHPREV_CR to value 0"]
impl crate::Resettable for ChprevCrSpec {
    const RESET_VALUE: u8 = 0;
}
