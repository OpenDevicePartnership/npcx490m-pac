#[doc = "Register `TnPRSC` reader"]
pub type R = crate::R<TnPrscSpec>;
#[doc = "Register `TnPRSC` writer"]
pub type W = crate::W<TnPrscSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock Prescaler Register (TnPRSC)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_prsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_prsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnPrscSpec;
impl crate::RegisterSpec for TnPrscSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_prsc::R`](R) reader structure"]
impl crate::Readable for TnPrscSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_prsc::W`](W) writer structure"]
impl crate::Writable for TnPrscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnPRSC to value 0"]
impl crate::Resettable for TnPrscSpec {
    const RESET_VALUE: u8 = 0;
}
