#[doc = "Register `DCRn` reader"]
pub type R = crate::R<DcrnSpec>;
#[doc = "Register `DCRn` writer"]
pub type W = crate::W<DcrnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Duty Cycle Register (DCRn)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrnSpec;
impl crate::RegisterSpec for DcrnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dcrn::R`](R) reader structure"]
impl crate::Readable for DcrnSpec {}
#[doc = "`write(|w| ..)` method takes [`dcrn::W`](W) writer structure"]
impl crate::Writable for DcrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DCRn to value 0"]
impl crate::Resettable for DcrnSpec {
    const RESET_VALUE: u16 = 0;
}
