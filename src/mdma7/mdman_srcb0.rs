#[doc = "Register `MDMAn_SRCB0` reader"]
pub type R = crate::R<MdmanSrcb0Spec>;
#[doc = "Register `MDMAn_SRCB0` writer"]
pub type W = crate::W<MdmanSrcb0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0 Source Base Address Register (MDMAn_SRCB0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_srcb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_srcb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmanSrcb0Spec;
impl crate::RegisterSpec for MdmanSrcb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdman_srcb0::R`](R) reader structure"]
impl crate::Readable for MdmanSrcb0Spec {}
#[doc = "`write(|w| ..)` method takes [`mdman_srcb0::W`](W) writer structure"]
impl crate::Writable for MdmanSrcb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMAn_SRCB0 to value 0"]
impl crate::Resettable for MdmanSrcb0Spec {
    const RESET_VALUE: u32 = 0;
}
