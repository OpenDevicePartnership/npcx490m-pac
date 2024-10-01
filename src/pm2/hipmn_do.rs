#[doc = "Register `HIPMnDO` reader"]
pub type R = crate::R<HipmnDoSpec>;
#[doc = "Register `HIPMnDO` writer"]
pub type W = crate::W<HipmnDoSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface PM n Data Out Buffer Register (HIPMnDO)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_do::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_do::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnDoSpec;
impl crate::RegisterSpec for HipmnDoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_do::R`](R) reader structure"]
impl crate::Readable for HipmnDoSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_do::W`](W) writer structure"]
impl crate::Writable for HipmnDoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnDO to value 0"]
impl crate::Resettable for HipmnDoSpec {
    const RESET_VALUE: u8 = 0;
}
