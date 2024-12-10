#[doc = "Register `DISIDL_CTL` reader"]
pub type R = crate::R<DisidlCtlSpec>;
#[doc = "Register `DISIDL_CTL` writer"]
pub type W = crate::W<DisidlCtlSpec>;
#[doc = "Field `PECI_DID` reader - PECI Disable in Idle"]
pub type PeciDidR = crate::BitReader;
#[doc = "Field `PECI_DID` writer - PECI Disable in Idle"]
pub type PeciDidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIP_DW` reader - SPI Peripheral Disable in Idle"]
pub type SpipDwR = crate::BitReader;
#[doc = "Field `SPIP_DW` writer - SPI Peripheral Disable in Idle"]
pub type SpipDwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_DID` reader - RAM Disable in Idle"]
pub type RamDidR = crate::BitReader;
#[doc = "Field `RAM_DID` writer - RAM Disable in Idle"]
pub type RamDidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM_DID` reader - ROM Disable in Idle"]
pub type RomDidR = crate::BitReader;
#[doc = "Field `ROM_DID` writer - ROM Disable in Idle"]
pub type RomDidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_DID` reader - Access Disable in Idle"]
pub type AccDidR = crate::BitReader;
#[doc = "Field `ACC_DID` writer - Access Disable in Idle"]
pub type AccDidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - PECI Disable in Idle"]
    #[inline(always)]
    pub fn peci_did(&self) -> PeciDidR {
        PeciDidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Peripheral Disable in Idle"]
    #[inline(always)]
    pub fn spip_dw(&self) -> SpipDwR {
        SpipDwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM Disable in Idle"]
    #[inline(always)]
    pub fn ram_did(&self) -> RamDidR {
        RamDidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ROM Disable in Idle"]
    #[inline(always)]
    pub fn rom_did(&self) -> RomDidR {
        RomDidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Access Disable in Idle"]
    #[inline(always)]
    pub fn acc_did(&self) -> AccDidR {
        AccDidR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DISIDL_CTL")
            .field("peci_did", &self.peci_did())
            .field("spip_dw", &self.spip_dw())
            .field("ram_did", &self.ram_did())
            .field("rom_did", &self.rom_did())
            .field("acc_did", &self.acc_did())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - PECI Disable in Idle"]
    #[inline(always)]
    pub fn peci_did(&mut self) -> PeciDidW<DisidlCtlSpec> {
        PeciDidW::new(self, 2)
    }
    #[doc = "Bit 4 - SPI Peripheral Disable in Idle"]
    #[inline(always)]
    pub fn spip_dw(&mut self) -> SpipDwW<DisidlCtlSpec> {
        SpipDwW::new(self, 4)
    }
    #[doc = "Bit 5 - RAM Disable in Idle"]
    #[inline(always)]
    pub fn ram_did(&mut self) -> RamDidW<DisidlCtlSpec> {
        RamDidW::new(self, 5)
    }
    #[doc = "Bit 6 - ROM Disable in Idle"]
    #[inline(always)]
    pub fn rom_did(&mut self) -> RomDidW<DisidlCtlSpec> {
        RomDidW::new(self, 6)
    }
    #[doc = "Bit 7 - Access Disable in Idle"]
    #[inline(always)]
    pub fn acc_did(&mut self) -> AccDidW<DisidlCtlSpec> {
        AccDidW::new(self, 7)
    }
}
#[doc = "Disable in Idle Control Register (DISIDL_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`disidl_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disidl_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisidlCtlSpec;
impl crate::RegisterSpec for DisidlCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`disidl_ctl::R`](R) reader structure"]
impl crate::Readable for DisidlCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`disidl_ctl::W`](W) writer structure"]
impl crate::Writable for DisidlCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DISIDL_CTL to value 0"]
impl crate::Resettable for DisidlCtlSpec {
    const RESET_VALUE: u8 = 0;
}
