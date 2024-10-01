#[doc = "Register `VWSWIRQ` reader"]
pub type R = crate::R<VwswirqSpec>;
#[doc = "Register `VWSWIRQ` writer"]
pub type W = crate::W<VwswirqSpec>;
#[doc = "Field `IRQ_NUM` reader - IRQ Number"]
pub type IrqNumR = crate::FieldReader;
#[doc = "Field `IRQ_NUM` writer - IRQ Number"]
pub type IrqNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IRQ_LVL` reader - IRQ Level"]
pub type IrqLvlR = crate::BitReader;
#[doc = "Field `IRQ_LVL` writer - IRQ Level"]
pub type IrqLvlW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EDGE_IRQ` reader - Edge Type IRQ"]
pub type EdgeIrqR = crate::BitReader;
#[doc = "Field `EDGE_IRQ` writer - Edge Type IRQ"]
pub type EdgeIrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - IRQ Number"]
    #[inline(always)]
    pub fn irq_num(&self) -> IrqNumR {
        IrqNumR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - IRQ Level"]
    #[inline(always)]
    pub fn irq_lvl(&self) -> IrqLvlR {
        IrqLvlR::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 28 - Edge Type IRQ"]
    #[inline(always)]
    pub fn edge_irq(&self) -> EdgeIrqR {
        EdgeIrqR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VWSWIRQ")
            .field("irq_num", &self.irq_num())
            .field("irq_lvl", &self.irq_lvl())
            .field("index", &self.index())
            .field("index_en", &self.index_en())
            .field("dirty", &self.dirty())
            .field("enpltrst", &self.enpltrst())
            .field("encdrst", &self.encdrst())
            .field("edge_irq", &self.edge_irq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - IRQ Number"]
    #[inline(always)]
    #[must_use]
    pub fn irq_num(&mut self) -> IrqNumW<VwswirqSpec> {
        IrqNumW::new(self, 0)
    }
    #[doc = "Bit 7 - IRQ Level"]
    #[inline(always)]
    #[must_use]
    pub fn irq_lvl(&mut self) -> IrqLvlW<VwswirqSpec> {
        IrqLvlW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Index Value"]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> IndexW<VwswirqSpec> {
        IndexW::new(self, 8)
    }
    #[doc = "Bit 15 - Index Enable"]
    #[inline(always)]
    #[must_use]
    pub fn index_en(&mut self) -> IndexEnW<VwswirqSpec> {
        IndexEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Dirty"]
    #[inline(always)]
    #[must_use]
    pub fn dirty(&mut self) -> DirtyW<VwswirqSpec> {
        DirtyW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable PLTRST"]
    #[inline(always)]
    #[must_use]
    pub fn enpltrst(&mut self) -> EnpltrstW<VwswirqSpec> {
        EnpltrstW::new(self, 17)
    }
    #[doc = "Bit 19 - Enable Core Reset"]
    #[inline(always)]
    #[must_use]
    pub fn encdrst(&mut self) -> EncdrstW<VwswirqSpec> {
        EncdrstW::new(self, 19)
    }
    #[doc = "Bit 28 - Edge Type IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn edge_irq(&mut self) -> EdgeIrqW<VwswirqSpec> {
        EdgeIrqW::new(self, 28)
    }
}
#[doc = "Virtual Wire Software IRQ Register (VWSWIRQ)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwswirq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwswirq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VwswirqSpec;
impl crate::RegisterSpec for VwswirqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwswirq::R`](R) reader structure"]
impl crate::Readable for VwswirqSpec {}
#[doc = "`write(|w| ..)` method takes [`vwswirq::W`](W) writer structure"]
impl crate::Writable for VwswirqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWSWIRQ to value 0"]
impl crate::Resettable for VwswirqSpec {
    const RESET_VALUE: u32 = 0;
}
