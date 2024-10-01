#[doc = "Register `VWEVMS0-112` reader"]
pub type R = crate::R<Vwevms0112Spec>;
#[doc = "Register `VWEVMS0-112` writer"]
pub type W = crate::W<Vwevms0112Spec>;
#[doc = "Field `WIRE 3_0` reader - Wire 3-0"]
pub type Wire3_0R = crate::FieldReader;
#[doc = "Field `WIRE 3_0` writer - Wire 3-0"]
pub type Wire3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIRE 3_0 VALID` reader - Wire 3-0 Valid"]
pub type Wire3_0validR = crate::FieldReader;
#[doc = "Field `WIRE 3_0 VALID` writer - Wire 3-0 Valid"]
pub type Wire3_0validW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INDEX` reader - Index Value"]
pub type IndexR = crate::FieldReader;
#[doc = "Field `INDEX` writer - Index Value"]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `INDEX_EN` reader - Index Enable"]
pub type IndexEnR = crate::BitReader;
#[doc = "Field `INDEX_EN` writer - Index Enable"]
pub type IndexEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODIFIED` reader - Modified"]
pub type ModifiedR = crate::BitReader;
#[doc = "Field `MODIFIED` writer - Modified"]
pub type ModifiedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENPLTRST` reader - Enable PLTRST"]
pub type EnpltrstR = crate::BitReader;
#[doc = "Field `ENPLTRST` writer - Enable PLTRST"]
pub type EnpltrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENESPIRST` reader - Enable eSPI_RST"]
pub type EnespirstR = crate::BitReader;
#[doc = "Field `ENESPIRST` writer - Enable eSPI_RST"]
pub type EnespirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WE` reader - Wake-up Enable"]
pub type WeR = crate::BitReader;
#[doc = "Field `WE` writer - Wake-up Enable"]
pub type WeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Wire 3-0"]
    #[inline(always)]
    pub fn wire3_0(&self) -> Wire3_0R {
        Wire3_0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Wire 3-0 Valid"]
    #[inline(always)]
    pub fn wire3_0valid(&self) -> Wire3_0validR {
        Wire3_0validR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Index Value"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Index Enable"]
    #[inline(always)]
    pub fn index_en(&self) -> IndexEnR {
        IndexEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Modified"]
    #[inline(always)]
    pub fn modified(&self) -> ModifiedR {
        ModifiedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable PLTRST"]
    #[inline(always)]
    pub fn enpltrst(&self) -> EnpltrstR {
        EnpltrstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable eSPI_RST"]
    #[inline(always)]
    pub fn enespirst(&self) -> EnespirstR {
        EnespirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake-up Enable"]
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VWEVMS0-112")
            .field("wire3_0", &self.wire3_0())
            .field("wire3_0valid", &self.wire3_0valid())
            .field("index", &self.index())
            .field("index_en", &self.index_en())
            .field("modified", &self.modified())
            .field("enpltrst", &self.enpltrst())
            .field("ie", &self.ie())
            .field("enespirst", &self.enespirst())
            .field("we", &self.we())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Wire 3-0"]
    #[inline(always)]
    #[must_use]
    pub fn wire3_0(&mut self) -> Wire3_0W<Vwevms0112Spec> {
        Wire3_0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Wire 3-0 Valid"]
    #[inline(always)]
    #[must_use]
    pub fn wire3_0valid(&mut self) -> Wire3_0validW<Vwevms0112Spec> {
        Wire3_0validW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Index Value"]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> IndexW<Vwevms0112Spec> {
        IndexW::new(self, 8)
    }
    #[doc = "Bit 15 - Index Enable"]
    #[inline(always)]
    #[must_use]
    pub fn index_en(&mut self) -> IndexEnW<Vwevms0112Spec> {
        IndexEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Modified"]
    #[inline(always)]
    #[must_use]
    pub fn modified(&mut self) -> ModifiedW<Vwevms0112Spec> {
        ModifiedW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable PLTRST"]
    #[inline(always)]
    #[must_use]
    pub fn enpltrst(&mut self) -> EnpltrstW<Vwevms0112Spec> {
        EnpltrstW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<Vwevms0112Spec> {
        IeW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable eSPI_RST"]
    #[inline(always)]
    #[must_use]
    pub fn enespirst(&mut self) -> EnespirstW<Vwevms0112Spec> {
        EnespirstW::new(self, 19)
    }
    #[doc = "Bit 20 - Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WeW<Vwevms0112Spec> {
        WeW::new(self, 20)
    }
}
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0112::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0112::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vwevms0112Spec;
impl crate::RegisterSpec for Vwevms0112Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwevms0112::R`](R) reader structure"]
impl crate::Readable for Vwevms0112Spec {}
#[doc = "`write(|w| ..)` method takes [`vwevms0112::W`](W) writer structure"]
impl crate::Writable for Vwevms0112Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWEVMS0-112 to value 0"]
impl crate::Resettable for Vwevms0112Spec {
    const RESET_VALUE: u32 = 0;
}
