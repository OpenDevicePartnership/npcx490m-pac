#[doc = "Register `ESPIIE` reader"]
pub type R = crate::R<EspiieSpec>;
#[doc = "Register `ESPIIE` writer"]
pub type W = crate::W<EspiieSpec>;
#[doc = "Field `IBRSTIE` reader - IBRST Interrupt Enable"]
pub type IbrstieR = crate::BitReader;
#[doc = "Field `IBRSTIE` writer - IBRST Interrupt Enable"]
pub type IbrstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGUPDIE` reader - CFGUPD Interrupt Enable"]
pub type CfgupdieR = crate::BitReader;
#[doc = "Field `CFGUPDIE` writer - CFGUPD Interrupt Enable"]
pub type CfgupdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRIE` reader - BERR Interrupt Enable"]
pub type BerrieR = crate::BitReader;
#[doc = "Field `BERRIE` writer - BERR Interrupt Enable"]
pub type BerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOBRXIE` reader - OOBRX Interrupt Enable"]
pub type OobrxieR = crate::BitReader;
#[doc = "Field `OOBRXIE` writer - OOBRX Interrupt Enable"]
pub type OobrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRXIE` reader - FLASHRX Interrupt Enable"]
pub type FlashrxieR = crate::BitReader;
#[doc = "Field `FLASHRXIE` writer - FLASHRX Interrupt Enable"]
pub type FlashrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLNACSIE` reader - FLNACS Interrupt Enable"]
pub type FlnacsieR = crate::BitReader;
#[doc = "Field `FLNACSIE` writer - FLNACS Interrupt Enable"]
pub type FlnacsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERACCIE` reader - PERACC Interrupt Enable"]
pub type PeraccieR = crate::BitReader;
#[doc = "Field `PERACCIE` writer - PERACC Interrupt Enable"]
pub type PeraccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFRDIE` reader - DFRD Interrupt Enable"]
pub type DfrdieR = crate::BitReader;
#[doc = "Field `DFRDIE` writer - DFRD Interrupt Enable"]
pub type DfrdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWUPDIE` reader - VWUPD Interrupt Enable"]
pub type VwupdieR = crate::BitReader;
#[doc = "Field `VWUPDIE` writer - VWUPD Interrupt Enable"]
pub type VwupdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPIRSTIE` reader - eSPI_RST Interrupt Enable"]
pub type EspirstieR = crate::BitReader;
#[doc = "Field `ESPIRSTIE` writer - eSPI_RST Interrupt Enable"]
pub type EspirstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLTRSTIE` reader - PLTRST Interrupt Enable"]
pub type PltrstieR = crate::BitReader;
#[doc = "Field `PLTRSTIE` writer - PLTRST Interrupt Enable"]
pub type PltrstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMERRIE` reader - AMERR Interrupt Enable"]
pub type AmerrieR = crate::BitReader;
#[doc = "Field `AMERRIE` writer - AMERR Interrupt Enable"]
pub type AmerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMDONEIE` reader - AMDONE Interrupt Enable"]
pub type AmdoneieR = crate::BitReader;
#[doc = "Field `AMDONEIE` writer - AMDONE Interrupt Enable"]
pub type AmdoneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLNPRQSIE` reader - FLNPRQS Interrupt Enable"]
pub type FlnprqsieR = crate::BitReader;
#[doc = "Field `FLNPRQSIE` writer - FLNPRQS Interrupt Enable"]
pub type FlnprqsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMTXDONEIE` reader - BMTXDONE Interrupt Enable"]
pub type BmtxdoneieR = crate::BitReader;
#[doc = "Field `BMTXDONEIE` writer - BMTXDONE Interrupt Enable"]
pub type BmtxdoneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBMRXIE` reader - PBMRX Interrupt Enable"]
pub type PbmrxieR = crate::BitReader;
#[doc = "Field `PBMRXIE` writer - PBMRX Interrupt Enable"]
pub type PbmrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMSGRXIE` reader - PMSGRX Interrupt Enable"]
pub type PmsgrxieR = crate::BitReader;
#[doc = "Field `PMSGRXIE` writer - PMSGRX Interrupt Enable"]
pub type PmsgrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMBURSTERRIE` reader - BMBURSTERR Interrupt Enable"]
pub type BmbursterrieR = crate::BitReader;
#[doc = "Field `BMBURSTERRIE` writer - BMBURSTERR Interrupt Enable"]
pub type BmbursterrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMBURSTDONEIE` reader - BMBURSTDONE Interrupt Enable"]
pub type BmburstdoneieR = crate::BitReader;
#[doc = "Field `BMBURSTDONEIE` writer - BMBURSTDONE Interrupt Enable"]
pub type BmburstdoneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLPRTERRIE` reader - FLPRTERR Interrupt Enable"]
pub type FlprterrieR = crate::BitReader;
#[doc = "Field `FLPRTERRIE` writer - FLPRTERR Interrupt Enable"]
pub type FlprterrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAUTORDSTRIE` reader - FLAUTORDSTR Interrupt Enable"]
pub type FlautordstrieR = crate::BitReader;
#[doc = "Field `FLAUTORDSTRIE` writer - FLAUTORDSTR Interrupt Enable"]
pub type FlautordstrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAUTORDPNDIE` reader - FLAUTORDPND Interrupt Enable"]
pub type FlautordpndieR = crate::BitReader;
#[doc = "Field `FLAUTORDPNDIE` writer - FLAUTORDPND Interrupt Enable"]
pub type FlautordpndieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAUTORDQEMPIE` reader - FLAUTORDQEMP Interrupt Enable"]
pub type FlautordqempieR = crate::BitReader;
#[doc = "Field `FLAUTORDQEMPIE` writer - FLAUTORDQEMP Interrupt Enable"]
pub type FlautordqempieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAUTORDDISIE` reader - AUTO_RD_DIS_STS Interrupt Enable"]
pub type FlautorddisieR = crate::BitReader;
#[doc = "Field `FLAUTORDDISIE` writer - AUTO_RD_DIS_STS Interrupt Enable"]
pub type FlautorddisieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMWBURSTERRIE` reader - BMWRBURSTERR Interrupt Enable"]
pub type BmwbursterrieR = crate::BitReader;
#[doc = "Field `BMWBURSTERRIE` writer - BMWRBURSTERR Interrupt Enable"]
pub type BmwbursterrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMWBURSTDONEIE` reader - BMWBURSTDONE Interrupt Enable"]
pub type BmwburstdoneieR = crate::BitReader;
#[doc = "Field `BMWBURSTDONEIE` writer - BMWBURSTDONE Interrupt Enable"]
pub type BmwburstdoneieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IBRST Interrupt Enable"]
    #[inline(always)]
    pub fn ibrstie(&self) -> IbrstieR {
        IbrstieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CFGUPD Interrupt Enable"]
    #[inline(always)]
    pub fn cfgupdie(&self) -> CfgupdieR {
        CfgupdieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BERR Interrupt Enable"]
    #[inline(always)]
    pub fn berrie(&self) -> BerrieR {
        BerrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OOBRX Interrupt Enable"]
    #[inline(always)]
    pub fn oobrxie(&self) -> OobrxieR {
        OobrxieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASHRX Interrupt Enable"]
    #[inline(always)]
    pub fn flashrxie(&self) -> FlashrxieR {
        FlashrxieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLNACS Interrupt Enable"]
    #[inline(always)]
    pub fn flnacsie(&self) -> FlnacsieR {
        FlnacsieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PERACC Interrupt Enable"]
    #[inline(always)]
    pub fn peraccie(&self) -> PeraccieR {
        PeraccieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFRD Interrupt Enable"]
    #[inline(always)]
    pub fn dfrdie(&self) -> DfrdieR {
        DfrdieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VWUPD Interrupt Enable"]
    #[inline(always)]
    pub fn vwupdie(&self) -> VwupdieR {
        VwupdieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - eSPI_RST Interrupt Enable"]
    #[inline(always)]
    pub fn espirstie(&self) -> EspirstieR {
        EspirstieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLTRST Interrupt Enable"]
    #[inline(always)]
    pub fn pltrstie(&self) -> PltrstieR {
        PltrstieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - AMERR Interrupt Enable"]
    #[inline(always)]
    pub fn amerrie(&self) -> AmerrieR {
        AmerrieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AMDONE Interrupt Enable"]
    #[inline(always)]
    pub fn amdoneie(&self) -> AmdoneieR {
        AmdoneieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - FLNPRQS Interrupt Enable"]
    #[inline(always)]
    pub fn flnprqsie(&self) -> FlnprqsieR {
        FlnprqsieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BMTXDONE Interrupt Enable"]
    #[inline(always)]
    pub fn bmtxdoneie(&self) -> BmtxdoneieR {
        BmtxdoneieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PBMRX Interrupt Enable"]
    #[inline(always)]
    pub fn pbmrxie(&self) -> PbmrxieR {
        PbmrxieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PMSGRX Interrupt Enable"]
    #[inline(always)]
    pub fn pmsgrxie(&self) -> PmsgrxieR {
        PmsgrxieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - BMBURSTERR Interrupt Enable"]
    #[inline(always)]
    pub fn bmbursterrie(&self) -> BmbursterrieR {
        BmbursterrieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - BMBURSTDONE Interrupt Enable"]
    #[inline(always)]
    pub fn bmburstdoneie(&self) -> BmburstdoneieR {
        BmburstdoneieR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FLPRTERR Interrupt Enable"]
    #[inline(always)]
    pub fn flprterrie(&self) -> FlprterrieR {
        FlprterrieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLAUTORDSTR Interrupt Enable"]
    #[inline(always)]
    pub fn flautordstrie(&self) -> FlautordstrieR {
        FlautordstrieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FLAUTORDPND Interrupt Enable"]
    #[inline(always)]
    pub fn flautordpndie(&self) -> FlautordpndieR {
        FlautordpndieR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FLAUTORDQEMP Interrupt Enable"]
    #[inline(always)]
    pub fn flautordqempie(&self) -> FlautordqempieR {
        FlautordqempieR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AUTO_RD_DIS_STS Interrupt Enable"]
    #[inline(always)]
    pub fn flautorddisie(&self) -> FlautorddisieR {
        FlautorddisieR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - BMWRBURSTERR Interrupt Enable"]
    #[inline(always)]
    pub fn bmwbursterrie(&self) -> BmwbursterrieR {
        BmwbursterrieR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BMWBURSTDONE Interrupt Enable"]
    #[inline(always)]
    pub fn bmwburstdoneie(&self) -> BmwburstdoneieR {
        BmwburstdoneieR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPIIE")
            .field("ibrstie", &self.ibrstie())
            .field("cfgupdie", &self.cfgupdie())
            .field("berrie", &self.berrie())
            .field("oobrxie", &self.oobrxie())
            .field("flashrxie", &self.flashrxie())
            .field("flnacsie", &self.flnacsie())
            .field("peraccie", &self.peraccie())
            .field("dfrdie", &self.dfrdie())
            .field("vwupdie", &self.vwupdie())
            .field("espirstie", &self.espirstie())
            .field("pltrstie", &self.pltrstie())
            .field("amerrie", &self.amerrie())
            .field("amdoneie", &self.amdoneie())
            .field("flnprqsie", &self.flnprqsie())
            .field("bmtxdoneie", &self.bmtxdoneie())
            .field("pbmrxie", &self.pbmrxie())
            .field("pmsgrxie", &self.pmsgrxie())
            .field("bmbursterrie", &self.bmbursterrie())
            .field("bmburstdoneie", &self.bmburstdoneie())
            .field("flprterrie", &self.flprterrie())
            .field("flautordstrie", &self.flautordstrie())
            .field("flautordpndie", &self.flautordpndie())
            .field("flautordqempie", &self.flautordqempie())
            .field("flautorddisie", &self.flautorddisie())
            .field("bmwbursterrie", &self.bmwbursterrie())
            .field("bmwburstdoneie", &self.bmwburstdoneie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IBRST Interrupt Enable"]
    #[inline(always)]
    pub fn ibrstie(&mut self) -> IbrstieW<EspiieSpec> {
        IbrstieW::new(self, 0)
    }
    #[doc = "Bit 1 - CFGUPD Interrupt Enable"]
    #[inline(always)]
    pub fn cfgupdie(&mut self) -> CfgupdieW<EspiieSpec> {
        CfgupdieW::new(self, 1)
    }
    #[doc = "Bit 2 - BERR Interrupt Enable"]
    #[inline(always)]
    pub fn berrie(&mut self) -> BerrieW<EspiieSpec> {
        BerrieW::new(self, 2)
    }
    #[doc = "Bit 3 - OOBRX Interrupt Enable"]
    #[inline(always)]
    pub fn oobrxie(&mut self) -> OobrxieW<EspiieSpec> {
        OobrxieW::new(self, 3)
    }
    #[doc = "Bit 4 - FLASHRX Interrupt Enable"]
    #[inline(always)]
    pub fn flashrxie(&mut self) -> FlashrxieW<EspiieSpec> {
        FlashrxieW::new(self, 4)
    }
    #[doc = "Bit 5 - FLNACS Interrupt Enable"]
    #[inline(always)]
    pub fn flnacsie(&mut self) -> FlnacsieW<EspiieSpec> {
        FlnacsieW::new(self, 5)
    }
    #[doc = "Bit 6 - PERACC Interrupt Enable"]
    #[inline(always)]
    pub fn peraccie(&mut self) -> PeraccieW<EspiieSpec> {
        PeraccieW::new(self, 6)
    }
    #[doc = "Bit 7 - DFRD Interrupt Enable"]
    #[inline(always)]
    pub fn dfrdie(&mut self) -> DfrdieW<EspiieSpec> {
        DfrdieW::new(self, 7)
    }
    #[doc = "Bit 8 - VWUPD Interrupt Enable"]
    #[inline(always)]
    pub fn vwupdie(&mut self) -> VwupdieW<EspiieSpec> {
        VwupdieW::new(self, 8)
    }
    #[doc = "Bit 9 - eSPI_RST Interrupt Enable"]
    #[inline(always)]
    pub fn espirstie(&mut self) -> EspirstieW<EspiieSpec> {
        EspirstieW::new(self, 9)
    }
    #[doc = "Bit 10 - PLTRST Interrupt Enable"]
    #[inline(always)]
    pub fn pltrstie(&mut self) -> PltrstieW<EspiieSpec> {
        PltrstieW::new(self, 10)
    }
    #[doc = "Bit 15 - AMERR Interrupt Enable"]
    #[inline(always)]
    pub fn amerrie(&mut self) -> AmerrieW<EspiieSpec> {
        AmerrieW::new(self, 15)
    }
    #[doc = "Bit 16 - AMDONE Interrupt Enable"]
    #[inline(always)]
    pub fn amdoneie(&mut self) -> AmdoneieW<EspiieSpec> {
        AmdoneieW::new(self, 16)
    }
    #[doc = "Bit 18 - FLNPRQS Interrupt Enable"]
    #[inline(always)]
    pub fn flnprqsie(&mut self) -> FlnprqsieW<EspiieSpec> {
        FlnprqsieW::new(self, 18)
    }
    #[doc = "Bit 19 - BMTXDONE Interrupt Enable"]
    #[inline(always)]
    pub fn bmtxdoneie(&mut self) -> BmtxdoneieW<EspiieSpec> {
        BmtxdoneieW::new(self, 19)
    }
    #[doc = "Bit 20 - PBMRX Interrupt Enable"]
    #[inline(always)]
    pub fn pbmrxie(&mut self) -> PbmrxieW<EspiieSpec> {
        PbmrxieW::new(self, 20)
    }
    #[doc = "Bit 21 - PMSGRX Interrupt Enable"]
    #[inline(always)]
    pub fn pmsgrxie(&mut self) -> PmsgrxieW<EspiieSpec> {
        PmsgrxieW::new(self, 21)
    }
    #[doc = "Bit 22 - BMBURSTERR Interrupt Enable"]
    #[inline(always)]
    pub fn bmbursterrie(&mut self) -> BmbursterrieW<EspiieSpec> {
        BmbursterrieW::new(self, 22)
    }
    #[doc = "Bit 23 - BMBURSTDONE Interrupt Enable"]
    #[inline(always)]
    pub fn bmburstdoneie(&mut self) -> BmburstdoneieW<EspiieSpec> {
        BmburstdoneieW::new(self, 23)
    }
    #[doc = "Bit 25 - FLPRTERR Interrupt Enable"]
    #[inline(always)]
    pub fn flprterrie(&mut self) -> FlprterrieW<EspiieSpec> {
        FlprterrieW::new(self, 25)
    }
    #[doc = "Bit 26 - FLAUTORDSTR Interrupt Enable"]
    #[inline(always)]
    pub fn flautordstrie(&mut self) -> FlautordstrieW<EspiieSpec> {
        FlautordstrieW::new(self, 26)
    }
    #[doc = "Bit 27 - FLAUTORDPND Interrupt Enable"]
    #[inline(always)]
    pub fn flautordpndie(&mut self) -> FlautordpndieW<EspiieSpec> {
        FlautordpndieW::new(self, 27)
    }
    #[doc = "Bit 28 - FLAUTORDQEMP Interrupt Enable"]
    #[inline(always)]
    pub fn flautordqempie(&mut self) -> FlautordqempieW<EspiieSpec> {
        FlautordqempieW::new(self, 28)
    }
    #[doc = "Bit 29 - AUTO_RD_DIS_STS Interrupt Enable"]
    #[inline(always)]
    pub fn flautorddisie(&mut self) -> FlautorddisieW<EspiieSpec> {
        FlautorddisieW::new(self, 29)
    }
    #[doc = "Bit 30 - BMWRBURSTERR Interrupt Enable"]
    #[inline(always)]
    pub fn bmwbursterrie(&mut self) -> BmwbursterrieW<EspiieSpec> {
        BmwbursterrieW::new(self, 30)
    }
    #[doc = "Bit 31 - BMWBURSTDONE Interrupt Enable"]
    #[inline(always)]
    pub fn bmwburstdoneie(&mut self) -> BmwburstdoneieW<EspiieSpec> {
        BmwburstdoneieW::new(self, 31)
    }
}
#[doc = "eSPI Interrupt Enable Register (ESPIIE)\n\nYou can [`read`](crate::Reg::read) this register and get [`espiie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspiieSpec;
impl crate::RegisterSpec for EspiieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espiie::R`](R) reader structure"]
impl crate::Readable for EspiieSpec {}
#[doc = "`write(|w| ..)` method takes [`espiie::W`](W) writer structure"]
impl crate::Writable for EspiieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESPIIE to value 0"]
impl crate::Resettable for EspiieSpec {
    const RESET_VALUE: u32 = 0;
}
