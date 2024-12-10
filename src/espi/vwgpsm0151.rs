#[doc = "Register `VWGPSM0-151` reader"]
pub type R = crate::R<Vwgpsm0151Spec>;
#[doc = "Register `VWGPSM0-151` writer"]
pub type W = crate::W<Vwgpsm0151Spec>;
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
#[doc = "Field `DIRTY` reader - Dirty"]
pub type DirtyR = crate::BitReader;
#[doc = "Field `DIRTY` writer - Dirty"]
pub type DirtyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENPLTRST` reader - Enable PLTRST"]
pub type EnpltrstR = crate::BitReader;
#[doc = "Field `ENPLTRST` writer - Enable PLTRST"]
pub type EnpltrstW<'a, REG> = crate::BitWriter<'a, REG>;
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
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VWGPSM0-151")
            .field("wire3_0", &self.wire3_0())
            .field("wire3_0valid", &self.wire3_0valid())
            .field("index_en", &self.index_en())
            .field("dirty", &self.dirty())
            .field("enpltrst", &self.enpltrst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Wire 3-0"]
    #[inline(always)]
    pub fn wire3_0(&mut self) -> Wire3_0W<Vwgpsm0151Spec> {
        Wire3_0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Wire 3-0 Valid"]
    #[inline(always)]
    pub fn wire3_0valid(&mut self) -> Wire3_0validW<Vwgpsm0151Spec> {
        Wire3_0validW::new(self, 4)
    }
    #[doc = "Bit 15 - Index Enable"]
    #[inline(always)]
    pub fn index_en(&mut self) -> IndexEnW<Vwgpsm0151Spec> {
        IndexEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Dirty"]
    #[inline(always)]
    pub fn dirty(&mut self) -> DirtyW<Vwgpsm0151Spec> {
        DirtyW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable PLTRST"]
    #[inline(always)]
    pub fn enpltrst(&mut self) -> EnpltrstW<Vwgpsm0151Spec> {
        EnpltrstW::new(self, 17)
    }
}
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0151::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0151::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vwgpsm0151Spec;
impl crate::RegisterSpec for Vwgpsm0151Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwgpsm0151::R`](R) reader structure"]
impl crate::Readable for Vwgpsm0151Spec {}
#[doc = "`write(|w| ..)` method takes [`vwgpsm0151::W`](W) writer structure"]
impl crate::Writable for Vwgpsm0151Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWGPSM0-151 to value 0"]
impl crate::Resettable for Vwgpsm0151Spec {
    const RESET_VALUE: u32 = 0;
}
