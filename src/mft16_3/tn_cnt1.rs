#[doc = "Register `TnCNT1` reader"]
pub type R = crate::R<TnCnt1Spec>;
#[doc = "Register `TnCNT1` writer"]
pub type W = crate::W<TnCnt1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer/Counter 1 Register (TnCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCnt1Spec;
impl crate::RegisterSpec for TnCnt1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tn_cnt1::R`](R) reader structure"]
impl crate::Readable for TnCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`tn_cnt1::W`](W) writer structure"]
impl crate::Writable for TnCnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TnCNT1 to value 0"]
impl crate::Resettable for TnCnt1Spec {
    const RESET_VALUE: u16 = 0;
}
