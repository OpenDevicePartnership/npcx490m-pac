#[doc = "Register `HICTRL` reader"]
pub type R = crate::R<HictrlSpec>;
#[doc = "Register `HICTRL` writer"]
pub type W = crate::W<HictrlSpec>;
#[doc = "Field `OBFKIE` reader - Output Buffer Full Keyboard Interrupt Enable"]
pub type ObfkieR = crate::BitReader;
#[doc = "Field `OBFKIE` writer - Output Buffer Full Keyboard Interrupt Enable"]
pub type ObfkieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBFMIE` reader - Output Buffer Full Mouse Interrupt Enable"]
pub type ObfmieR = crate::BitReader;
#[doc = "Field `OBFMIE` writer - Output Buffer Full Mouse Interrupt Enable"]
pub type ObfmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBECIE` reader - Output Buffer Empty Core Interrupt Enable"]
pub type ObecieR = crate::BitReader;
#[doc = "Field `OBECIE` writer - Output Buffer Empty Core Interrupt Enable"]
pub type ObecieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBFCIE` reader - Input Buffer Full Core Interrupt Enable"]
pub type IbfcieR = crate::BitReader;
#[doc = "Field `IBFCIE` writer - Input Buffer Full Core Interrupt Enable"]
pub type IbfcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_OBF` reader - Firmware Control Over OBF"]
pub type FwObfR = crate::BitReader;
#[doc = "Field `FW_OBF` writer - Firmware Control Over OBF"]
pub type FwObfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output Buffer Full Keyboard Interrupt Enable"]
    #[inline(always)]
    pub fn obfkie(&self) -> ObfkieR {
        ObfkieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Buffer Full Mouse Interrupt Enable"]
    #[inline(always)]
    pub fn obfmie(&self) -> ObfmieR {
        ObfmieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Buffer Empty Core Interrupt Enable"]
    #[inline(always)]
    pub fn obecie(&self) -> ObecieR {
        ObecieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Buffer Full Core Interrupt Enable"]
    #[inline(always)]
    pub fn ibfcie(&self) -> IbfcieR {
        IbfcieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Firmware Control Over OBF"]
    #[inline(always)]
    pub fn fw_obf(&self) -> FwObfR {
        FwObfR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HICTRL")
            .field("obfkie", &self.obfkie())
            .field("obfmie", &self.obfmie())
            .field("obecie", &self.obecie())
            .field("ibfcie", &self.ibfcie())
            .field("fw_obf", &self.fw_obf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Output Buffer Full Keyboard Interrupt Enable"]
    #[inline(always)]
    pub fn obfkie(&mut self) -> ObfkieW<HictrlSpec> {
        ObfkieW::new(self, 0)
    }
    #[doc = "Bit 1 - Output Buffer Full Mouse Interrupt Enable"]
    #[inline(always)]
    pub fn obfmie(&mut self) -> ObfmieW<HictrlSpec> {
        ObfmieW::new(self, 1)
    }
    #[doc = "Bit 2 - Output Buffer Empty Core Interrupt Enable"]
    #[inline(always)]
    pub fn obecie(&mut self) -> ObecieW<HictrlSpec> {
        ObecieW::new(self, 2)
    }
    #[doc = "Bit 3 - Input Buffer Full Core Interrupt Enable"]
    #[inline(always)]
    pub fn ibfcie(&mut self) -> IbfcieW<HictrlSpec> {
        IbfcieW::new(self, 3)
    }
    #[doc = "Bit 7 - Firmware Control Over OBF"]
    #[inline(always)]
    pub fn fw_obf(&mut self) -> FwObfW<HictrlSpec> {
        FwObfW::new(self, 7)
    }
}
#[doc = "Host Interface Control Register (HICTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HictrlSpec;
impl crate::RegisterSpec for HictrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hictrl::R`](R) reader structure"]
impl crate::Readable for HictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hictrl::W`](W) writer structure"]
impl crate::Writable for HictrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HICTRL to value 0"]
impl crate::Resettable for HictrlSpec {
    const RESET_VALUE: u8 = 0;
}
