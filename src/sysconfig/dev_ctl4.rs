#[doc = "Register `DEV_CTL4` reader"]
pub type R = crate::R<DevCtl4Spec>;
#[doc = "Register `DEV_CTL4` writer"]
pub type W = crate::W<DevCtl4Spec>;
#[doc = "Field `VCIO_TYP_SEL` reader - VCIO Supply Type Select"]
pub type VcioTypSelR = crate::FieldReader;
#[doc = "Field `VCIO_TYP_SEL` writer - VCIO Supply Type Select"]
pub type VcioTypSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SHD_SPI_SLLK` reader - Shared Flash SPI Interface-Select Lock"]
pub type ShdSpiSllkR = crate::BitReader;
#[doc = "Field `SHD_SPI_SLLK` writer - Shared Flash SPI Interface-Select Lock"]
pub type ShdSpiSllkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP_IF` reader - Write-Protect Internal Flash"]
pub type WpIfR = crate::BitReader;
#[doc = "Field `WP_IF` writer - Write-Protect Internal Flash"]
pub type WpIfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCC1_RST_LK` reader - NO_VCC1_RST Bit Lock"]
pub type Vcc1RstLkR = crate::BitReader;
#[doc = "Field `VCC1_RST_LK` writer - NO_VCC1_RST Bit Lock"]
pub type Vcc1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - VCIO Supply Type Select"]
    #[inline(always)]
    pub fn vcio_typ_sel(&self) -> VcioTypSelR {
        VcioTypSelR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Shared Flash SPI Interface-Select Lock"]
    #[inline(always)]
    pub fn shd_spi_sllk(&self) -> ShdSpiSllkR {
        ShdSpiSllkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Write-Protect Internal Flash"]
    #[inline(always)]
    pub fn wp_if(&self) -> WpIfR {
        WpIfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NO_VCC1_RST Bit Lock"]
    #[inline(always)]
    pub fn vcc1_rst_lk(&self) -> Vcc1RstLkR {
        Vcc1RstLkR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_CTL4")
            .field("vcio_typ_sel", &self.vcio_typ_sel())
            .field("shd_spi_sllk", &self.shd_spi_sllk())
            .field("wp_if", &self.wp_if())
            .field("vcc1_rst_lk", &self.vcc1_rst_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - VCIO Supply Type Select"]
    #[inline(always)]
    pub fn vcio_typ_sel(&mut self) -> VcioTypSelW<DevCtl4Spec> {
        VcioTypSelW::new(self, 0)
    }
    #[doc = "Bit 2 - Shared Flash SPI Interface-Select Lock"]
    #[inline(always)]
    pub fn shd_spi_sllk(&mut self) -> ShdSpiSllkW<DevCtl4Spec> {
        ShdSpiSllkW::new(self, 2)
    }
    #[doc = "Bit 5 - Write-Protect Internal Flash"]
    #[inline(always)]
    pub fn wp_if(&mut self) -> WpIfW<DevCtl4Spec> {
        WpIfW::new(self, 5)
    }
    #[doc = "Bit 6 - NO_VCC1_RST Bit Lock"]
    #[inline(always)]
    pub fn vcc1_rst_lk(&mut self) -> Vcc1RstLkW<DevCtl4Spec> {
        Vcc1RstLkW::new(self, 6)
    }
}
#[doc = "Device Control 4 Register (DEV_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCtl4Spec;
impl crate::RegisterSpec for DevCtl4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dev_ctl4::R`](R) reader structure"]
impl crate::Readable for DevCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_ctl4::W`](W) writer structure"]
impl crate::Writable for DevCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEV_CTL4 to value 0"]
impl crate::Resettable for DevCtl4Spec {
    const RESET_VALUE: u8 = 0;
}
