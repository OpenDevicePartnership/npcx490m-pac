#[doc = "Register `PECI_WR_LENGTH` reader"]
pub type R = crate::R<PeciWrLengthSpec>;
#[doc = "Register `PECI_WR_LENGTH` writer"]
pub type W = crate::W<PeciWrLengthSpec>;
#[doc = "Field `WRITE_LENGTH` reader - WRITE Length"]
pub type WriteLengthR = crate::FieldReader;
#[doc = "Field `WRITE_LENGTH` writer - WRITE Length"]
pub type WriteLengthW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRITE Length"]
    #[inline(always)]
    pub fn write_length(&self) -> WriteLengthR {
        WriteLengthR::new(self.bits & 0x7f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECI_WR_LENGTH")
            .field("write_length", &self.write_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - WRITE Length"]
    #[inline(always)]
    #[must_use]
    pub fn write_length(&mut self) -> WriteLengthW<PeciWrLengthSpec> {
        WriteLengthW::new(self, 0)
    }
}
#[doc = "PECI Write Length Register (PECI_WR_LENGTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_wr_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_wr_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciWrLengthSpec;
impl crate::RegisterSpec for PeciWrLengthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_wr_length::R`](R) reader structure"]
impl crate::Readable for PeciWrLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_wr_length::W`](W) writer structure"]
impl crate::Writable for PeciWrLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_WR_LENGTH to value 0"]
impl crate::Resettable for PeciWrLengthSpec {
    const RESET_VALUE: u8 = 0;
}
