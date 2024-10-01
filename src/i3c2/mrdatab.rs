#[doc = "Register `MRDATAB` reader"]
pub type R = crate::R<MrdatabSpec>;
#[doc = "Register `MRDATAB` writer"]
pub type W = crate::W<MrdatabSpec>;
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
        f.debug_struct("MRDATAB")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MrdatabSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Controller Read Byte Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatab::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrdatab::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrdatabSpec;
impl crate::RegisterSpec for MrdatabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrdatab::R`](R) reader structure"]
impl crate::Readable for MrdatabSpec {}
#[doc = "`write(|w| ..)` method takes [`mrdatab::W`](W) writer structure"]
impl crate::Writable for MrdatabSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRDATAB to value 0"]
impl crate::Resettable for MrdatabSpec {
    const RESET_VALUE: u32 = 0;
}
