#[doc = "Register `DEVALT0` reader"]
pub type R = crate::R<Devalt0Spec>;
#[doc = "Register `DEVALT0` writer"]
pub type W = crate::W<Devalt0Spec>;
#[doc = "Field `SPIP_SL` reader - SPI Peripheral Interface-Select"]
pub type SpipSlR = crate::BitReader;
#[doc = "Field `SPIP_SL` writer - SPI Peripheral Interface-Select"]
pub type SpipSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_NO_SPIP` reader - GPIO No SPI Peripheral-Select"]
pub type GpioNoSpipR = crate::BitReader;
#[doc = "Field `GPIO_NO_SPIP` writer - GPIO No SPI Peripheral-Select"]
pub type GpioNoSpipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_SPI_CS1` reader - SPI Flash Chip-Select 1"]
pub type FSpiCs1R = crate::BitReader;
#[doc = "Field `F_SPI_CS1` writer - SPI Flash Chip-Select 1"]
pub type FSpiCs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_SPI_QUAD` reader - SPI Flash Quad Interface-Select"]
pub type FSpiQuadR = crate::BitReader;
#[doc = "Field `F_SPI_QUAD` writer - SPI Flash Quad Interface-Select"]
pub type FSpiQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_F_SPI` reader - No SPI Flash-Select"]
pub type NoFSpiR = crate::BitReader;
#[doc = "Field `NO_F_SPI` writer - No SPI Flash-Select"]
pub type NoFSpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Peripheral Interface-Select"]
    #[inline(always)]
    pub fn spip_sl(&self) -> SpipSlR {
        SpipSlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO No SPI Peripheral-Select"]
    #[inline(always)]
    pub fn gpio_no_spip(&self) -> GpioNoSpipR {
        GpioNoSpipR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Flash Chip-Select 1"]
    #[inline(always)]
    pub fn f_spi_cs1(&self) -> FSpiCs1R {
        FSpiCs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Flash Quad Interface-Select"]
    #[inline(always)]
    pub fn f_spi_quad(&self) -> FSpiQuadR {
        FSpiQuadR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No SPI Flash-Select"]
    #[inline(always)]
    pub fn no_f_spi(&self) -> NoFSpiR {
        NoFSpiR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT0")
            .field("spip_sl", &self.spip_sl())
            .field("no_f_spi", &self.no_f_spi())
            .field("f_spi_quad", &self.f_spi_quad())
            .field("f_spi_cs1", &self.f_spi_cs1())
            .field("gpio_no_spip", &self.gpio_no_spip())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI Peripheral Interface-Select"]
    #[inline(always)]
    pub fn spip_sl(&mut self) -> SpipSlW<Devalt0Spec> {
        SpipSlW::new(self, 0)
    }
    #[doc = "Bit 3 - GPIO No SPI Peripheral-Select"]
    #[inline(always)]
    pub fn gpio_no_spip(&mut self) -> GpioNoSpipW<Devalt0Spec> {
        GpioNoSpipW::new(self, 3)
    }
    #[doc = "Bit 4 - SPI Flash Chip-Select 1"]
    #[inline(always)]
    pub fn f_spi_cs1(&mut self) -> FSpiCs1W<Devalt0Spec> {
        FSpiCs1W::new(self, 4)
    }
    #[doc = "Bit 6 - SPI Flash Quad Interface-Select"]
    #[inline(always)]
    pub fn f_spi_quad(&mut self) -> FSpiQuadW<Devalt0Spec> {
        FSpiQuadW::new(self, 6)
    }
    #[doc = "Bit 7 - No SPI Flash-Select"]
    #[inline(always)]
    pub fn no_f_spi(&mut self) -> NoFSpiW<Devalt0Spec> {
        NoFSpiW::new(self, 7)
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
