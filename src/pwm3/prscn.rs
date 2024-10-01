#[doc = "Register `PRSCn` reader"]
pub type R = crate::R<PrscnSpec>;
#[doc = "Register `PRSCn` writer"]
pub type W = crate::W<PrscnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock Prescaler Register (PRSCn)\n\nYou can [`read`](crate::Reg::read) this register and get [`prscn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prscn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrscnSpec;
impl crate::RegisterSpec for PrscnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`prscn::R`](R) reader structure"]
impl crate::Readable for PrscnSpec {}
#[doc = "`write(|w| ..)` method takes [`prscn::W`](W) writer structure"]
impl crate::Writable for PrscnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PRSCn to value 0"]
impl crate::Resettable for PrscnSpec {
    const RESET_VALUE: u16 = 0;
}
