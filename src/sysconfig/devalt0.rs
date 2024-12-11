#[doc = "Register `DEVALT0` reader"]
pub type R = crate::R<Devalt0Spec>;
#[doc = "Register `DEVALT0` writer"]
pub type W = crate::W<Devalt0Spec>;
#[doc = "Field `SPIP_SL` reader - SPI Peripheral Interface-Select"]
pub type SpipSlR = crate::BitReader;
#[doc = "Field `SPIP_SL` writer - SPI Peripheral Interface-Select"]
pub type SpipSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT_SPI_CS1` reader - Private SPI Flash Chip-Select 1 Select"]
pub type PvtSpiCs1R = crate::BitReader;
#[doc = "Field `PVT_SPI_CS1` writer - Private SPI Flash Chip-Select 1 Select"]
pub type PvtSpiCs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT_SPI_QUAD` reader - Private SPI Flash Quad Interface-Select"]
pub type PvtSpiQuadR = crate::BitReader;
#[doc = "Field `PVT_SPI_QUAD` writer - Private SPI Flash Quad Interface-Select"]
pub type PvtSpiQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_PVT_SPI` reader - No Private SPI Flash-Select"]
pub type NoPvtSpiR = crate::BitReader;
#[doc = "Field `NO_PVT_SPI` writer - No Private SPI Flash-Select"]
pub type NoPvtSpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHD_SPI_CS1` reader - Shared SPI Flash Chip-Select 1"]
pub type ShdSpiCs1R = crate::BitReader;
#[doc = "Field `SHD_SPI_CS1` writer - Shared SPI Flash Chip-Select 1"]
pub type ShdSpiCs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHD_SPI_QUAD` reader - Shared SPI Flash Quad Interface-Select"]
pub type ShdSpiQuadR = crate::BitReader;
#[doc = "Field `SHD_SPI_QUAD` writer - Shared SPI Flash Quad Interface-Select"]
pub type ShdSpiQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_SHD_SPI` reader - No Shared SPI Flash-Select"]
pub type NoShdSpiR = crate::BitReader;
#[doc = "Field `NO_SHD_SPI` writer - No Shared SPI Flash-Select"]
pub type NoShdSpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Peripheral Interface-Select"]
    #[inline(always)]
    pub fn spip_sl(&self) -> SpipSlR {
        SpipSlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Private SPI Flash Chip-Select 1 Select"]
    #[inline(always)]
    pub fn pvt_spi_cs1(&self) -> PvtSpiCs1R {
        PvtSpiCs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Private SPI Flash Quad Interface-Select"]
    #[inline(always)]
    pub fn pvt_spi_quad(&self) -> PvtSpiQuadR {
        PvtSpiQuadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Private SPI Flash-Select"]
    #[inline(always)]
    pub fn no_pvt_spi(&self) -> NoPvtSpiR {
        NoPvtSpiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shared SPI Flash Chip-Select 1"]
    #[inline(always)]
    pub fn shd_spi_cs1(&self) -> ShdSpiCs1R {
        ShdSpiCs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Shared SPI Flash Quad Interface-Select"]
    #[inline(always)]
    pub fn shd_spi_quad(&self) -> ShdSpiQuadR {
        ShdSpiQuadR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Shared SPI Flash-Select"]
    #[inline(always)]
    pub fn no_shd_spi(&self) -> NoShdSpiR {
        NoShdSpiR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT0")
            .field("spip_sl", &self.spip_sl())
            .field("pvt_spi_cs1", &self.pvt_spi_cs1())
            .field("pvt_spi_quad", &self.pvt_spi_quad())
            .field("no_pvt_spi", &self.no_pvt_spi())
            .field("shd_spi_cs1", &self.shd_spi_cs1())
            .field("shd_spi_quad", &self.shd_spi_quad())
            .field("no_shd_spi", &self.no_shd_spi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI Peripheral Interface-Select"]
    #[inline(always)]
    pub fn spip_sl(&mut self) -> SpipSlW<Devalt0Spec> {
        SpipSlW::new(self, 0)
    }
    #[doc = "Bit 1 - Private SPI Flash Chip-Select 1 Select"]
    #[inline(always)]
    pub fn pvt_spi_cs1(&mut self) -> PvtSpiCs1W<Devalt0Spec> {
        PvtSpiCs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Private SPI Flash Quad Interface-Select"]
    #[inline(always)]
    pub fn pvt_spi_quad(&mut self) -> PvtSpiQuadW<Devalt0Spec> {
        PvtSpiQuadW::new(self, 2)
    }
    #[doc = "Bit 3 - No Private SPI Flash-Select"]
    #[inline(always)]
    pub fn no_pvt_spi(&mut self) -> NoPvtSpiW<Devalt0Spec> {
        NoPvtSpiW::new(self, 3)
    }
    #[doc = "Bit 4 - Shared SPI Flash Chip-Select 1"]
    #[inline(always)]
    pub fn shd_spi_cs1(&mut self) -> ShdSpiCs1W<Devalt0Spec> {
        ShdSpiCs1W::new(self, 4)
    }
    #[doc = "Bit 6 - Shared SPI Flash Quad Interface-Select"]
    #[inline(always)]
    pub fn shd_spi_quad(&mut self) -> ShdSpiQuadW<Devalt0Spec> {
        ShdSpiQuadW::new(self, 6)
    }
    #[doc = "Bit 7 - No Shared SPI Flash-Select"]
    #[inline(always)]
    pub fn no_shd_spi(&mut self) -> NoShdSpiW<Devalt0Spec> {
        NoShdSpiW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 0 Register (DEVALT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt0Spec;
impl crate::RegisterSpec for Devalt0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt0::R`](R) reader structure"]
impl crate::Readable for Devalt0Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt0::W`](W) writer structure"]
impl crate::Writable for Devalt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT0 to value 0"]
impl crate::Resettable for Devalt0Spec {
    const RESET_VALUE: u8 = 0;
}
