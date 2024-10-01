#[doc = "Register `HIPMnDIC` reader"]
pub type R = crate::R<HipmnDicSpec>;
#[doc = "Register `HIPMnDIC` writer"]
pub type W = crate::W<HipmnDicSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface PM n Data In Buffer with SCI Register (HIPMnDIC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_dic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_dic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HipmnDicSpec;
impl crate::RegisterSpec for HipmnDicSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hipmn_dic::R`](R) reader structure"]
impl crate::Readable for HipmnDicSpec {}
#[doc = "`write(|w| ..)` method takes [`hipmn_dic::W`](W) writer structure"]
impl crate::Writable for HipmnDicSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIPMnDIC to value 0"]
impl crate::Resettable for HipmnDicSpec {
    const RESET_VALUE: u8 = 0;
}
