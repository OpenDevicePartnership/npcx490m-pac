#[doc = "Register `FIU_EXT_CFG` reader"]
pub type R = crate::R<FiuExtCfgSpec>;
#[doc = "Register `FIU_EXT_CFG` writer"]
pub type W = crate::W<FiuExtCfgSpec>;
#[doc = "Field `DN_SET_RD_CMD_EN` reader - Device 'n' Set Read Command Enable"]
pub type DnSetRdCmdEnR = crate::BitReader;
#[doc = "Field `DN_SET_RD_CMD_EN` writer - Device 'n' Set Read Command Enable"]
pub type DnSetRdCmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DN_SET_DMM_EN` reader - Device 'n' Set Dummy Enable"]
pub type DnSetDmmEnR = crate::BitReader;
#[doc = "Field `DN_SET_DMM_EN` writer - Device 'n' Set Dummy Enable"]
pub type DnSetDmmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_2DEV` reader - SPI Bus, Two Devices Enable"]
pub type Spi2devR = crate::BitReader;
#[doc = "Field `SPI_2DEV` writer - SPI Bus, Two Devices Enable"]
pub type Spi2devW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_LO_DEV_NUM` reader - SPI0 Low Device Number"]
pub type SpiLoDevNumR = crate::BitReader;
#[doc = "Field `SPI_LO_DEV_NUM` writer - SPI0 Low Device Number"]
pub type SpiLoDevNumW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Device 'n' Set Read Command Enable"]
    #[inline(always)]
    pub fn dn_set_rd_cmd_en(&self) -> DnSetRdCmdEnR {
        DnSetRdCmdEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Device 'n' Set Dummy Enable"]
    #[inline(always)]
    pub fn dn_set_dmm_en(&self) -> DnSetDmmEnR {
        DnSetDmmEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Bus, Two Devices Enable"]
    #[inline(always)]
    pub fn spi_2dev(&self) -> Spi2devR {
        Spi2devR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI0 Low Device Number"]
    #[inline(always)]
    pub fn spi_lo_dev_num(&self) -> SpiLoDevNumR {
        SpiLoDevNumR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIU_EXT_CFG")
            .field("dn_set_rd_cmd_en", &self.dn_set_rd_cmd_en())
            .field("dn_set_dmm_en", &self.dn_set_dmm_en())
            .field("spi_2dev", &self.spi_2dev())
            .field("spi_lo_dev_num", &self.spi_lo_dev_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Device 'n' Set Read Command Enable"]
    #[inline(always)]
    pub fn dn_set_rd_cmd_en(&mut self) -> DnSetRdCmdEnW<FiuExtCfgSpec> {
        DnSetRdCmdEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Device 'n' Set Dummy Enable"]
    #[inline(always)]
    pub fn dn_set_dmm_en(&mut self) -> DnSetDmmEnW<FiuExtCfgSpec> {
        DnSetDmmEnW::new(self, 2)
    }
    #[doc = "Bit 6 - SPI Bus, Two Devices Enable"]
    #[inline(always)]
    pub fn spi_2dev(&mut self) -> Spi2devW<FiuExtCfgSpec> {
        Spi2devW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI0 Low Device Number"]
    #[inline(always)]
    pub fn spi_lo_dev_num(&mut self) -> SpiLoDevNumW<FiuExtCfgSpec> {
        SpiLoDevNumW::new(self, 7)
    }
}
#[doc = "FIU Extended Configuration Register (FIU_EXT_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`fiu_ext_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiu_ext_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiuExtCfgSpec;
impl crate::RegisterSpec for FiuExtCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fiu_ext_cfg::R`](R) reader structure"]
impl crate::Readable for FiuExtCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fiu_ext_cfg::W`](W) writer structure"]
impl crate::Writable for FiuExtCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FIU_EXT_CFG to value 0"]
impl crate::Resettable for FiuExtCfgSpec {
    const RESET_VALUE: u8 = 0;
}
