#[doc = "Register `LCTDAY` reader"]
pub type R = crate::R<LctdaySpec>;
#[doc = "Register `LCTDAY` writer"]
pub type W = crate::W<LctdaySpec>;
#[doc = "Field `DAYS` reader - Days Value"]
pub type DaysR = crate::FieldReader;
#[doc = "Field `DAYS` writer - Days Value"]
pub type DaysW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Days Value"]
    #[inline(always)]
    pub fn days(&self) -> DaysR {
        DaysR::new(self.bits & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCTDAY")
            .field("days", &self.days())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Days Value"]
    #[inline(always)]
    #[must_use]
    pub fn days(&mut self) -> DaysW<LctdaySpec> {
        DaysW::new(self, 0)
    }
}
#[doc = "LCT Days Register (LCTDAY)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctday::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctday::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctdaySpec;
impl crate::RegisterSpec for LctdaySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lctday::R`](R) reader structure"]
impl crate::Readable for LctdaySpec {}
#[doc = "`write(|w| ..)` method takes [`lctday::W`](W) writer structure"]
impl crate::Writable for LctdaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTDAY to value 0"]
impl crate::Resettable for LctdaySpec {
    const RESET_VALUE: u8 = 0;
}
