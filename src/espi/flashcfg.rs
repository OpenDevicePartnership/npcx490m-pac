#[doc = "Register `FLASHCFG` reader"]
pub type R = crate::R<FlashcfgSpec>;
#[doc = "Register `FLASHCFG` writer"]
pub type W = crate::W<FlashcfgSpec>;
#[doc = "Field `FLASHBLERSSIZE` reader - Flash Access Channel Block Erase Size"]
pub type FlashblerssizeR = crate::FieldReader;
#[doc = "Field `FLASHBLERSSIZE` writer - Flash Access Channel Block Erase Size"]
pub type FlashblerssizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLASHPLSIZE` reader - Flash Access Channel Maximum Payload Size"]
pub type FlashplsizeR = crate::FieldReader;
#[doc = "Field `FLASHPLSIZE` writer - Flash Access Channel Maximum Payload Size"]
pub type FlashplsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLASHREQSIZE` reader - Flash Access Channel Maximum Read Request Size"]
pub type FlashreqsizeR = crate::FieldReader;
#[doc = "Field `FLASHREQSIZE` writer - Flash Access Channel Maximum Read Request Size"]
pub type FlashreqsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGFLEBLKSIZE` reader - Target Flash Erase Block Size"]
pub type TrgfleblksizeR = crate::FieldReader;
#[doc = "Field `TRGFLEBLKSIZE` writer - Target Flash Erase Block Size"]
pub type TrgfleblksizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLCAPA` reader - Flash Sharing Capability Support"]
pub type FlcapaR = crate::FieldReader;
#[doc = "Field `FLCAPA` writer - Flash Sharing Capability Support"]
pub type FlcapaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 7:9 - Flash Access Channel Block Erase Size"]
    #[inline(always)]
    pub fn flashblerssize(&self) -> FlashblerssizeR {
        FlashblerssizeR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Flash Access Channel Maximum Payload Size"]
    #[inline(always)]
    pub fn flashplsize(&self) -> FlashplsizeR {
        FlashplsizeR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Flash Access Channel Maximum Read Request Size"]
    #[inline(always)]
    pub fn flashreqsize(&self) -> FlashreqsizeR {
        FlashreqsizeR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Target Flash Erase Block Size"]
    #[inline(always)]
    pub fn trgfleblksize(&self) -> TrgfleblksizeR {
        TrgfleblksizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Flash Sharing Capability Support"]
    #[inline(always)]
    pub fn flcapa(&self) -> FlcapaR {
        FlcapaR::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHCFG")
            .field("flashblerssize", &self.flashblerssize())
            .field("flashplsize", &self.flashplsize())
            .field("flashreqsize", &self.flashreqsize())
            .field("trgfleblksize", &self.trgfleblksize())
            .field("flcapa", &self.flcapa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 7:9 - Flash Access Channel Block Erase Size"]
    #[inline(always)]
    pub fn flashblerssize(&mut self) -> FlashblerssizeW<FlashcfgSpec> {
        FlashblerssizeW::new(self, 7)
    }
    #[doc = "Bits 10:12 - Flash Access Channel Maximum Payload Size"]
    #[inline(always)]
    pub fn flashplsize(&mut self) -> FlashplsizeW<FlashcfgSpec> {
        FlashplsizeW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Flash Access Channel Maximum Read Request Size"]
    #[inline(always)]
    pub fn flashreqsize(&mut self) -> FlashreqsizeW<FlashcfgSpec> {
        FlashreqsizeW::new(self, 13)
    }
    #[doc = "Bits 16:23 - Target Flash Erase Block Size"]
    #[inline(always)]
    pub fn trgfleblksize(&mut self) -> TrgfleblksizeW<FlashcfgSpec> {
        TrgfleblksizeW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Flash Sharing Capability Support"]
    #[inline(always)]
    pub fn flcapa(&mut self) -> FlcapaW<FlashcfgSpec> {
        FlcapaW::new(self, 24)
    }
}
#[doc = "Flash Channel Configuration Register (FLASHCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcfgSpec;
impl crate::RegisterSpec for FlashcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcfg::R`](R) reader structure"]
impl crate::Readable for FlashcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcfg::W`](W) writer structure"]
impl crate::Writable for FlashcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHCFG to value 0"]
impl crate::Resettable for FlashcfgSpec {
    const RESET_VALUE: u32 = 0;
}
