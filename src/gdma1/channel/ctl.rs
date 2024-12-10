#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `GDMAEN` reader - GDMA Enable"]
pub type GdmaenR = crate::BitReader;
#[doc = "Field `GDMAEN` writer - GDMA Enable"]
pub type GdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPD` reader - GDMA Power-Down"]
pub type GpdR = crate::BitReader;
#[doc = "Field `GPD` writer - GDMA Power-Down"]
pub type GpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMAMS` reader - GDMA Mode Select"]
pub type GdmamsR = crate::FieldReader;
#[doc = "Field `GDMAMS` writer - GDMA Mode Select"]
pub type GdmamsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DADIR` reader - Destination Address Direction"]
pub type DadirR = crate::BitReader;
#[doc = "Field `DADIR` writer - Destination Address Direction"]
pub type DadirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADIR` reader - Source Address Direction"]
pub type SadirR = crate::BitReader;
#[doc = "Field `SADIR` writer - Source Address Direction"]
pub type SadirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAFIX` reader - Destination Address Fixed"]
pub type DafixR = crate::BitReader;
#[doc = "Field `DAFIX` writer - Destination Address Fixed"]
pub type DafixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAFIX` reader - Source Address Fixed"]
pub type SafixR = crate::BitReader;
#[doc = "Field `SAFIX` writer - Source Address Fixed"]
pub type SafixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIEN` reader - Stop Interrupt Enable"]
pub type SienR = crate::BitReader;
#[doc = "Field `SIEN` writer - Stop Interrupt Enable"]
pub type SienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BME` reader - Burst Mode Enable"]
pub type BmeR = crate::BitReader;
#[doc = "Field `BME` writer - Burst Mode Enable"]
pub type BmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWS` reader - Transfer Width Select"]
pub type TwsR = crate::FieldReader;
#[doc = "Field `TWS` writer - Transfer Width Select"]
pub type TwsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPS` reader - GDMA Power Save"]
pub type GpsR = crate::BitReader;
#[doc = "Field `GPS` writer - GDMA Power Save"]
pub type GpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Demand Mode"]
pub type DmR = crate::BitReader;
#[doc = "Field `DM` writer - Demand Mode"]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTREQ` reader - Software Triggered GDMA Request"]
pub type SoftreqR = crate::BitReader;
#[doc = "Field `SOFTREQ` writer - Software Triggered GDMA Request"]
pub type SoftreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Terminal Count"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Terminal Count"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMDAFIX` reader - Burst Mode Destination Address Fixed"]
pub type BmdafixR = crate::BitReader;
#[doc = "Field `BMDAFIX` writer - Burst Mode Destination Address Fixed"]
pub type BmdafixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMSAFIX` reader - Burst Mode Source Address Fixed"]
pub type BmsafixR = crate::BitReader;
#[doc = "Field `BMSAFIX` writer - Burst Mode Source Address Fixed"]
pub type BmsafixW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GDMA Enable"]
    #[inline(always)]
    pub fn gdmaen(&self) -> GdmaenR {
        GdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GDMA Power-Down"]
    #[inline(always)]
    pub fn gpd(&self) -> GpdR {
        GpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GDMA Mode Select"]
    #[inline(always)]
    pub fn gdmams(&self) -> GdmamsR {
        GdmamsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Destination Address Direction"]
    #[inline(always)]
    pub fn dadir(&self) -> DadirR {
        DadirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source Address Direction"]
    #[inline(always)]
    pub fn sadir(&self) -> SadirR {
        SadirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Destination Address Fixed"]
    #[inline(always)]
    pub fn dafix(&self) -> DafixR {
        DafixR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source Address Fixed"]
    #[inline(always)]
    pub fn safix(&self) -> SafixR {
        SafixR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn sien(&self) -> SienR {
        SienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Burst Mode Enable"]
    #[inline(always)]
    pub fn bme(&self) -> BmeR {
        BmeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Transfer Width Select"]
    #[inline(always)]
    pub fn tws(&self) -> TwsR {
        TwsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - GDMA Power Save"]
    #[inline(always)]
    pub fn gps(&self) -> GpsR {
        GpsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Demand Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Triggered GDMA Request"]
    #[inline(always)]
    pub fn softreq(&self) -> SoftreqR {
        SoftreqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Terminal Count"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - Burst Mode Destination Address Fixed"]
    #[inline(always)]
    pub fn bmdafix(&self) -> BmdafixR {
        BmdafixR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Burst Mode Source Address Fixed"]
    #[inline(always)]
    pub fn bmsafix(&self) -> BmsafixR {
        BmsafixR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("gdmaen", &self.gdmaen())
            .field("gpd", &self.gpd())
            .field("gdmams", &self.gdmams())
            .field("dadir", &self.dadir())
            .field("sadir", &self.sadir())
            .field("dafix", &self.dafix())
            .field("safix", &self.safix())
            .field("sien", &self.sien())
            .field("bme", &self.bme())
            .field("tws", &self.tws())
            .field("gps", &self.gps())
            .field("dm", &self.dm())
            .field("softreq", &self.softreq())
            .field("tc", &self.tc())
            .field("bmdafix", &self.bmdafix())
            .field("bmsafix", &self.bmsafix())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GDMA Enable"]
    #[inline(always)]
    pub fn gdmaen(&mut self) -> GdmaenW<CtlSpec> {
        GdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - GDMA Power-Down"]
    #[inline(always)]
    pub fn gpd(&mut self) -> GpdW<CtlSpec> {
        GpdW::new(self, 1)
    }
    #[doc = "Bits 2:3 - GDMA Mode Select"]
    #[inline(always)]
    pub fn gdmams(&mut self) -> GdmamsW<CtlSpec> {
        GdmamsW::new(self, 2)
    }
    #[doc = "Bit 4 - Destination Address Direction"]
    #[inline(always)]
    pub fn dadir(&mut self) -> DadirW<CtlSpec> {
        DadirW::new(self, 4)
    }
    #[doc = "Bit 5 - Source Address Direction"]
    #[inline(always)]
    pub fn sadir(&mut self) -> SadirW<CtlSpec> {
        SadirW::new(self, 5)
    }
    #[doc = "Bit 6 - Destination Address Fixed"]
    #[inline(always)]
    pub fn dafix(&mut self) -> DafixW<CtlSpec> {
        DafixW::new(self, 6)
    }
    #[doc = "Bit 7 - Source Address Fixed"]
    #[inline(always)]
    pub fn safix(&mut self) -> SafixW<CtlSpec> {
        SafixW::new(self, 7)
    }
    #[doc = "Bit 8 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn sien(&mut self) -> SienW<CtlSpec> {
        SienW::new(self, 8)
    }
    #[doc = "Bit 9 - Burst Mode Enable"]
    #[inline(always)]
    pub fn bme(&mut self) -> BmeW<CtlSpec> {
        BmeW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Transfer Width Select"]
    #[inline(always)]
    pub fn tws(&mut self) -> TwsW<CtlSpec> {
        TwsW::new(self, 12)
    }
    #[doc = "Bit 14 - GDMA Power Save"]
    #[inline(always)]
    pub fn gps(&mut self) -> GpsW<CtlSpec> {
        GpsW::new(self, 14)
    }
    #[doc = "Bit 15 - Demand Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DmW<CtlSpec> {
        DmW::new(self, 15)
    }
    #[doc = "Bit 16 - Software Triggered GDMA Request"]
    #[inline(always)]
    pub fn softreq(&mut self) -> SoftreqW<CtlSpec> {
        SoftreqW::new(self, 16)
    }
    #[doc = "Bit 18 - Terminal Count"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<CtlSpec> {
        TcW::new(self, 18)
    }
    #[doc = "Bit 30 - Burst Mode Destination Address Fixed"]
    #[inline(always)]
    pub fn bmdafix(&mut self) -> BmdafixW<CtlSpec> {
        BmdafixW::new(self, 30)
    }
    #[doc = "Bit 31 - Burst Mode Source Address Fixed"]
    #[inline(always)]
    pub fn bmsafix(&mut self) -> BmsafixW<CtlSpec> {
        BmsafixW::new(self, 31)
    }
}
#[doc = "Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
