#[doc = "Register `CSRC` reader"]
pub type R = crate::R<CsrcSpec>;
#[doc = "Register `CSRC` writer"]
pub type W = crate::W<CsrcSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`csrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrcSpec;
impl crate::RegisterSpec for CsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csrc::R`](R) reader structure"]
impl crate::Readable for CsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`csrc::W`](W) writer structure"]
impl crate::Writable for CsrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSRC to value 0"]
impl crate::Resettable for CsrcSpec {
    const RESET_VALUE: u32 = 0;
}
