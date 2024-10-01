#[doc = "Register `VWEVSM0-99` reader"]
pub type R = crate::R<Vwevsm099Spec>;
#[doc = "Register `VWEVSM0-99` writer"]
pub type W = crate::W<Vwevsm099Spec>;
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
#[doc = "Field `DIRTY` reader - Dirty"]
pub type DirtyR = crate::BitReader;
#[doc = "Field `DIRTY` writer - Dirty"]
pub type DirtyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENPLTRST` reader - Enable PLTRST"]
pub type EnpltrstR = crate::BitReader;
#[doc = "Field `ENPLTRST` writer - Enable PLTRST"]
pub type EnpltrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCDRST` reader - Enable Core Reset"]
pub type EncdrstR = crate::BitReader;
#[doc = "Field `ENCDRST` writer - Enable Core Reset"]
pub type EncdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL_WIRE 3_0` reader - Polarity of Wire 3-0"]
pub type PolWire3_0R = crate::FieldReader;
#[doc = "Field `POL_WIRE 3_0` writer - Polarity of Wire 3-0"]
pub type PolWire3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HW_WIRE 3_0` reader - Hardware-Controlled Wire 3-0"]
pub type HwWire3_0R = crate::FieldReader;
#[doc = "Field `HW_WIRE 3_0` writer - Hardware-Controlled Wire 3-0"]
pub type HwWire3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EDGE_WIRE 3_0` reader - Edge Type Wire 3-0"]
pub type EdgeWire3_0R = crate::FieldReader;
#[doc = "Field `EDGE_WIRE 3_0` writer - Edge Type Wire 3-0"]
pub type EdgeWire3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bit 16 - Dirty"]
    #[inline(always)]
    pub fn dirty(&self) -> DirtyR {
        DirtyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable PLTRST"]
    #[inline(always)]
    pub fn enpltrst(&self) -> EnpltrstR {
        EnpltrstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Core Reset"]
    #[inline(always)]
    pub fn encdrst(&self) -> EncdrstR {
        EncdrstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Polarity of Wire 3-0"]
    #[inline(always)]
    pub fn pol_wire3_0(&self) -> PolWire3_0R {
        PolWire3_0R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Hardware-Controlled Wire 3-0"]
    #[inline(always)]
    pub fn hw_wire3_0(&self) -> HwWire3_0R {
        HwWire3_0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Edge Type Wire 3-0"]
    #[inline(always)]
    pub fn edge_wire3_0(&self) -> EdgeWire3_0R {
        EdgeWire3_0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VWEVSM0-99")
            .field("wire3_0", &self.wire3_0())
            .field("wire3_0valid", &self.wire3_0valid())
            .field("index", &self.index())
            .field("index_en", &self.index_en())
            .field("dirty", &self.dirty())
            .field("enpltrst", &self.enpltrst())
            .field("encdrst", &self.encdrst())
            .field("pol_wire3_0", &self.pol_wire3_0())
            .field("hw_wire3_0", &self.hw_wire3_0())
            .field("edge_wire3_0", &self.edge_wire3_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Wire 3-0"]
    #[inline(always)]
    #[must_use]
    pub fn wire3_0(&mut self) -> Wire3_0W<Vwevsm099Spec> {
        Wire3_0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Wire 3-0 Valid"]
    #[inline(always)]
    #[must_use]
    pub fn wire3_0valid(&mut self) -> Wire3_0validW<Vwevsm099Spec> {
        Wire3_0validW::new(self, 4)
    }
    #[doc = "Bits 8:14 - Index Value"]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> IndexW<Vwevsm099Spec> {
        IndexW::new(self, 8)
    }
    #[doc = "Bit 15 - Index Enable"]
    #[inline(always)]
    #[must_use]
    pub fn index_en(&mut self) -> IndexEnW<Vwevsm099Spec> {
        IndexEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Dirty"]
    #[inline(always)]
    #[must_use]
    pub fn dirty(&mut self) -> DirtyW<Vwevsm099Spec> {
        DirtyW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable PLTRST"]
    #[inline(always)]
    #[must_use]
    pub fn enpltrst(&mut self) -> EnpltrstW<Vwevsm099Spec> {
        EnpltrstW::new(self, 17)
    }
    #[doc = "Bit 19 - Enable Core Reset"]
    #[inline(always)]
    #[must_use]
    pub fn encdrst(&mut self) -> EncdrstW<Vwevsm099Spec> {
        EncdrstW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Polarity of Wire 3-0"]
    #[inline(always)]
    #[must_use]
    pub fn pol_wire3_0(&mut self) -> PolWire3_0W<Vwevsm099Spec> {
        PolWire3_0W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Hardware-Controlled Wire 3-0"]
    #[inline(always)]
    #[must_use]
    pub fn hw_wire3_0(&mut self) -> HwWire3_0W<Vwevsm099Spec> {
        HwWire3_0W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Edge Type Wire 3-0"]
    #[inline(always)]
    #[must_use]
    pub fn edge_wire3_0(&mut self) -> EdgeWire3_0W<Vwevsm099Spec> {
        EdgeWire3_0W::new(self, 28)
    }
}
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm099::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm099::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vwevsm099Spec;
impl crate::RegisterSpec for Vwevsm099Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwevsm099::R`](R) reader structure"]
impl crate::Readable for Vwevsm099Spec {}
#[doc = "`write(|w| ..)` method takes [`vwevsm099::W`](W) writer structure"]
impl crate::Writable for Vwevsm099Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWEVSM0-99 to value 0"]
impl crate::Resettable for Vwevsm099Spec {
    const RESET_VALUE: u32 = 0;
}
