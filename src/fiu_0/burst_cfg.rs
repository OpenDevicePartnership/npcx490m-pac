#[doc = "Register `BURST_CFG` reader"]
pub type R = crate::R<BurstCfgSpec>;
#[doc = "Register `BURST_CFG` writer"]
pub type W = crate::W<BurstCfgSpec>;
#[doc = "Field `DMA_REQ_EN` reader - DMA Request Enable"]
pub type DmaReqEnR = crate::BitReader;
#[doc = "Field `DMA_REQ_EN` writer - DMA Request Enable"]
pub type DmaReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLIM_BURST` reader - Unlimited Burst"]
pub type UnlimBurstR = crate::BitReader;
#[doc = "Field `UNLIM_BURST` writer - Unlimited Burst"]
pub type UnlimBurstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DEV_SEL` reader - SPI Device Select"]
pub type SpiDevSelR = crate::FieldReader;
#[doc = "Field `SPI_DEV_SEL` writer - SPI Device Select"]
pub type SpiDevSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS0_AUTO_EN` reader - F_CS0 Auto-Assertion Enable"]
pub type Cs0AutoEnR = crate::BitReader;
#[doc = "Field `CS0_AUTO_EN` writer - F_CS0 Auto-Assertion Enable"]
pub type Cs0AutoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1_AUTO_EN` reader - F_CS1 Auto-Assertion Enable"]
pub type Cs1AutoEnR = crate::BitReader;
#[doc = "Field `CS1_AUTO_EN` writer - F_CS1 Auto-Assertion Enable"]
pub type Cs1AutoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - DMA Request Enable"]
    #[inline(always)]
    pub fn dma_req_en(&self) -> DmaReqEnR {
        DmaReqEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unlimited Burst"]
    #[inline(always)]
    pub fn unlim_burst(&self) -> UnlimBurstR {
        UnlimBurstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SPI Device Select"]
    #[inline(always)]
    pub fn spi_dev_sel(&self) -> SpiDevSelR {
        SpiDevSelR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - F_CS0 Auto-Assertion Enable"]
    #[inline(always)]
    pub fn cs0_auto_en(&self) -> Cs0AutoEnR {
        Cs0AutoEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - F_CS1 Auto-Assertion Enable"]
    #[inline(always)]
    pub fn cs1_auto_en(&self) -> Cs1AutoEnR {
        Cs1AutoEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BURST_CFG")
            .field("dma_req_en", &self.dma_req_en())
            .field("unlim_burst", &self.unlim_burst())
            .field("spi_dev_sel", &self.spi_dev_sel())
            .field("cs0_auto_en", &self.cs0_auto_en())
            .field("cs1_auto_en", &self.cs1_auto_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - DMA Request Enable"]
    #[inline(always)]
    pub fn dma_req_en(&mut self) -> DmaReqEnW<BurstCfgSpec> {
        DmaReqEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Unlimited Burst"]
    #[inline(always)]
    pub fn unlim_burst(&mut self) -> UnlimBurstW<BurstCfgSpec> {
        UnlimBurstW::new(self, 3)
    }
    #[doc = "Bits 4:5 - SPI Device Select"]
    #[inline(always)]
    pub fn spi_dev_sel(&mut self) -> SpiDevSelW<BurstCfgSpec> {
        SpiDevSelW::new(self, 4)
    }
    #[doc = "Bit 6 - F_CS0 Auto-Assertion Enable"]
    #[inline(always)]
    pub fn cs0_auto_en(&mut self) -> Cs0AutoEnW<BurstCfgSpec> {
        Cs0AutoEnW::new(self, 6)
    }
    #[doc = "Bit 7 - F_CS1 Auto-Assertion Enable"]
    #[inline(always)]
    pub fn cs1_auto_en(&mut self) -> Cs1AutoEnW<BurstCfgSpec> {
        Cs1AutoEnW::new(self, 7)
    }
}
#[doc = "Burst Configuration Register (BURST_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`burst_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`burst_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BurstCfgSpec;
impl crate::RegisterSpec for BurstCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`burst_cfg::R`](R) reader structure"]
impl crate::Readable for BurstCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`burst_cfg::W`](W) writer structure"]
impl crate::Writable for BurstCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BURST_CFG to value 0"]
impl crate::Resettable for BurstCfgSpec {
    const RESET_VALUE: u8 = 0;
}
