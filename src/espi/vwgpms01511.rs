#[doc = "Register `VWGPMS0-1511` reader"]
pub type R = crate::R<Vwgpms01511Spec>;
#[doc = "Register `VWGPMS0-1511` writer"]
pub type W = crate::W<Vwgpms01511Spec>;
#[doc = "Field `WIRE 3_0` reader - Wire 3-0"]
pub type Wire3_0R = crate::FieldReader;
#[doc = "Field `WIRE 3_0` writer - Wire 3-0"]
pub type Wire3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIRE 3_0 VALID` reader - Wire 3-0 Valid"]
pub type Wire3_0validR = crate::FieldReader;
#[doc = "Field `WIRE 3_0 VALID` writer - Wire 3-0 Valid"]
pub type Wire3_0validW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
        f.debug_struct("VWGPMS0-1511")
            .field("wire3_0", &self.wire3_0())
            .field("wire3_0valid", &self.wire3_0valid())
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
    pub fn wire3_0(&mut self) -> Wire3_0W<Vwgpms01511Spec> {
        Wire3_0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Wire 3-0 Valid"]
    #[inline(always)]
    pub fn wire3_0valid(&mut self) -> Wire3_0validW<Vwgpms01511Spec> {
        Wire3_0validW::new(self, 4)
    }
    #[doc = "Bit 15 - Index Enable"]
    #[inline(always)]
    pub fn index_en(&mut self) -> IndexEnW<Vwgpms01511Spec> {
        IndexEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Modified"]
    #[inline(always)]
    pub fn modified(&mut self) -> ModifiedW<Vwgpms01511Spec> {
        ModifiedW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable PLTRST"]
    #[inline(always)]
    pub fn enpltrst(&mut self) -> EnpltrstW<Vwgpms01511Spec> {
        EnpltrstW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<Vwgpms01511Spec> {
        IeW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable eSPI_RST"]
    #[inline(always)]
    pub fn enespirst(&mut self) -> EnespirstW<Vwgpms01511Spec> {
        EnespirstW::new(self, 19)
    }
    #[doc = "Bit 20 - Wake-up Enable"]
    #[inline(always)]
    pub fn we(&mut self) -> WeW<Vwgpms01511Spec> {
        WeW::new(self, 20)
    }
}
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms01511::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms01511::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vwgpms01511Spec;
impl crate::RegisterSpec for Vwgpms01511Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwgpms01511::R`](R) reader structure"]
impl crate::Readable for Vwgpms01511Spec {}
#[doc = "`write(|w| ..)` method takes [`vwgpms01511::W`](W) writer structure"]
impl crate::Writable for Vwgpms01511Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWGPMS0-1511 to value 0"]
impl crate::Resettable for Vwgpms01511Spec {
    const RESET_VALUE: u32 = 0;
}
