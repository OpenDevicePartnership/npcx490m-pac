#[doc = "Register `OOBRXBUF%s` reader"]
pub type R = crate::R<OobrxbufSpec>;
#[doc = "Register `OOBRXBUF%s` writer"]
pub type W = crate::W<OobrxbufSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OobrxbufSpec;
impl crate::RegisterSpec for OobrxbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oobrxbuf::R`](R) reader structure"]
impl crate::Readable for OobrxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`oobrxbuf::W`](W) writer structure"]
impl crate::Writable for OobrxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OOBRXBUF%s to value 0"]
impl crate::Resettable for OobrxbufSpec {
    const RESET_VALUE: u32 = 0;
}
