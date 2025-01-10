#[doc = "Register `DEVCNT` reader"]
pub type R = crate::R<DevcntSpec>;
#[doc = "Register `DEVCNT` writer"]
pub type W = crate::W<DevcntSpec>;
#[doc = "Field `F_TRIS_MODE` reader - SPI Flash TRI-STATE Mode"]
pub type FTrisModeR = crate::BitReader;
#[doc = "Field `F_TRIS_MODE` writer - SPI Flash TRI-STATE Mode"]
pub type FTrisModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIF_TYP_SEL` reader - Host Interface Type Select"]
pub type HifTypSelR = crate::FieldReader;
#[doc = "Field `HIF_TYP_SEL` writer - Host Interface Type Select"]
pub type HifTypSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_TYP_SEL` reader - SPI Interface Type Select"]
pub type SpiTypSelR = crate::FieldReader;
#[doc = "Field `SPI_TYP_SEL` writer - SPI Interface Type Select"]
pub type SpiTypSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F_SPI_TRIS` reader - SPI Flash Interface TRI-STATE"]
pub type FSpiTrisR = crate::BitReader;
#[doc = "Field `F_SPI_TRIS` writer - SPI Flash Interface TRI-STATE"]
pub type FSpiTrisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Flash TRI-STATE Mode"]
    #[inline(always)]
    pub fn f_tris_mode(&self) -> FTrisModeR {
        FTrisModeR::new((self.bits & 1) != 0)
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
    #[doc = "Bit 6 - SPI Flash Interface TRI-STATE"]
    #[inline(always)]
    pub fn f_spi_tris(&self) -> FSpiTrisR {
        FSpiTrisR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVCNT")
            .field("hif_typ_sel", &self.hif_typ_sel())
            .field("spi_typ_sel", &self.spi_typ_sel())
            .field("f_spi_tris", &self.f_spi_tris())
            .field("f_tris_mode", &self.f_tris_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI Flash TRI-STATE Mode"]
    #[inline(always)]
    pub fn f_tris_mode(&mut self) -> FTrisModeW<DevcntSpec> {
        FTrisModeW::new(self, 0)
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
    #[doc = "Bit 6 - SPI Flash Interface TRI-STATE"]
    #[inline(always)]
    pub fn f_spi_tris(&mut self) -> FSpiTrisW<DevcntSpec> {
        FSpiTrisW::new(self, 6)
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
#[doc = "`reset()` method sets DEVCNT to value 0xc0"]
impl crate::Resettable for DevcntSpec {
    const RESET_VALUE: u8 = 0xc0;
}
