#[doc = "Register `HFCGMH` reader"]
pub type R = crate::R<HfcgmhSpec>;
#[doc = "Register `HFCGMH` writer"]
pub type W = crate::W<HfcgmhSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "HFCG M High Value Register (HFCGMH)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgmh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgmh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcgmhSpec;
impl crate::RegisterSpec for HfcgmhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcgmh::R`](R) reader structure"]
impl crate::Readable for HfcgmhSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcgmh::W`](W) writer structure"]
impl crate::Writable for HfcgmhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCGMH to value 0"]
impl crate::Resettable for HfcgmhSpec {
    const RESET_VALUE: u8 = 0;
}
