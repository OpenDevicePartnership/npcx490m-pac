#[doc = "Register `LCTSECOND` reader"]
pub type R = crate::R<LctsecondSpec>;
#[doc = "Register `LCTSECOND` writer"]
pub type W = crate::W<LctsecondSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LCT Seconds Register (LCTSECOND)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctsecond::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctsecond::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctsecondSpec;
impl crate::RegisterSpec for LctsecondSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lctsecond::R`](R) reader structure"]
impl crate::Readable for LctsecondSpec {}
#[doc = "`write(|w| ..)` method takes [`lctsecond::W`](W) writer structure"]
impl crate::Writable for LctsecondSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTSECOND to value 0"]
impl crate::Resettable for LctsecondSpec {
    const RESET_VALUE: u8 = 0;
}
