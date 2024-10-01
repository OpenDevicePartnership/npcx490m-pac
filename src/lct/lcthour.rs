#[doc = "Register `LCTHOUR` reader"]
pub type R = crate::R<LcthourSpec>;
#[doc = "Register `LCTHOUR` writer"]
pub type W = crate::W<LcthourSpec>;
#[doc = "Field `HOURS` reader - Hours Value"]
pub type HoursR = crate::FieldReader;
#[doc = "Field `HOURS` writer - Hours Value"]
pub type HoursW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Hours Value"]
    #[inline(always)]
    pub fn hours(&self) -> HoursR {
        HoursR::new(self.bits & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCTHOUR")
            .field("hours", &self.hours())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours Value"]
    #[inline(always)]
    #[must_use]
    pub fn hours(&mut self) -> HoursW<LcthourSpec> {
        HoursW::new(self, 0)
    }
}
#[doc = "LCT Hours Register (LCTHOUR)\n\nYou can [`read`](crate::Reg::read) this register and get [`lcthour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcthour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcthourSpec;
impl crate::RegisterSpec for LcthourSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcthour::R`](R) reader structure"]
impl crate::Readable for LcthourSpec {}
#[doc = "`write(|w| ..)` method takes [`lcthour::W`](W) writer structure"]
impl crate::Writable for LcthourSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTHOUR to value 0"]
impl crate::Resettable for LcthourSpec {
    const RESET_VALUE: u8 = 0;
}
