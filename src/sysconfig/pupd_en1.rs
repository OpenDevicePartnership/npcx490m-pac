#[doc = "Register `PUPD_EN1` reader"]
pub type R = crate::R<PupdEn1Spec>;
#[doc = "Register `PUPD_EN1` writer"]
pub type W = crate::W<PupdEn1Spec>;
#[doc = "Field `I2C6_1_PUE` reader - SMBus/I2C Module 6, Bus 1 Pull-Up Enable"]
pub type I2c6_1PueR = crate::BitReader;
#[doc = "Field `I2C6_1_PUE` writer - SMBus/I2C Module 6, Bus 1 Pull-Up Enable"]
pub type I2c6_1PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C5_1_PUE` reader - SMBus/I2C Module 5, Bus 1 Pull-Up Enable"]
pub type I2c5_1PueR = crate::BitReader;
#[doc = "Field `I2C5_1_PUE` writer - SMBus/I2C Module 5, Bus 1 Pull-Up Enable"]
pub type I2c5_1PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_1_PUE` reader - SMBus/I2C Module 4, Bus 1 Pull-Up Enable"]
pub type I2c4_1PueR = crate::BitReader;
#[doc = "Field `I2C4_1_PUE` writer - SMBus/I2C Module 4, Bus 1 Pull-Up Enable"]
pub type I2c4_1PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C7_1_PUE` reader - SMBus/I2C Module 7, Bus 1 Pull-Up Enable"]
pub type I2c7_1PueR = crate::BitReader;
#[doc = "Field `I2C7_1_PUE` writer - SMBus/I2C Module 7, Bus 1 Pull-Up Enable"]
pub type I2c7_1PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHI_PUD_EN` reader - SHI Interface Pull-Up/Down Enable"]
pub type ShiPudEnR = crate::BitReader;
#[doc = "Field `SHI_PUD_EN` writer - SHI Interface Pull-Up/Down Enable"]
pub type ShiPudEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIP_PD_EN` reader - SPI Peripheral Pull-Down Enable"]
pub type SpipPdEnR = crate::BitReader;
#[doc = "Field `SPIP_PD_EN` writer - SPI Peripheral Pull-Down Enable"]
pub type SpipPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_SPI_PUD_EN` reader - SPI Flash Pull-Up/Down Enable"]
pub type FSpiPudEnR = crate::BitReader;
#[doc = "Field `F_SPI_PUD_EN` writer - SPI Flash Pull-Up/Down Enable"]
pub type FSpiPudEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMBus/I2C Module 6, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c6_1_pue(&self) -> I2c6_1PueR {
        I2c6_1PueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 5, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c5_1_pue(&self) -> I2c5_1PueR {
        I2c5_1PueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 4, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c4_1_pue(&self) -> I2c4_1PueR {
        I2c4_1PueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 7, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c7_1_pue(&self) -> I2c7_1PueR {
        I2c7_1PueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SHI Interface Pull-Up/Down Enable"]
    #[inline(always)]
    pub fn shi_pud_en(&self) -> ShiPudEnR {
        ShiPudEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Peripheral Pull-Down Enable"]
    #[inline(always)]
    pub fn spip_pd_en(&self) -> SpipPdEnR {
        SpipPdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Flash Pull-Up/Down Enable"]
    #[inline(always)]
    pub fn f_spi_pud_en(&self) -> FSpiPudEnR {
        FSpiPudEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPD_EN1")
            .field("i2c6_1_pue", &self.i2c6_1_pue())
            .field("i2c5_1_pue", &self.i2c5_1_pue())
            .field("i2c4_1_pue", &self.i2c4_1_pue())
            .field("i2c7_1_pue", &self.i2c7_1_pue())
            .field("shi_pud_en", &self.shi_pud_en())
            .field("spip_pd_en", &self.spip_pd_en())
            .field("f_spi_pud_en", &self.f_spi_pud_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMBus/I2C Module 6, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c6_1_pue(&mut self) -> I2c6_1PueW<PupdEn1Spec> {
        I2c6_1PueW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 5, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c5_1_pue(&mut self) -> I2c5_1PueW<PupdEn1Spec> {
        I2c5_1PueW::new(self, 1)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 4, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c4_1_pue(&mut self) -> I2c4_1PueW<PupdEn1Spec> {
        I2c4_1PueW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 7, Bus 1 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c7_1_pue(&mut self) -> I2c7_1PueW<PupdEn1Spec> {
        I2c7_1PueW::new(self, 3)
    }
    #[doc = "Bit 4 - SHI Interface Pull-Up/Down Enable"]
    #[inline(always)]
    pub fn shi_pud_en(&mut self) -> ShiPudEnW<PupdEn1Spec> {
        ShiPudEnW::new(self, 4)
    }
    #[doc = "Bit 6 - SPI Peripheral Pull-Down Enable"]
    #[inline(always)]
    pub fn spip_pd_en(&mut self) -> SpipPdEnW<PupdEn1Spec> {
        SpipPdEnW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Flash Pull-Up/Down Enable"]
    #[inline(always)]
    pub fn f_spi_pud_en(&mut self) -> FSpiPudEnW<PupdEn1Spec> {
        FSpiPudEnW::new(self, 7)
    }
}
#[doc = "Pull-Up/Pull-Down Enable 1 Register (PUPD_EN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`pupd_en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupd_en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PupdEn1Spec;
impl crate::RegisterSpec for PupdEn1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pupd_en1::R`](R) reader structure"]
impl crate::Readable for PupdEn1Spec {}
#[doc = "`write(|w| ..)` method takes [`pupd_en1::W`](W) writer structure"]
impl crate::Writable for PupdEn1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PUPD_EN1 to value 0"]
impl crate::Resettable for PupdEn1Spec {
    const RESET_VALUE: u8 = 0;
}
