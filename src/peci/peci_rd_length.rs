#[doc = "Register `PECI_RD_LENGTH` reader"]
pub type R = crate::R<PeciRdLengthSpec>;
#[doc = "Register `PECI_RD_LENGTH` writer"]
pub type W = crate::W<PeciRdLengthSpec>;
#[doc = "Field `READ_LENGTH` reader - Read Length"]
pub type ReadLengthR = crate::FieldReader;
#[doc = "Field `READ_LENGTH` writer - Read Length"]
pub type ReadLengthW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Read Length"]
    #[inline(always)]
    pub fn read_length(&self) -> ReadLengthR {
        ReadLengthR::new(self.bits & 0x7f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECI_RD_LENGTH")
            .field("read_length", &self.read_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Read Length"]
    #[inline(always)]
    #[must_use]
    pub fn read_length(&mut self) -> ReadLengthW<PeciRdLengthSpec> {
        ReadLengthW::new(self, 0)
    }
}
#[doc = "PECI Read Length Register (PECI_RD_LENGTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_rd_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_rd_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciRdLengthSpec;
impl crate::RegisterSpec for PeciRdLengthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_rd_length::R`](R) reader structure"]
impl crate::Readable for PeciRdLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_rd_length::W`](W) writer structure"]
impl crate::Writable for PeciRdLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_RD_LENGTH to value 0"]
impl crate::Resettable for PeciRdLengthSpec {
    const RESET_VALUE: u8 = 0;
}
