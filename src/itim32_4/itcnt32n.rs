#[doc = "Register `ITCNT32n` reader"]
pub type R = crate::R<Itcnt32nSpec>;
#[doc = "Register `ITCNT32n` writer"]
pub type W = crate::W<Itcnt32nSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal 32-Bit Timer Counter Register (ITCNT32n)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcnt32n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcnt32n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itcnt32nSpec;
impl crate::RegisterSpec for Itcnt32nSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itcnt32n::R`](R) reader structure"]
impl crate::Readable for Itcnt32nSpec {}
#[doc = "`write(|w| ..)` method takes [`itcnt32n::W`](W) writer structure"]
impl crate::Writable for Itcnt32nSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITCNT32n to value 0"]
impl crate::Resettable for Itcnt32nSpec {
    const RESET_VALUE: u32 = 0;
}
