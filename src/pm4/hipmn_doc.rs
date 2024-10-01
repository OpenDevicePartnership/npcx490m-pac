#[doc = "Register `HIPMnDOC` reader"]
pub type R = crate::R<HipmnDocSpec>;
#[doc = "Register `HIPMnDOC` writer"]
pub type W = crate::W<HipmnDocSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface PM n Data Out Buffer with SCI Register (HIPMnDOC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_doc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_doc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnDocSpec;
impl crate::RegisterSpec for HipmnDocSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_doc::R`](R) reader structure"]
impl crate::Readable for HipmnDocSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_doc::W`](W) writer structure"]
impl crate::Writable for HipmnDocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnDOC to value 0"]
impl crate::Resettable for HipmnDocSpec {
    const RESET_VALUE: u8 = 0;
}
