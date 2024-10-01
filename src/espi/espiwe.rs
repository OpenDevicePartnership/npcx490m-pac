#[doc = "Register `ESPIWE` reader"]
pub type R = crate::R<EspiweSpec>;
#[doc = "Register `ESPIWE` writer"]
pub type W = crate::W<EspiweSpec>;
#[doc = "Field `IBRSTWE` reader - IBRST Wake-Up Enable"]
pub type IbrstweR = crate::BitReader;
#[doc = "Field `IBRSTWE` writer - IBRST Wake-Up Enable"]
pub type IbrstweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGUPDWE` reader - CFGUPD Wake-Up Enable"]
pub type CfgupdweR = crate::BitReader;
#[doc = "Field `CFGUPDWE` writer - CFGUPD Wake-Up Enable"]
pub type CfgupdweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRWE` reader - BERR Wake-Up Enable"]
pub type BerrweR = crate::BitReader;
#[doc = "Field `BERRWE` writer - BERR Wake-Up Enable"]
pub type BerrweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOBRXWE` reader - OOBRX Wake-Up Enable"]
pub type OobrxweR = crate::BitReader;
#[doc = "Field `OOBRXWE` writer - OOBRX Wake-Up Enable"]
pub type OobrxweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRXWE` reader - FLASHRX Wake-Up Enable"]
pub type FlashrxweR = crate::BitReader;
#[doc = "Field `FLASHRXWE` writer - FLASHRX Wake-Up Enable"]
pub type FlashrxweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERACCWE` reader - PERACC Wake-Up Enable"]
pub type PeraccweR = crate::BitReader;
#[doc = "Field `PERACCWE` writer - PERACC Wake-Up Enable"]
pub type PeraccweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFRDWE` reader - DFRD Wake-Up Enable"]
pub type DfrdweR = crate::BitReader;
#[doc = "Field `DFRDWE` writer - DFRD Wake-Up Enable"]
pub type DfrdweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWUPDWE` reader - VWUPD Wake-Up Enable"]
pub type VwupdweR = crate::BitReader;
#[doc = "Field `VWUPDWE` writer - VWUPD Wake-Up Enable"]
pub type VwupdweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPIRSTWE` reader - eSPI_RST Wake-Up Enable"]
pub type EspirstweR = crate::BitReader;
#[doc = "Field `ESPIRSTWE` writer - eSPI_RST Wake-Up Enable"]
pub type EspirstweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBMRXWE` reader - PBMRX Wake-Up Enable"]
pub type PbmrxweR = crate::BitReader;
#[doc = "Field `PBMRXWE` writer - PBMRX Wake-Up Enable"]
pub type PbmrxweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMSGRXWE` reader - PMSGRX Wake-Up Enable"]
pub type PmsgrxweR = crate::BitReader;
#[doc = "Field `PMSGRXWE` writer - PMSGRX Wake-Up Enable"]
pub type PmsgrxweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAUTORDSTRWE` reader - FLAUTORDSTR Wake-Up Enable"]
pub type FlautordstrweR = crate::BitReader;
#[doc = "Field `FLAUTORDSTRWE` writer - FLAUTORDSTR Wake-Up Enable"]
pub type FlautordstrweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IBRST Wake-Up Enable"]
    #[inline(always)]
    pub fn ibrstwe(&self) -> IbrstweR {
        IbrstweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CFGUPD Wake-Up Enable"]
    #[inline(always)]
    pub fn cfgupdwe(&self) -> CfgupdweR {
        CfgupdweR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BERR Wake-Up Enable"]
    #[inline(always)]
    pub fn berrwe(&self) -> BerrweR {
        BerrweR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OOBRX Wake-Up Enable"]
    #[inline(always)]
    pub fn oobrxwe(&self) -> OobrxweR {
        OobrxweR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASHRX Wake-Up Enable"]
    #[inline(always)]
    pub fn flashrxwe(&self) -> FlashrxweR {
        FlashrxweR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - PERACC Wake-Up Enable"]
    #[inline(always)]
    pub fn peraccwe(&self) -> PeraccweR {
        PeraccweR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFRD Wake-Up Enable"]
    #[inline(always)]
    pub fn dfrdwe(&self) -> DfrdweR {
        DfrdweR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VWUPD Wake-Up Enable"]
    #[inline(always)]
    pub fn vwupdwe(&self) -> VwupdweR {
        VwupdweR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - eSPI_RST Wake-Up Enable"]
    #[inline(always)]
    pub fn espirstwe(&self) -> EspirstweR {
        EspirstweR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 20 - PBMRX Wake-Up Enable"]
    #[inline(always)]
    pub fn pbmrxwe(&self) -> PbmrxweR {
        PbmrxweR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PMSGRX Wake-Up Enable"]
    #[inline(always)]
    pub fn pmsgrxwe(&self) -> PmsgrxweR {
        PmsgrxweR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - FLAUTORDSTR Wake-Up Enable"]
    #[inline(always)]
    pub fn flautordstrwe(&self) -> FlautordstrweR {
        FlautordstrweR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPIWE")
            .field("ibrstwe", &self.ibrstwe())
            .field("cfgupdwe", &self.cfgupdwe())
            .field("berrwe", &self.berrwe())
            .field("oobrxwe", &self.oobrxwe())
            .field("flashrxwe", &self.flashrxwe())
            .field("peraccwe", &self.peraccwe())
            .field("dfrdwe", &self.dfrdwe())
            .field("vwupdwe", &self.vwupdwe())
            .field("espirstwe", &self.espirstwe())
            .field("pbmrxwe", &self.pbmrxwe())
            .field("pmsgrxwe", &self.pmsgrxwe())
            .field("flautordstrwe", &self.flautordstrwe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IBRST Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ibrstwe(&mut self) -> IbrstweW<EspiweSpec> {
        IbrstweW::new(self, 0)
    }
    #[doc = "Bit 1 - CFGUPD Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfgupdwe(&mut self) -> CfgupdweW<EspiweSpec> {
        CfgupdweW::new(self, 1)
    }
    #[doc = "Bit 2 - BERR Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn berrwe(&mut self) -> BerrweW<EspiweSpec> {
        BerrweW::new(self, 2)
    }
    #[doc = "Bit 3 - OOBRX Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oobrxwe(&mut self) -> OobrxweW<EspiweSpec> {
        OobrxweW::new(self, 3)
    }
    #[doc = "Bit 4 - FLASHRX Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashrxwe(&mut self) -> FlashrxweW<EspiweSpec> {
        FlashrxweW::new(self, 4)
    }
    #[doc = "Bit 6 - PERACC Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peraccwe(&mut self) -> PeraccweW<EspiweSpec> {
        PeraccweW::new(self, 6)
    }
    #[doc = "Bit 7 - DFRD Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfrdwe(&mut self) -> DfrdweW<EspiweSpec> {
        DfrdweW::new(self, 7)
    }
    #[doc = "Bit 8 - VWUPD Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vwupdwe(&mut self) -> VwupdweW<EspiweSpec> {
        VwupdweW::new(self, 8)
    }
    #[doc = "Bit 9 - eSPI_RST Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn espirstwe(&mut self) -> EspirstweW<EspiweSpec> {
        EspirstweW::new(self, 9)
    }
    #[doc = "Bit 20 - PBMRX Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pbmrxwe(&mut self) -> PbmrxweW<EspiweSpec> {
        PbmrxweW::new(self, 20)
    }
    #[doc = "Bit 21 - PMSGRX Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmsgrxwe(&mut self) -> PmsgrxweW<EspiweSpec> {
        PmsgrxweW::new(self, 21)
    }
    #[doc = "Bit 26 - FLAUTORDSTR Wake-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flautordstrwe(&mut self) -> FlautordstrweW<EspiweSpec> {
        FlautordstrweW::new(self, 26)
    }
}
#[doc = "eSPI Wake-Up Enable Register (ESPIWE)\n\nYou can [`read`](crate::Reg::read) this register and get [`espiwe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiwe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspiweSpec;
impl crate::RegisterSpec for EspiweSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espiwe::R`](R) reader structure"]
impl crate::Readable for EspiweSpec {}
#[doc = "`write(|w| ..)` method takes [`espiwe::W`](W) writer structure"]
impl crate::Writable for EspiweSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESPIWE to value 0"]
impl crate::Resettable for EspiweSpec {
    const RESET_VALUE: u32 = 0;
}
