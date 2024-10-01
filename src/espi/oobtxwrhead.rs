#[doc = "Register `OOBTXWRHEAD` reader"]
pub type R = crate::R<OobtxwrheadSpec>;
#[doc = "Register `OOBTXWRHEAD` writer"]
pub type W = crate::W<OobtxwrheadSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OOB Transmit Buffer Write Head Register (OOBTXWRHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxwrhead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxwrhead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OobtxwrheadSpec;
impl crate::RegisterSpec for OobtxwrheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oobtxwrhead::R`](R) reader structure"]
impl crate::Readable for OobtxwrheadSpec {}
#[doc = "`write(|w| ..)` method takes [`oobtxwrhead::W`](W) writer structure"]
impl crate::Writable for OobtxwrheadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOBTXWRHEAD to value 0"]
impl crate::Resettable for OobtxwrheadSpec {
    const RESET_VALUE: u32 = 0;
}
