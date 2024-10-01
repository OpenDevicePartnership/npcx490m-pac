#[doc = "Register `HIMDO` reader"]
pub type R = crate::R<HimdoSpec>;
#[doc = "Register `HIMDO` writer"]
pub type W = crate::W<HimdoSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface Mouse Data Out Buffer Register (HIMDO)\n\nYou can [`read`](crate::Reg::read) this register and get [`himdo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`himdo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HimdoSpec;
impl crate::RegisterSpec for HimdoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`himdo::R`](R) reader structure"]
impl crate::Readable for HimdoSpec {}
#[doc = "`write(|w| ..)` method takes [`himdo::W`](W) writer structure"]
impl crate::Writable for HimdoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIMDO to value 0"]
impl crate::Resettable for HimdoSpec {
    const RESET_VALUE: u8 = 0;
}
