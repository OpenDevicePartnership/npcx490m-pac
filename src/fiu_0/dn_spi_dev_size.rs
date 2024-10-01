#[doc = "Register `Dn_SPI_DEV_SIZE` reader"]
pub type R = crate::R<DnSpiDevSizeSpec>;
#[doc = "Register `Dn_SPI_DEV_SIZE` writer"]
pub type W = crate::W<DnSpiDevSizeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Device 'n' SPI Device Size Register (Dn_SPI_DEV_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`dn_spi_dev_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dn_spi_dev_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DnSpiDevSizeSpec;
impl crate::RegisterSpec for DnSpiDevSizeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dn_spi_dev_size::R`](R) reader structure"]
impl crate::Readable for DnSpiDevSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`dn_spi_dev_size::W`](W) writer structure"]
impl crate::Writable for DnSpiDevSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets Dn_SPI_DEV_SIZE to value 0"]
impl crate::Resettable for DnSpiDevSizeSpec {
    const RESET_VALUE: u8 = 0;
}
