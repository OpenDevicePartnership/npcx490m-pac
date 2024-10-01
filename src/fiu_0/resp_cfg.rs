#[doc = "Register `RESP_CFG` reader"]
pub type R = crate::R<RespCfgSpec>;
#[doc = "Register `RESP_CFG` writer"]
pub type W = crate::W<RespCfgSpec>;
#[doc = "Field `SPI_WR_EN` reader - SPI Write Enable"]
pub type SpiWrEnR = crate::BitReader;
#[doc = "Field `SPI_WR_EN` writer - SPI Write Enable"]
pub type SpiWrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU_DRV1` reader - FIU IO2, IO3 Drive High"]
pub type FiuDrv1R = crate::BitReader;
#[doc = "Field `FIU_DRV1` writer - FIU IO2, IO3 Drive High"]
pub type FiuDrv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DN_QUAD_EN` reader - Device 'n' Quad Enable"]
pub type DnQuadEnR = crate::BitReader;
#[doc = "Field `DN_QUAD_EN` writer - Device 'n' Quad Enable"]
pub type DnQuadEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SPI Write Enable"]
    #[inline(always)]
    pub fn spi_wr_en(&self) -> SpiWrEnR {
        SpiWrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIU IO2, IO3 Drive High"]
    #[inline(always)]
    pub fn fiu_drv1(&self) -> FiuDrv1R {
        FiuDrv1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Device 'n' Quad Enable"]
    #[inline(always)]
    pub fn dn_quad_en(&self) -> DnQuadEnR {
        DnQuadEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP_CFG")
            .field("spi_wr_en", &self.spi_wr_en())
            .field("fiu_drv1", &self.fiu_drv1())
            .field("dn_quad_en", &self.dn_quad_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - SPI Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_wr_en(&mut self) -> SpiWrEnW<RespCfgSpec> {
        SpiWrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - FIU IO2, IO3 Drive High"]
    #[inline(always)]
    #[must_use]
    pub fn fiu_drv1(&mut self) -> FiuDrv1W<RespCfgSpec> {
        FiuDrv1W::new(self, 2)
    }
    #[doc = "Bit 3 - Device 'n' Quad Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dn_quad_en(&mut self) -> DnQuadEnW<RespCfgSpec> {
        DnQuadEnW::new(self, 3)
    }
}
#[doc = "FIU Response Configuration Register (RESP_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`resp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespCfgSpec;
impl crate::RegisterSpec for RespCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`resp_cfg::R`](R) reader structure"]
impl crate::Readable for RespCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`resp_cfg::W`](W) writer structure"]
impl crate::Writable for RespCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RESP_CFG to value 0"]
impl crate::Resettable for RespCfgSpec {
    const RESET_VALUE: u8 = 0;
}
