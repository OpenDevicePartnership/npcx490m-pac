#[doc = "Register `PECI_INDEX` reader"]
pub type R = crate::R<PeciIndexSpec>;
#[doc = "Register `PECI_INDEX` writer"]
pub type W = crate::W<PeciIndexSpec>;
#[doc = "Field `DATA_INDEX` reader - Data Index"]
pub type DataIndexR = crate::FieldReader;
#[doc = "Field `DATA_INDEX` writer - Data Index"]
pub type DataIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Data Index"]
    #[inline(always)]
    pub fn data_index(&self) -> DataIndexR {
        DataIndexR::new(self.bits & 0x3f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECI_INDEX")
            .field("data_index", &self.data_index())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Index"]
    #[inline(always)]
    #[must_use]
    pub fn data_index(&mut self) -> DataIndexW<PeciIndexSpec> {
        DataIndexW::new(self, 0)
    }
}
#[doc = "PECI Index Register (PECI_INDEX)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciIndexSpec;
impl crate::RegisterSpec for PeciIndexSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_index::R`](R) reader structure"]
impl crate::Readable for PeciIndexSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_index::W`](W) writer structure"]
impl crate::Writable for PeciIndexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_INDEX to value 0"]
impl crate::Resettable for PeciIndexSpec {
    const RESET_VALUE: u8 = 0;
}
