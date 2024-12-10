#[doc = "Register `DEVCNT` reader"]
pub type R = crate::R<DevcntSpec>;
#[doc = "Register `DEVCNT` writer"]
pub type W = crate::W<DevcntSpec>;
#[doc = "Field `SHD_TRIS_MODE` reader - Shared Flash SPI TRI-STATE Mode"]
pub type ShdTrisModeR = crate::BitReader;
#[doc = "Field `SHD_TRIS_MODE` writer - Shared Flash SPI TRI-STATE Mode"]
pub type ShdTrisModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIF_TYP_SEL` reader - Host Interface Type Select"]
pub type HifTypSelR = crate::FieldReader;
#[doc = "Field `HIF_TYP_SEL` writer - Host Interface Type Select"]
pub type HifTypSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_TYP_SEL` reader - SPI Interface Type Select"]
pub type SpiTypSelR = crate::FieldReader;
#[doc = "Field `SPI_TYP_SEL` writer - SPI Interface Type Select"]
pub type SpiTypSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SHD_SPI_TRIS` reader - Shared Flash SPI Interface TRI-STATE"]
pub type ShdSpiTrisR = crate::BitReader;
#[doc = "Field `SHD_SPI_TRIS` writer - Shared Flash SPI Interface TRI-STATE"]
pub type ShdSpiTrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT_SPI_TRIS` reader - Private Flash SPI Interface TRI-STATE"]
pub type PvtSpiTrisR = crate::BitReader;
#[doc = "Field `PVT_SPI_TRIS` writer - Private Flash SPI Interface TRI-STATE"]
pub type PvtSpiTrisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shared Flash SPI TRI-STATE Mode"]
    #[inline(always)]
    pub fn shd_tris_mode(&self) -> ShdTrisModeR {
        ShdTrisModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Host Interface Type Select"]
    #[inline(always)]
    pub fn hif_typ_sel(&self) -> HifTypSelR {
        HifTypSelR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - SPI Interface Type Select"]
    #[inline(always)]
    pub fn spi_typ_sel(&self) -> SpiTypSelR {
        SpiTypSelR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Shared Flash SPI Interface TRI-STATE"]
    #[inline(always)]
    pub fn shd_spi_tris(&self) -> ShdSpiTrisR {
        ShdSpiTrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Private Flash SPI Interface TRI-STATE"]
    #[inline(always)]
    pub fn pvt_spi_tris(&self) -> PvtSpiTrisR {
        PvtSpiTrisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVCNT")
            .field("shd_tris_mode", &self.shd_tris_mode())
            .field("hif_typ_sel", &self.hif_typ_sel())
            .field("spi_typ_sel", &self.spi_typ_sel())
            .field("shd_spi_tris", &self.shd_spi_tris())
            .field("pvt_spi_tris", &self.pvt_spi_tris())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Shared Flash SPI TRI-STATE Mode"]
    #[inline(always)]
    pub fn shd_tris_mode(&mut self) -> ShdTrisModeW<DevcntSpec> {
        ShdTrisModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Host Interface Type Select"]
    #[inline(always)]
    pub fn hif_typ_sel(&mut self) -> HifTypSelW<DevcntSpec> {
        HifTypSelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SPI Interface Type Select"]
    #[inline(always)]
    pub fn spi_typ_sel(&mut self) -> SpiTypSelW<DevcntSpec> {
        SpiTypSelW::new(self, 4)
    }
    #[doc = "Bit 6 - Shared Flash SPI Interface TRI-STATE"]
    #[inline(always)]
    pub fn shd_spi_tris(&mut self) -> ShdSpiTrisW<DevcntSpec> {
        ShdSpiTrisW::new(self, 6)
    }
    #[doc = "Bit 7 - Private Flash SPI Interface TRI-STATE"]
    #[inline(always)]
    pub fn pvt_spi_tris(&mut self) -> PvtSpiTrisW<DevcntSpec> {
        PvtSpiTrisW::new(self, 7)
    }
}
#[doc = "Device Control Register (DEVCNT)\n\nYou can [`read`](crate::Reg::read) this register and get [`devcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevcntSpec;
impl crate::RegisterSpec for DevcntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devcnt::R`](R) reader structure"]
impl crate::Readable for DevcntSpec {}
#[doc = "`write(|w| ..)` method takes [`devcnt::W`](W) writer structure"]
impl crate::Writable for DevcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVCNT to value 0"]
impl crate::Resettable for DevcntSpec {
    const RESET_VALUE: u8 = 0;
}
