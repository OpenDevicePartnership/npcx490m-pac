#[doc = "Register `TnCPA` reader"]
pub type R = crate::R<TnCpaSpec>;
#[doc = "Register `TnCPA` writer"]
pub type W = crate::W<TnCpaSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Compare A Register (TnCPA)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cpa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cpa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCpaSpec;
impl crate::RegisterSpec for TnCpaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tn_cpa::R`](R) reader structure"]
impl crate::Readable for TnCpaSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_cpa::W`](W) writer structure"]
impl crate::Writable for TnCpaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TnCPA to value 0"]
impl crate::Resettable for TnCpaSpec {
    const RESET_VALUE: u16 = 0;
}
