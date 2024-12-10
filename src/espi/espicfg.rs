#[doc = "Register `ESPICFG` reader"]
pub type R = crate::R<EspicfgSpec>;
#[doc = "Register `ESPICFG` writer"]
pub type W = crate::W<EspicfgSpec>;
#[doc = "Field `PCHANEN` reader - Core Peripheral Channel Enable"]
pub type PchanenR = crate::BitReader;
#[doc = "Field `PCHANEN` writer - Core Peripheral Channel Enable"]
pub type PchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWCHANEN` reader - Core Virtual Wire Channel Enable"]
pub type VwchanenR = crate::BitReader;
#[doc = "Field `VWCHANEN` writer - Core Virtual Wire Channel Enable"]
pub type VwchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOBCHANEN` reader - Core OOB Channel Enable"]
pub type OobchanenR = crate::BitReader;
#[doc = "Field `OOBCHANEN` writer - Core OOB Channel Enable"]
pub type OobchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHCHANEN` reader - Core Flash Access Channel Enable"]
pub type FlashchanenR = crate::BitReader;
#[doc = "Field `FLASHCHANEN` writer - Core Flash Access Channel Enable"]
pub type FlashchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCHANEN` reader - Host Peripheral Channel Enable"]
pub type HpchanenR = crate::BitReader;
#[doc = "Field `HPCHANEN` writer - Host Peripheral Channel Enable"]
pub type HpchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVWCHANEN` reader - Host Virtual Wire Channel Enable"]
pub type HvwchanenR = crate::BitReader;
#[doc = "Field `HVWCHANEN` writer - Host Virtual Wire Channel Enable"]
pub type HvwchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOOBCHANEN` reader - Host OOB Channel Enable"]
pub type HoobchanenR = crate::BitReader;
#[doc = "Field `HOOBCHANEN` writer - Host OOB Channel Enable"]
pub type HoobchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFLASHCHANEN` reader - Host Flash Access Channel Enable"]
pub type HflashchanenR = crate::BitReader;
#[doc = "Field `HFLASHCHANEN` writer - Host Flash Access Channel Enable"]
pub type HflashchanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMODE` reader - I/O mode support"]
pub type IomodeR = crate::FieldReader;
#[doc = "Field `IOMODE` writer - I/O mode support"]
pub type IomodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAXFREQ` reader - Maximum Frequency Supported"]
pub type MaxfreqR = crate::FieldReader;
#[doc = "Field `MAXFREQ` writer - Maximum Frequency Supported"]
pub type MaxfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLCHANMODE` reader - Flash Access Channel Mode"]
pub type FlchanmodeR = crate::BitReader;
#[doc = "Field `FLCHANMODE` writer - Flash Access Channel Mode"]
pub type FlchanmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPFREQ` reader - Operating Frequency"]
pub type OpfreqR = crate::FieldReader;
#[doc = "Field `OPFREQ` writer - Operating Frequency"]
pub type OpfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IOMODESEL` reader - I/O Mode Select"]
pub type IomodeselR = crate::FieldReader;
#[doc = "Field `IOMODESEL` writer - I/O Mode Select"]
pub type IomodeselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALERTMODE` reader - ALERT Mode"]
pub type AlertmodeR = crate::BitReader;
#[doc = "Field `ALERTMODE` writer - ALERT Mode"]
pub type AlertmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_CHK_EN` reader - CRC Checking Enable"]
pub type CrcChkEnR = crate::BitReader;
#[doc = "Field `CRC_CHK_EN` writer - CRC Checking Enable"]
pub type CrcChkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCHN_SUPP` reader - Peripheral Channel Supported"]
pub type PcchnSuppR = crate::BitReader;
#[doc = "Field `PCCHN_SUPP` writer - Peripheral Channel Supported"]
pub type PcchnSuppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWCHN_SUPP` reader - Virtual Wire Channel Supported"]
pub type VwchnSuppR = crate::BitReader;
#[doc = "Field `VWCHN_SUPP` writer - Virtual Wire Channel Supported"]
pub type VwchnSuppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOBCHN_SUPP` reader - OOB Channel Supported"]
pub type OobchnSuppR = crate::BitReader;
#[doc = "Field `OOBCHN_SUPP` writer - OOB Channel Supported"]
pub type OobchnSuppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHCHN_SUPP` reader - Flash Access Channel Supported"]
pub type FlashchnSuppR = crate::BitReader;
#[doc = "Field `FLASHCHN_SUPP` writer - Flash Access Channel Supported"]
pub type FlashchnSuppW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core Peripheral Channel Enable"]
    #[inline(always)]
    pub fn pchanen(&self) -> PchanenR {
        PchanenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Virtual Wire Channel Enable"]
    #[inline(always)]
    pub fn vwchanen(&self) -> VwchanenR {
        VwchanenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core OOB Channel Enable"]
    #[inline(always)]
    pub fn oobchanen(&self) -> OobchanenR {
        OobchanenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core Flash Access Channel Enable"]
    #[inline(always)]
    pub fn flashchanen(&self) -> FlashchanenR {
        FlashchanenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host Peripheral Channel Enable"]
    #[inline(always)]
    pub fn hpchanen(&self) -> HpchanenR {
        HpchanenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Virtual Wire Channel Enable"]
    #[inline(always)]
    pub fn hvwchanen(&self) -> HvwchanenR {
        HvwchanenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host OOB Channel Enable"]
    #[inline(always)]
    pub fn hoobchanen(&self) -> HoobchanenR {
        HoobchanenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Host Flash Access Channel Enable"]
    #[inline(always)]
    pub fn hflashchanen(&self) -> HflashchanenR {
        HflashchanenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I/O mode support"]
    #[inline(always)]
    pub fn iomode(&self) -> IomodeR {
        IomodeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Maximum Frequency Supported"]
    #[inline(always)]
    pub fn maxfreq(&self) -> MaxfreqR {
        MaxfreqR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 16 - Flash Access Channel Mode"]
    #[inline(always)]
    pub fn flchanmode(&self) -> FlchanmodeR {
        FlchanmodeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Operating Frequency"]
    #[inline(always)]
    pub fn opfreq(&self) -> OpfreqR {
        OpfreqR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - I/O Mode Select"]
    #[inline(always)]
    pub fn iomodesel(&self) -> IomodeselR {
        IomodeselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - ALERT Mode"]
    #[inline(always)]
    pub fn alertmode(&self) -> AlertmodeR {
        AlertmodeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CRC Checking Enable"]
    #[inline(always)]
    pub fn crc_chk_en(&self) -> CrcChkEnR {
        CrcChkEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral Channel Supported"]
    #[inline(always)]
    pub fn pcchn_supp(&self) -> PcchnSuppR {
        PcchnSuppR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Virtual Wire Channel Supported"]
    #[inline(always)]
    pub fn vwchn_supp(&self) -> VwchnSuppR {
        VwchnSuppR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OOB Channel Supported"]
    #[inline(always)]
    pub fn oobchn_supp(&self) -> OobchnSuppR {
        OobchnSuppR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Flash Access Channel Supported"]
    #[inline(always)]
    pub fn flashchn_supp(&self) -> FlashchnSuppR {
        FlashchnSuppR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPICFG")
            .field("pchanen", &self.pchanen())
            .field("vwchanen", &self.vwchanen())
            .field("oobchanen", &self.oobchanen())
            .field("flashchanen", &self.flashchanen())
            .field("hpchanen", &self.hpchanen())
            .field("hvwchanen", &self.hvwchanen())
            .field("hoobchanen", &self.hoobchanen())
            .field("hflashchanen", &self.hflashchanen())
            .field("iomode", &self.iomode())
            .field("maxfreq", &self.maxfreq())
            .field("flchanmode", &self.flchanmode())
            .field("opfreq", &self.opfreq())
            .field("iomodesel", &self.iomodesel())
            .field("alertmode", &self.alertmode())
            .field("crc_chk_en", &self.crc_chk_en())
            .field("pcchn_supp", &self.pcchn_supp())
            .field("vwchn_supp", &self.vwchn_supp())
            .field("oobchn_supp", &self.oobchn_supp())
            .field("flashchn_supp", &self.flashchn_supp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core Peripheral Channel Enable"]
    #[inline(always)]
    pub fn pchanen(&mut self) -> PchanenW<EspicfgSpec> {
        PchanenW::new(self, 0)
    }
    #[doc = "Bit 1 - Core Virtual Wire Channel Enable"]
    #[inline(always)]
    pub fn vwchanen(&mut self) -> VwchanenW<EspicfgSpec> {
        VwchanenW::new(self, 1)
    }
    #[doc = "Bit 2 - Core OOB Channel Enable"]
    #[inline(always)]
    pub fn oobchanen(&mut self) -> OobchanenW<EspicfgSpec> {
        OobchanenW::new(self, 2)
    }
    #[doc = "Bit 3 - Core Flash Access Channel Enable"]
    #[inline(always)]
    pub fn flashchanen(&mut self) -> FlashchanenW<EspicfgSpec> {
        FlashchanenW::new(self, 3)
    }
    #[doc = "Bit 4 - Host Peripheral Channel Enable"]
    #[inline(always)]
    pub fn hpchanen(&mut self) -> HpchanenW<EspicfgSpec> {
        HpchanenW::new(self, 4)
    }
    #[doc = "Bit 5 - Host Virtual Wire Channel Enable"]
    #[inline(always)]
    pub fn hvwchanen(&mut self) -> HvwchanenW<EspicfgSpec> {
        HvwchanenW::new(self, 5)
    }
    #[doc = "Bit 6 - Host OOB Channel Enable"]
    #[inline(always)]
    pub fn hoobchanen(&mut self) -> HoobchanenW<EspicfgSpec> {
        HoobchanenW::new(self, 6)
    }
    #[doc = "Bit 7 - Host Flash Access Channel Enable"]
    #[inline(always)]
    pub fn hflashchanen(&mut self) -> HflashchanenW<EspicfgSpec> {
        HflashchanenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - I/O mode support"]
    #[inline(always)]
    pub fn iomode(&mut self) -> IomodeW<EspicfgSpec> {
        IomodeW::new(self, 8)
    }
    #[doc = "Bits 10:12 - Maximum Frequency Supported"]
    #[inline(always)]
    pub fn maxfreq(&mut self) -> MaxfreqW<EspicfgSpec> {
        MaxfreqW::new(self, 10)
    }
    #[doc = "Bit 16 - Flash Access Channel Mode"]
    #[inline(always)]
    pub fn flchanmode(&mut self) -> FlchanmodeW<EspicfgSpec> {
        FlchanmodeW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Operating Frequency"]
    #[inline(always)]
    pub fn opfreq(&mut self) -> OpfreqW<EspicfgSpec> {
        OpfreqW::new(self, 17)
    }
    #[doc = "Bits 20:21 - I/O Mode Select"]
    #[inline(always)]
    pub fn iomodesel(&mut self) -> IomodeselW<EspicfgSpec> {
        IomodeselW::new(self, 20)
    }
    #[doc = "Bit 22 - ALERT Mode"]
    #[inline(always)]
    pub fn alertmode(&mut self) -> AlertmodeW<EspicfgSpec> {
        AlertmodeW::new(self, 22)
    }
    #[doc = "Bit 23 - CRC Checking Enable"]
    #[inline(always)]
    pub fn crc_chk_en(&mut self) -> CrcChkEnW<EspicfgSpec> {
        CrcChkEnW::new(self, 23)
    }
    #[doc = "Bit 24 - Peripheral Channel Supported"]
    #[inline(always)]
    pub fn pcchn_supp(&mut self) -> PcchnSuppW<EspicfgSpec> {
        PcchnSuppW::new(self, 24)
    }
    #[doc = "Bit 25 - Virtual Wire Channel Supported"]
    #[inline(always)]
    pub fn vwchn_supp(&mut self) -> VwchnSuppW<EspicfgSpec> {
        VwchnSuppW::new(self, 25)
    }
    #[doc = "Bit 26 - OOB Channel Supported"]
    #[inline(always)]
    pub fn oobchn_supp(&mut self) -> OobchnSuppW<EspicfgSpec> {
        OobchnSuppW::new(self, 26)
    }
    #[doc = "Bit 27 - Flash Access Channel Supported"]
    #[inline(always)]
    pub fn flashchn_supp(&mut self) -> FlashchnSuppW<EspicfgSpec> {
        FlashchnSuppW::new(self, 27)
    }
}
#[doc = "eSPI Configuration Register (ESPICFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`espicfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espicfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspicfgSpec;
impl crate::RegisterSpec for EspicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espicfg::R`](R) reader structure"]
impl crate::Readable for EspicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`espicfg::W`](W) writer structure"]
impl crate::Writable for EspicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESPICFG to value 0"]
impl crate::Resettable for EspicfgSpec {
    const RESET_VALUE: u32 = 0;
}
