#[doc = "Register `ITCNT64L` reader"]
pub type R = crate::R<Itcnt64lSpec>;
#[doc = "Register `ITCNT64L` writer"]
pub type W = crate::W<Itcnt64lSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal 64-Bit Timer Counter Register, Low DWord (ITCNT64L)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcnt64l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcnt64l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itcnt64lSpec;
impl crate::RegisterSpec for Itcnt64lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itcnt64l::R`](R) reader structure"]
impl crate::Readable for Itcnt64lSpec {}
#[doc = "`write(|w| ..)` method takes [`itcnt64l::W`](W) writer structure"]
impl crate::Writable for Itcnt64lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITCNT64L to value 0"]
impl crate::Resettable for Itcnt64lSpec {
    const RESET_VALUE: u32 = 0;
}
