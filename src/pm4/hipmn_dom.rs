#[doc = "Register `HIPMnDOM` reader"]
pub type R = crate::R<HipmnDomSpec>;
#[doc = "Register `HIPMnDOM` writer"]
pub type W = crate::W<HipmnDomSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface PM n Data Out Buffer with SMI Register (HIPMnDOM)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_dom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_dom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnDomSpec;
impl crate::RegisterSpec for HipmnDomSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_dom::R`](R) reader structure"]
impl crate::Readable for HipmnDomSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_dom::W`](W) writer structure"]
impl crate::Writable for HipmnDomSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnDOM to value 0"]
impl crate::Resettable for HipmnDomSpec {
    const RESET_VALUE: u8 = 0;
}
