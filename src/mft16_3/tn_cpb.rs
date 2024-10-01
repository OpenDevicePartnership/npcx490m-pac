#[doc = "Register `TnCPB` reader"]
pub type R = crate::R<TnCpbSpec>;
#[doc = "Register `TnCPB` writer"]
pub type W = crate::W<TnCpbSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Compare B Register (TnCPB)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cpb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cpb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCpbSpec;
impl crate::RegisterSpec for TnCpbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tn_cpb::R`](R) reader structure"]
impl crate::Readable for TnCpbSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_cpb::W`](W) writer structure"]
impl crate::Writable for TnCpbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TnCPB to value 0"]
impl crate::Resettable for TnCpbSpec {
    const RESET_VALUE: u16 = 0;
}
