#[doc = "Register `ITCNT64H` reader"]
pub type R = crate::R<Itcnt64hSpec>;
#[doc = "Register `ITCNT64H` writer"]
pub type W = crate::W<Itcnt64hSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal 64-Bit Timer Counter Register, High DWord (ITCNT64H)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcnt64h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcnt64h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itcnt64hSpec;
impl crate::RegisterSpec for Itcnt64hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itcnt64h::R`](R) reader structure"]
impl crate::Readable for Itcnt64hSpec {}
#[doc = "`write(|w| ..)` method takes [`itcnt64h::W`](W) writer structure"]
impl crate::Writable for Itcnt64hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITCNT64H to value 0"]
impl crate::Resettable for Itcnt64hSpec {
    const RESET_VALUE: u32 = 0;
}
