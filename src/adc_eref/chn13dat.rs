#[doc = "Register `CHN13DAT` reader"]
pub type R = crate::R<Chn13datSpec>;
#[doc = "Register `CHN13DAT` writer"]
pub type W = crate::W<Chn13datSpec>;
#[doc = "Field `CHDAT` reader - Channel Data"]
pub type ChdatR = crate::FieldReader<u16>;
#[doc = "Field `CHDAT` writer - Channel Data"]
pub type ChdatW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NEW` reader - New Conversion Result"]
pub type NewR = crate::BitReader;
#[doc = "Field `NEW` writer - New Conversion Result"]
pub type NewW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Channel Data"]
    #[inline(always)]
    pub fn chdat(&self) -> ChdatR {
        ChdatR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 15 - New Conversion Result"]
    #[inline(always)]
    pub fn new(&self) -> NewR {
        NewR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHN13DAT")
            .field("chdat", &self.chdat())
            .field("new", &self.new())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Channel Data"]
    #[inline(always)]
    #[must_use]
    pub fn chdat(&mut self) -> ChdatW<Chn13datSpec> {
        ChdatW::new(self, 0)
    }
    #[doc = "Bit 15 - New Conversion Result"]
    #[inline(always)]
    #[must_use]
    pub fn new(&mut self) -> NewW<Chn13datSpec> {
        NewW::new(self, 15)
    }
}
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn13dat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn13dat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chn13datSpec;
impl crate::RegisterSpec for Chn13datSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chn13dat::R`](R) reader structure"]
impl crate::Readable for Chn13datSpec {}
#[doc = "`write(|w| ..)` method takes [`chn13dat::W`](W) writer structure"]
impl crate::Writable for Chn13datSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CHN13DAT to value 0"]
impl crate::Resettable for Chn13datSpec {
    const RESET_VALUE: u16 = 0;
}
