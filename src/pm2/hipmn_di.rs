#[doc = "Register `HIPMnDI` reader"]
pub type R = crate::R<HipmnDiSpec>;
#[doc = "Register `HIPMnDI` writer"]
pub type W = crate::W<HipmnDiSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface PM n Data In Buffer Register (HIPMnDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_di::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_di::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnDiSpec;
impl crate::RegisterSpec for HipmnDiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_di::R`](R) reader structure"]
impl crate::Readable for HipmnDiSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_di::W`](W) writer structure"]
impl crate::Writable for HipmnDiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnDI to value 0"]
impl crate::Resettable for HipmnDiSpec {
    const RESET_VALUE: u8 = 0;
}
