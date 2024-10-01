#[doc = "Register `SPI_DEV` reader"]
pub type R = crate::R<SpiDevSpec>;
#[doc = "Register `SPI_DEV` writer"]
pub type W = crate::W<SpiDevSpec>;
#[doc = "Field `DN_NADDRB` reader - Device 'n' Number of Address Bytes"]
pub type DnNaddrbR = crate::FieldReader;
#[doc = "Field `DN_NADDRB` writer - Device 'n' Number of Address Bytes"]
pub type DnNaddrbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 5:7 - Device 'n' Number of Address Bytes"]
    #[inline(always)]
    pub fn dn_naddrb(&self) -> DnNaddrbR {
        DnNaddrbR::new((self.bits >> 5) & 7)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DEV")
            .field("dn_naddrb", &self.dn_naddrb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:7 - Device 'n' Number of Address Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn dn_naddrb(&mut self) -> DnNaddrbW<SpiDevSpec> {
        DnNaddrbW::new(self, 5)
    }
}
#[doc = "SPI Device Register (SPI_DEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiDevSpec;
impl crate::RegisterSpec for SpiDevSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spi_dev::R`](R) reader structure"]
impl crate::Readable for SpiDevSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_dev::W`](W) writer structure"]
impl crate::Writable for SpiDevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SPI_DEV to value 0"]
impl crate::Resettable for SpiDevSpec {
    const RESET_VALUE: u8 = 0;
}
