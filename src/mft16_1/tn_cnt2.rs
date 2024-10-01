#[doc = "Register `TnCNT2` reader"]
pub type R = crate::R<TnCnt2Spec>;
#[doc = "Register `TnCNT2` writer"]
pub type W = crate::W<TnCnt2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer/Counter 2 Register (TnCNT2)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCnt2Spec;
impl crate::RegisterSpec for TnCnt2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tn_cnt2::R`](R) reader structure"]
impl crate::Readable for TnCnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`tn_cnt2::W`](W) writer structure"]
impl crate::Writable for TnCnt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TnCNT2 to value 0"]
impl crate::Resettable for TnCnt2Spec {
    const RESET_VALUE: u16 = 0;
}
