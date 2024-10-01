#[doc = "Register `SMBnCTL3` reader"]
pub type R = crate::R<SmbnCtl3Spec>;
#[doc = "Register `SMBnCTL3` writer"]
pub type W = crate::W<SmbnCtl3Spec>;
#[doc = "Field `SCLFRQ8_7` reader - SCL Frequency bits 8 and 7"]
pub type Sclfrq8_7R = crate::FieldReader;
#[doc = "Field `SCLFRQ8_7` writer - SCL Frequency bits 8 and 7"]
pub type Sclfrq8_7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARPMEN` reader - ARP Match Enable"]
pub type ArpmenR = crate::BitReader;
#[doc = "Field `ARPMEN` writer - ARP Match Enable"]
pub type ArpmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_START` reader - Start Detect in Sleep Enable"]
pub type SlpStartR = crate::BitReader;
#[doc = "Field `SLP_START` writer - Start Detect in Sleep Enable"]
pub type SlpStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `400K_MODE` reader - 400 KHz Master Enable"]
pub type _400kModeR = crate::BitReader;
#[doc = "Field `400K_MODE` writer - 400 KHz Master Enable"]
pub type _400kModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNK_SEL` reader - Bank Select"]
pub type BnkSelR = crate::BitReader;
#[doc = "Field `BNK_SEL` writer - Bank Select"]
pub type BnkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDA_LVL` reader - SDA Level"]
pub type SdaLvlR = crate::BitReader;
#[doc = "Field `SDA_LVL` writer - SDA Level"]
pub type SdaLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_LVL` reader - SCL Level"]
pub type SclLvlR = crate::BitReader;
#[doc = "Field `SCL_LVL` writer - SCL Level"]
pub type SclLvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SCL Frequency bits 8 and 7"]
    #[inline(always)]
    pub fn sclfrq8_7(&self) -> Sclfrq8_7R {
        Sclfrq8_7R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ARP Match Enable"]
    #[inline(always)]
    pub fn arpmen(&self) -> ArpmenR {
        ArpmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start Detect in Sleep Enable"]
    #[inline(always)]
    pub fn slp_start(&self) -> SlpStartR {
        SlpStartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 400 KHz Master Enable"]
    #[inline(always)]
    pub fn _400k_mode(&self) -> _400kModeR {
        _400kModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bank Select"]
    #[inline(always)]
    pub fn bnk_sel(&self) -> BnkSelR {
        BnkSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SDA Level"]
    #[inline(always)]
    pub fn sda_lvl(&self) -> SdaLvlR {
        SdaLvlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCL Level"]
    #[inline(always)]
    pub fn scl_lvl(&self) -> SclLvlR {
        SclLvlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnCTL3")
            .field("sclfrq8_7", &self.sclfrq8_7())
            .field("arpmen", &self.arpmen())
            .field("slp_start", &self.slp_start())
            .field("_400k_mode", &self._400k_mode())
            .field("bnk_sel", &self.bnk_sel())
            .field("sda_lvl", &self.sda_lvl())
            .field("scl_lvl", &self.scl_lvl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SCL Frequency bits 8 and 7"]
    #[inline(always)]
    #[must_use]
    pub fn sclfrq8_7(&mut self) -> Sclfrq8_7W<SmbnCtl3Spec> {
        Sclfrq8_7W::new(self, 0)
    }
    #[doc = "Bit 2 - ARP Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpmen(&mut self) -> ArpmenW<SmbnCtl3Spec> {
        ArpmenW::new(self, 2)
    }
    #[doc = "Bit 3 - Start Detect in Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn slp_start(&mut self) -> SlpStartW<SmbnCtl3Spec> {
        SlpStartW::new(self, 3)
    }
    #[doc = "Bit 4 - 400 KHz Master Enable"]
    #[inline(always)]
    #[must_use]
    pub fn _400k_mode(&mut self) -> _400kModeW<SmbnCtl3Spec> {
        _400kModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Bank Select"]
    #[inline(always)]
    #[must_use]
    pub fn bnk_sel(&mut self) -> BnkSelW<SmbnCtl3Spec> {
        BnkSelW::new(self, 5)
    }
    #[doc = "Bit 6 - SDA Level"]
    #[inline(always)]
    #[must_use]
    pub fn sda_lvl(&mut self) -> SdaLvlW<SmbnCtl3Spec> {
        SdaLvlW::new(self, 6)
    }
    #[doc = "Bit 7 - SCL Level"]
    #[inline(always)]
    #[must_use]
    pub fn scl_lvl(&mut self) -> SclLvlW<SmbnCtl3Spec> {
        SclLvlW::new(self, 7)
    }
}
#[doc = "SMB Control 3 Register (SMBnCTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnCtl3Spec;
impl crate::RegisterSpec for SmbnCtl3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_ctl3::R`](R) reader structure"]
impl crate::Readable for SmbnCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`smbn_ctl3::W`](W) writer structure"]
impl crate::Writable for SmbnCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnCTL3 to value 0"]
impl crate::Resettable for SmbnCtl3Spec {
    const RESET_VALUE: u8 = 0;
}
