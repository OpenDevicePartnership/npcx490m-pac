#[doc = "Register `DEVALT0_LK` reader"]
pub type R = crate::R<Devalt0LkSpec>;
#[doc = "Register `DEVALT0_LK` writer"]
pub type W = crate::W<Devalt0LkSpec>;
#[doc = "Field `SPIP_SL_LK` reader - SPI Peripheral Interface-Select Lock"]
pub type SpipSlLkR = crate::BitReader;
#[doc = "Field `SPIP_SL_LK` writer - SPI Peripheral Interface-Select Lock"]
pub type SpipSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_PVT_SPI_LK` reader - No Private SPI Flash-Select Lock"]
pub type NoPvtSpiLkR = crate::BitReader;
#[doc = "Field `NO_PVT_SPI_LK` writer - No Private SPI Flash-Select Lock"]
pub type NoPvtSpiLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Peripheral Interface-Select Lock"]
    #[inline(always)]
    pub fn spip_sl_lk(&self) -> SpipSlLkR {
        SpipSlLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - No Private SPI Flash-Select Lock"]
    #[inline(always)]
    pub fn no_pvt_spi_lk(&self) -> NoPvtSpiLkR {
        NoPvtSpiLkR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT0_LK")
            .field("spip_sl_lk", &self.spip_sl_lk())
            .field("no_pvt_spi_lk", &self.no_pvt_spi_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI Peripheral Interface-Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn spip_sl_lk(&mut self) -> SpipSlLkW<Devalt0LkSpec> {
        SpipSlLkW::new(self, 0)
    }
    #[doc = "Bit 3 - No Private SPI Flash-Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn no_pvt_spi_lk(&mut self) -> NoPvtSpiLkW<Devalt0LkSpec> {
        NoPvtSpiLkW::new(self, 3)
    }
}
#[doc = "Device Alternate Function 0 Lock Register (DEVALT0_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt0_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt0_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt0LkSpec;
impl crate::RegisterSpec for Devalt0LkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt0_lk::R`](R) reader structure"]
impl crate::Readable for Devalt0LkSpec {}
#[doc = "`write(|w| ..)` method takes [`devalt0_lk::W`](W) writer structure"]
impl crate::Writable for Devalt0LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT0_LK to value 0"]
impl crate::Resettable for Devalt0LkSpec {
    const RESET_VALUE: u8 = 0;
}
