#[doc = "Register `SPI_FL_CFG` reader"]
pub type R = crate::R<SpiFlCfgSpec>;
#[doc = "Register `SPI_FL_CFG` writer"]
pub type W = crate::W<SpiFlCfgSpec>;
#[doc = "Field `DN_WR_MODE` reader - Device 'n' Write Mode Select"]
pub type DnWrModeR = crate::FieldReader;
#[doc = "Field `DN_WR_MODE` writer - Device 'n' Write Mode Select"]
pub type DnWrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DN_RD_MODE` reader - Device 'n' Read Mode Select"]
pub type DnRdModeR = crate::FieldReader;
#[doc = "Field `DN_RD_MODE` writer - Device 'n' Read Mode Select"]
pub type DnRdModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - Device 'n' Write Mode Select"]
    #[inline(always)]
    pub fn dn_wr_mode(&self) -> DnWrModeR {
        DnWrModeR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Device 'n' Read Mode Select"]
    #[inline(always)]
    pub fn dn_rd_mode(&self) -> DnRdModeR {
        DnRdModeR::new((self.bits >> 6) & 3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_FL_CFG")
            .field("dn_wr_mode", &self.dn_wr_mode())
            .field("dn_rd_mode", &self.dn_rd_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - Device 'n' Write Mode Select"]
    #[inline(always)]
    pub fn dn_wr_mode(&mut self) -> DnWrModeW<SpiFlCfgSpec> {
        DnWrModeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Device 'n' Read Mode Select"]
    #[inline(always)]
    pub fn dn_rd_mode(&mut self) -> DnRdModeW<SpiFlCfgSpec> {
        DnRdModeW::new(self, 6)
    }
}
#[doc = "SPI Flash Configuration Register (SPI_FL_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiFlCfgSpec;
impl crate::RegisterSpec for SpiFlCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spi_fl_cfg::R`](R) reader structure"]
impl crate::Readable for SpiFlCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_fl_cfg::W`](W) writer structure"]
impl crate::Writable for SpiFlCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SPI_FL_CFG to value 0"]
impl crate::Resettable for SpiFlCfgSpec {
    const RESET_VALUE: u8 = 0;
}
