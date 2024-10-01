#[doc = "Register `MWDATABE` reader"]
pub type R = crate::R<MwdatabeSpec>;
#[doc = "Register `MWDATABE` writer"]
pub type W = crate::W<MwdatabeSpec>;
#[doc = "Field `DATA` reader - Data Byte"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Data Byte"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MWDATABE")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MwdatabeSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Controller Write Byte Data as End Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwdatabe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatabe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwdatabeSpec;
impl crate::RegisterSpec for MwdatabeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mwdatabe::R`](R) reader structure"]
impl crate::Readable for MwdatabeSpec {}
#[doc = "`write(|w| ..)` method takes [`mwdatabe::W`](W) writer structure"]
impl crate::Writable for MwdatabeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWDATABE to value 0"]
impl crate::Resettable for MwdatabeSpec {
    const RESET_VALUE: u32 = 0;
}
