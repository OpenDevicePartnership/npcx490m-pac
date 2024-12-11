#[doc = "Register `FLRLCK` reader"]
pub type R = crate::R<FlrlckSpec>;
#[doc = "Register `FLRLCK` writer"]
pub type W = crate::W<FlrlckSpec>;
#[doc = "Field `SAF_PROT_RGLK` reader - Target-Attached Flash Protection Reversible Global Lock"]
pub type SafProtRglkR = crate::FieldReader;
#[doc = "Field `SAF_PROT_RGLK` writer - Target-Attached Flash Protection Reversible Global Lock"]
pub type SafProtRglkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAF_RMAP_RGLK` reader - Target-Attached Flash Remap Reversible Global Lock"]
pub type SafRmapRglkR = crate::FieldReader;
#[doc = "Field `SAF_RMAP_RGLK` writer - Target-Attached Flash Remap Reversible Global Lock"]
pub type SafRmapRglkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Target-Attached Flash Protection Reversible Global Lock"]
    #[inline(always)]
    pub fn saf_prot_rglk(&self) -> SafProtRglkR {
        SafProtRglkR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Target-Attached Flash Remap Reversible Global Lock"]
    #[inline(always)]
    pub fn saf_rmap_rglk(&self) -> SafRmapRglkR {
        SafRmapRglkR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLRLCK")
            .field("saf_prot_rglk", &self.saf_prot_rglk())
            .field("saf_rmap_rglk", &self.saf_rmap_rglk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Target-Attached Flash Protection Reversible Global Lock"]
    #[inline(always)]
    pub fn saf_prot_rglk(&mut self) -> SafProtRglkW<FlrlckSpec> {
        SafProtRglkW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Target-Attached Flash Remap Reversible Global Lock"]
    #[inline(always)]
    pub fn saf_rmap_rglk(&mut self) -> SafRmapRglkW<FlrlckSpec> {
        SafRmapRglkW::new(self, 8)
    }
}
#[doc = "Flash Reversible Lock Register (FLRLCK)\n\nYou can [`read`](crate::Reg::read) this register and get [`flrlck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flrlck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlrlckSpec;
impl crate::RegisterSpec for FlrlckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flrlck::R`](R) reader structure"]
impl crate::Readable for FlrlckSpec {}
#[doc = "`write(|w| ..)` method takes [`flrlck::W`](W) writer structure"]
impl crate::Writable for FlrlckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLRLCK to value 0"]
impl crate::Resettable for FlrlckSpec {
    const RESET_VALUE: u32 = 0;
}
