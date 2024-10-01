#[doc = "Register `MDMAn_DSTB1` reader"]
pub type R = crate::R<MdmanDstb1Spec>;
#[doc = "Register `MDMAn_DSTB1` writer"]
pub type W = crate::W<MdmanDstb1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 1 Destination Base Address Register (MDMAn_DSTB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdman_dstb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdman_dstb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmanDstb1Spec;
impl crate::RegisterSpec for MdmanDstb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdman_dstb1::R`](R) reader structure"]
impl crate::Readable for MdmanDstb1Spec {}
#[doc = "`write(|w| ..)` method takes [`mdman_dstb1::W`](W) writer structure"]
impl crate::Writable for MdmanDstb1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMAn_DSTB1 to value 0"]
impl crate::Resettable for MdmanDstb1Spec {
    const RESET_VALUE: u32 = 0;
}
