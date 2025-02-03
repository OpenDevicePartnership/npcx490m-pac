#[doc = "Register `OOBTXBUF%s` reader"]
pub type R = crate::R<OobtxbufSpec>;
#[doc = "Register `OOBTXBUF%s` writer"]
pub type W = crate::W<OobtxbufSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OobtxbufSpec;
impl crate::RegisterSpec for OobtxbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oobtxbuf::R`](R) reader structure"]
impl crate::Readable for OobtxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`oobtxbuf::W`](W) writer structure"]
impl crate::Writable for OobtxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOBTXBUF%s to value 0"]
impl crate::Resettable for OobtxbufSpec {
    const RESET_VALUE: u32 = 0;
}
