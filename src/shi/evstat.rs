#[doc = "Register `EVSTAT` reader"]
pub type R = crate::R<EvstatSpec>;
#[doc = "Register `EVSTAT` writer"]
pub type W = crate::W<EvstatSpec>;
#[doc = "Field `OBE` reader - 128-Byte Payload Buffer Empty"]
pub type ObeR = crate::BitReader;
#[doc = "Field `OBE` writer - 128-Byte Payload Buffer Empty"]
pub type ObeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBHE` reader - 128-Byte Payload Buffer Half Empty"]
pub type ObheR = crate::BitReader;
#[doc = "Field `OBHE` writer - 128-Byte Payload Buffer Half Empty"]
pub type ObheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBF` reader - Input Buffer Full"]
pub type IbfR = crate::BitReader;
#[doc = "Field `IBF` writer - Input Buffer Full"]
pub type IbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBHF` reader - Input Buffer Half Full"]
pub type IbhfR = crate::BitReader;
#[doc = "Field `IBHF` writer - Input Buffer Half Full"]
pub type IbhfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOR` reader - End-of-Data for Read Transaction"]
pub type EorR = crate::BitReader;
#[doc = "Field `EOR` writer - End-of-Data for Read Transaction"]
pub type EorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOW` reader - End-of-Data for Write Transaction"]
pub type EowR = crate::BitReader;
#[doc = "Field `EOW` writer - End-of-Data for Write Transaction"]
pub type EowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSR` reader - Status Read"]
pub type StsrR = crate::BitReader;
#[doc = "Field `STSR` writer - Status Read"]
pub type StsrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 128-Byte Payload Buffer Empty"]
    #[inline(always)]
    pub fn obe(&self) -> ObeR {
        ObeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 128-Byte Payload Buffer Half Empty"]
    #[inline(always)]
    pub fn obhe(&self) -> ObheR {
        ObheR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Buffer Full"]
    #[inline(always)]
    pub fn ibf(&self) -> IbfR {
        IbfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Buffer Half Full"]
    #[inline(always)]
    pub fn ibhf(&self) -> IbhfR {
        IbhfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End-of-Data for Read Transaction"]
    #[inline(always)]
    pub fn eor(&self) -> EorR {
        EorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End-of-Data for Write Transaction"]
    #[inline(always)]
    pub fn eow(&self) -> EowR {
        EowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status Read"]
    #[inline(always)]
    pub fn stsr(&self) -> StsrR {
        StsrR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVSTAT")
            .field("obe", &self.obe())
            .field("obhe", &self.obhe())
            .field("ibf", &self.ibf())
            .field("ibhf", &self.ibhf())
            .field("eor", &self.eor())
            .field("eow", &self.eow())
            .field("stsr", &self.stsr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 128-Byte Payload Buffer Empty"]
    #[inline(always)]
    #[must_use]
    pub fn obe(&mut self) -> ObeW<EvstatSpec> {
        ObeW::new(self, 0)
    }
    #[doc = "Bit 1 - 128-Byte Payload Buffer Half Empty"]
    #[inline(always)]
    #[must_use]
    pub fn obhe(&mut self) -> ObheW<EvstatSpec> {
        ObheW::new(self, 1)
    }
    #[doc = "Bit 2 - Input Buffer Full"]
    #[inline(always)]
    #[must_use]
    pub fn ibf(&mut self) -> IbfW<EvstatSpec> {
        IbfW::new(self, 2)
    }
    #[doc = "Bit 3 - Input Buffer Half Full"]
    #[inline(always)]
    #[must_use]
    pub fn ibhf(&mut self) -> IbhfW<EvstatSpec> {
        IbhfW::new(self, 3)
    }
    #[doc = "Bit 4 - End-of-Data for Read Transaction"]
    #[inline(always)]
    #[must_use]
    pub fn eor(&mut self) -> EorW<EvstatSpec> {
        EorW::new(self, 4)
    }
    #[doc = "Bit 5 - End-of-Data for Write Transaction"]
    #[inline(always)]
    #[must_use]
    pub fn eow(&mut self) -> EowW<EvstatSpec> {
        EowW::new(self, 5)
    }
    #[doc = "Bit 6 - Status Read"]
    #[inline(always)]
    #[must_use]
    pub fn stsr(&mut self) -> StsrW<EvstatSpec> {
        StsrW::new(self, 6)
    }
}
#[doc = "Event Status\n\nYou can [`read`](crate::Reg::read) this register and get [`evstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvstatSpec;
impl crate::RegisterSpec for EvstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evstat::R`](R) reader structure"]
impl crate::Readable for EvstatSpec {}
#[doc = "`write(|w| ..)` method takes [`evstat::W`](W) writer structure"]
impl crate::Writable for EvstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EVSTAT to value 0"]
impl crate::Resettable for EvstatSpec {
    const RESET_VALUE: u8 = 0;
}
