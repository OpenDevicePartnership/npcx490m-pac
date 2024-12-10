#[doc = "Register `SWRST_CTL2_LK` reader"]
pub type R = crate::R<SwrstCtl2LkSpec>;
#[doc = "Register `SWRST_CTL2_LK` writer"]
pub type W = crate::W<SwrstCtl2LkSpec>;
#[doc = "Field `PMC_RST_LK` reader - PMC Reset Lock"]
pub type PmcRstLkR = crate::BitReader;
#[doc = "Field `PMC_RST_LK` writer - PMC Reset Lock"]
pub type PmcRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIP_RST_LK` reader - SPI Peripheral Reset Lock"]
pub type SpipRstLkR = crate::BitReader;
#[doc = "Field `SPIP_RST_LK` writer - SPI Peripheral Reset Lock"]
pub type SpipRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCE_RST_LK` reader - ADC_EREF Reset Lock"]
pub type AdceRstLkR = crate::BitReader;
#[doc = "Field `ADCE_RST_LK` writer - ADC_EREF Reset Lock"]
pub type AdceRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECI_RST_LK` reader - PECI Reset Lock"]
pub type PeciRstLkR = crate::BitReader;
#[doc = "Field `PECI_RST_LK` writer - PECI Reset Lock"]
pub type PeciRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT2_RST_LK` reader - CR_UART 2 Reset Lock"]
pub type Crurt2RstLkR = crate::BitReader;
#[doc = "Field `CRURT2_RST_LK` writer - CR_UART 2 Reset Lock"]
pub type Crurt2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCI_RST_LK` reader - ADC_IREF Reset Lock"]
pub type AdciRstLkR = crate::BitReader;
#[doc = "Field `ADCI_RST_LK` writer - ADC_IREF Reset Lock"]
pub type AdciRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB0_RST_LK` reader - SMB0 Reset Lock"]
pub type Smb0RstLkR = crate::BitReader;
#[doc = "Field `SMB0_RST_LK` writer - SMB0 Reset Lock"]
pub type Smb0RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB1_RST_LK` reader - SMB1 Reset Lock"]
pub type Smb1RstLkR = crate::BitReader;
#[doc = "Field `SMB1_RST_LK` writer - SMB1 Reset Lock"]
pub type Smb1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB2_RST_LK` reader - SMB2 Reset Lock"]
pub type Smb2RstLkR = crate::BitReader;
#[doc = "Field `SMB2_RST_LK` writer - SMB2 Reset Lock"]
pub type Smb2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB3_RST_LK` reader - SMB3 Reset Lock"]
pub type Smb3RstLkR = crate::BitReader;
#[doc = "Field `SMB3_RST_LK` writer - SMB3 Reset Lock"]
pub type Smb3RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB4_RST_LK` reader - SMB4 Reset Lock"]
pub type Smb4RstLkR = crate::BitReader;
#[doc = "Field `SMB4_RST_LK` writer - SMB4 Reset Lock"]
pub type Smb4RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB5_RST_LK` reader - SMB5 Reset Lock"]
pub type Smb5RstLkR = crate::BitReader;
#[doc = "Field `SMB5_RST_LK` writer - SMB5 Reset Lock"]
pub type Smb5RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB6_RST_LK` reader - SMB6 Reset Lock"]
pub type Smb6RstLkR = crate::BitReader;
#[doc = "Field `SMB6_RST_LK` writer - SMB6 Reset Lock"]
pub type Smb6RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_RST_LK` reader - TWD Reset Lock"]
pub type TwdRstLkR = crate::BitReader;
#[doc = "Field `TWD_RST_LK` writer - TWD Reset Lock"]
pub type TwdRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_RST_LK` reader - PWM0 Reset Lock"]
pub type Pwm0RstLkR = crate::BitReader;
#[doc = "Field `PWM0_RST_LK` writer - PWM0 Reset Lock"]
pub type Pwm0RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_RST_LK` reader - PWM1 Reset Lock"]
pub type Pwm1RstLkR = crate::BitReader;
#[doc = "Field `PWM1_RST_LK` writer - PWM1 Reset Lock"]
pub type Pwm1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_RST_LK` reader - PWM2 Reset Lock"]
pub type Pwm2RstLkR = crate::BitReader;
#[doc = "Field `PWM2_RST_LK` writer - PWM2 Reset Lock"]
pub type Pwm2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_RST_LK` reader - PWM3 Reset Lock"]
pub type Pwm3RstLkR = crate::BitReader;
#[doc = "Field `PWM3_RST_LK` writer - PWM3 Reset Lock"]
pub type Pwm3RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4_RST_LK` reader - PWM4 Reset Lock"]
pub type Pwm4RstLkR = crate::BitReader;
#[doc = "Field `PWM4_RST_LK` writer - PWM4 Reset Lock"]
pub type Pwm4RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM5_RST_LK` reader - PWM5 Reset Lock"]
pub type Pwm5RstLkR = crate::BitReader;
#[doc = "Field `PWM5_RST_LK` writer - PWM5 Reset Lock"]
pub type Pwm5RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM6_RST_LK` reader - PWM6 Reset Lock"]
pub type Pwm6RstLkR = crate::BitReader;
#[doc = "Field `PWM6_RST_LK` writer - PWM6 Reset Lock"]
pub type Pwm6RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM7_RST_LK` reader - PWM7 Reset Lock"]
pub type Pwm7RstLkR = crate::BitReader;
#[doc = "Field `PWM7_RST_LK` writer - PWM7 Reset Lock"]
pub type Pwm7RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT16_1_RST_LK` reader - MFT16-1 Reset Lock"]
pub type Mft16_1RstLkR = crate::BitReader;
#[doc = "Field `MFT16_1_RST_LK` writer - MFT16-1 Reset Lock"]
pub type Mft16_1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT16_2_RST_LK` reader - MFT16-2 Reset Lock"]
pub type Mft16_2RstLkR = crate::BitReader;
#[doc = "Field `MFT16_2_RST_LK` writer - MFT16-2 Reset Lock"]
pub type Mft16_2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT16_3_RST_LK` reader - MFT16-3 Reset Lock"]
pub type Mft16_3RstLkR = crate::BitReader;
#[doc = "Field `MFT16_3_RST_LK` writer - MFT16-3 Reset Lock"]
pub type Mft16_3RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB7_RST_LK` reader - SMB7 Reset Lock"]
pub type Smb7RstLkR = crate::BitReader;
#[doc = "Field `SMB7_RST_LK` writer - SMB7 Reset Lock"]
pub type Smb7RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT1_RST_LK` reader - CR_UART 1 Reset Lock"]
pub type Crurt1RstLkR = crate::BitReader;
#[doc = "Field `CRURT1_RST_LK` writer - CR_UART 1 Reset Lock"]
pub type Crurt1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_RST_LK` reader - PS/2 Reset Lock"]
pub type Ps2RstLkR = crate::BitReader;
#[doc = "Field `PS2_RST_LK` writer - PS/2 Reset Lock"]
pub type Ps2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDP_RST_LK` reader - SDP Reset Lock"]
pub type SdpRstLkR = crate::BitReader;
#[doc = "Field `SDP_RST_LK` writer - SDP Reset Lock"]
pub type SdpRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBS_RST_LK` reader - Keyboard Scan Reset Lock"]
pub type KbsRstLkR = crate::BitReader;
#[doc = "Field `KBS_RST_LK` writer - Keyboard Scan Reset Lock"]
pub type KbsRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PMC Reset Lock"]
    #[inline(always)]
    pub fn pmc_rst_lk(&self) -> PmcRstLkR {
        PmcRstLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - SPI Peripheral Reset Lock"]
    #[inline(always)]
    pub fn spip_rst_lk(&self) -> SpipRstLkR {
        SpipRstLkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC_EREF Reset Lock"]
    #[inline(always)]
    pub fn adce_rst_lk(&self) -> AdceRstLkR {
        AdceRstLkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PECI Reset Lock"]
    #[inline(always)]
    pub fn peci_rst_lk(&self) -> PeciRstLkR {
        PeciRstLkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CR_UART 2 Reset Lock"]
    #[inline(always)]
    pub fn crurt2_rst_lk(&self) -> Crurt2RstLkR {
        Crurt2RstLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC_IREF Reset Lock"]
    #[inline(always)]
    pub fn adci_rst_lk(&self) -> AdciRstLkR {
        AdciRstLkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMB0 Reset Lock"]
    #[inline(always)]
    pub fn smb0_rst_lk(&self) -> Smb0RstLkR {
        Smb0RstLkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SMB1 Reset Lock"]
    #[inline(always)]
    pub fn smb1_rst_lk(&self) -> Smb1RstLkR {
        Smb1RstLkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMB2 Reset Lock"]
    #[inline(always)]
    pub fn smb2_rst_lk(&self) -> Smb2RstLkR {
        Smb2RstLkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SMB3 Reset Lock"]
    #[inline(always)]
    pub fn smb3_rst_lk(&self) -> Smb3RstLkR {
        Smb3RstLkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SMB4 Reset Lock"]
    #[inline(always)]
    pub fn smb4_rst_lk(&self) -> Smb4RstLkR {
        Smb4RstLkR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB5 Reset Lock"]
    #[inline(always)]
    pub fn smb5_rst_lk(&self) -> Smb5RstLkR {
        Smb5RstLkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB6 Reset Lock"]
    #[inline(always)]
    pub fn smb6_rst_lk(&self) -> Smb6RstLkR {
        Smb6RstLkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TWD Reset Lock"]
    #[inline(always)]
    pub fn twd_rst_lk(&self) -> TwdRstLkR {
        TwdRstLkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PWM0 Reset Lock"]
    #[inline(always)]
    pub fn pwm0_rst_lk(&self) -> Pwm0RstLkR {
        Pwm0RstLkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM1 Reset Lock"]
    #[inline(always)]
    pub fn pwm1_rst_lk(&self) -> Pwm1RstLkR {
        Pwm1RstLkR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWM2 Reset Lock"]
    #[inline(always)]
    pub fn pwm2_rst_lk(&self) -> Pwm2RstLkR {
        Pwm2RstLkR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWM3 Reset Lock"]
    #[inline(always)]
    pub fn pwm3_rst_lk(&self) -> Pwm3RstLkR {
        Pwm3RstLkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM4 Reset Lock"]
    #[inline(always)]
    pub fn pwm4_rst_lk(&self) -> Pwm4RstLkR {
        Pwm4RstLkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PWM5 Reset Lock"]
    #[inline(always)]
    pub fn pwm5_rst_lk(&self) -> Pwm5RstLkR {
        Pwm5RstLkR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PWM6 Reset Lock"]
    #[inline(always)]
    pub fn pwm6_rst_lk(&self) -> Pwm6RstLkR {
        Pwm6RstLkR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PWM7 Reset Lock"]
    #[inline(always)]
    pub fn pwm7_rst_lk(&self) -> Pwm7RstLkR {
        Pwm7RstLkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MFT16-1 Reset Lock"]
    #[inline(always)]
    pub fn mft16_1_rst_lk(&self) -> Mft16_1RstLkR {
        Mft16_1RstLkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MFT16-2 Reset Lock"]
    #[inline(always)]
    pub fn mft16_2_rst_lk(&self) -> Mft16_2RstLkR {
        Mft16_2RstLkR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MFT16-3 Reset Lock"]
    #[inline(always)]
    pub fn mft16_3_rst_lk(&self) -> Mft16_3RstLkR {
        Mft16_3RstLkR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SMB7 Reset Lock"]
    #[inline(always)]
    pub fn smb7_rst_lk(&self) -> Smb7RstLkR {
        Smb7RstLkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CR_UART 1 Reset Lock"]
    #[inline(always)]
    pub fn crurt1_rst_lk(&self) -> Crurt1RstLkR {
        Crurt1RstLkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PS/2 Reset Lock"]
    #[inline(always)]
    pub fn ps2_rst_lk(&self) -> Ps2RstLkR {
        Ps2RstLkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SDP Reset Lock"]
    #[inline(always)]
    pub fn sdp_rst_lk(&self) -> SdpRstLkR {
        SdpRstLkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Keyboard Scan Reset Lock"]
    #[inline(always)]
    pub fn kbs_rst_lk(&self) -> KbsRstLkR {
        KbsRstLkR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRST_CTL2_LK")
            .field("pmc_rst_lk", &self.pmc_rst_lk())
            .field("spip_rst_lk", &self.spip_rst_lk())
            .field("adce_rst_lk", &self.adce_rst_lk())
            .field("peci_rst_lk", &self.peci_rst_lk())
            .field("crurt2_rst_lk", &self.crurt2_rst_lk())
            .field("adci_rst_lk", &self.adci_rst_lk())
            .field("smb0_rst_lk", &self.smb0_rst_lk())
            .field("smb1_rst_lk", &self.smb1_rst_lk())
            .field("smb2_rst_lk", &self.smb2_rst_lk())
            .field("smb3_rst_lk", &self.smb3_rst_lk())
            .field("smb4_rst_lk", &self.smb4_rst_lk())
            .field("smb5_rst_lk", &self.smb5_rst_lk())
            .field("smb6_rst_lk", &self.smb6_rst_lk())
            .field("twd_rst_lk", &self.twd_rst_lk())
            .field("pwm0_rst_lk", &self.pwm0_rst_lk())
            .field("pwm1_rst_lk", &self.pwm1_rst_lk())
            .field("pwm2_rst_lk", &self.pwm2_rst_lk())
            .field("pwm3_rst_lk", &self.pwm3_rst_lk())
            .field("pwm4_rst_lk", &self.pwm4_rst_lk())
            .field("pwm5_rst_lk", &self.pwm5_rst_lk())
            .field("pwm6_rst_lk", &self.pwm6_rst_lk())
            .field("pwm7_rst_lk", &self.pwm7_rst_lk())
            .field("mft16_1_rst_lk", &self.mft16_1_rst_lk())
            .field("mft16_2_rst_lk", &self.mft16_2_rst_lk())
            .field("mft16_3_rst_lk", &self.mft16_3_rst_lk())
            .field("smb7_rst_lk", &self.smb7_rst_lk())
            .field("crurt1_rst_lk", &self.crurt1_rst_lk())
            .field("ps2_rst_lk", &self.ps2_rst_lk())
            .field("sdp_rst_lk", &self.sdp_rst_lk())
            .field("kbs_rst_lk", &self.kbs_rst_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PMC Reset Lock"]
    #[inline(always)]
    pub fn pmc_rst_lk(&mut self) -> PmcRstLkW<SwrstCtl2LkSpec> {
        PmcRstLkW::new(self, 0)
    }
    #[doc = "Bit 3 - SPI Peripheral Reset Lock"]
    #[inline(always)]
    pub fn spip_rst_lk(&mut self) -> SpipRstLkW<SwrstCtl2LkSpec> {
        SpipRstLkW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC_EREF Reset Lock"]
    #[inline(always)]
    pub fn adce_rst_lk(&mut self) -> AdceRstLkW<SwrstCtl2LkSpec> {
        AdceRstLkW::new(self, 4)
    }
    #[doc = "Bit 5 - PECI Reset Lock"]
    #[inline(always)]
    pub fn peci_rst_lk(&mut self) -> PeciRstLkW<SwrstCtl2LkSpec> {
        PeciRstLkW::new(self, 5)
    }
    #[doc = "Bit 6 - CR_UART 2 Reset Lock"]
    #[inline(always)]
    pub fn crurt2_rst_lk(&mut self) -> Crurt2RstLkW<SwrstCtl2LkSpec> {
        Crurt2RstLkW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC_IREF Reset Lock"]
    #[inline(always)]
    pub fn adci_rst_lk(&mut self) -> AdciRstLkW<SwrstCtl2LkSpec> {
        AdciRstLkW::new(self, 7)
    }
    #[doc = "Bit 8 - SMB0 Reset Lock"]
    #[inline(always)]
    pub fn smb0_rst_lk(&mut self) -> Smb0RstLkW<SwrstCtl2LkSpec> {
        Smb0RstLkW::new(self, 8)
    }
    #[doc = "Bit 9 - SMB1 Reset Lock"]
    #[inline(always)]
    pub fn smb1_rst_lk(&mut self) -> Smb1RstLkW<SwrstCtl2LkSpec> {
        Smb1RstLkW::new(self, 9)
    }
    #[doc = "Bit 10 - SMB2 Reset Lock"]
    #[inline(always)]
    pub fn smb2_rst_lk(&mut self) -> Smb2RstLkW<SwrstCtl2LkSpec> {
        Smb2RstLkW::new(self, 10)
    }
    #[doc = "Bit 11 - SMB3 Reset Lock"]
    #[inline(always)]
    pub fn smb3_rst_lk(&mut self) -> Smb3RstLkW<SwrstCtl2LkSpec> {
        Smb3RstLkW::new(self, 11)
    }
    #[doc = "Bit 12 - SMB4 Reset Lock"]
    #[inline(always)]
    pub fn smb4_rst_lk(&mut self) -> Smb4RstLkW<SwrstCtl2LkSpec> {
        Smb4RstLkW::new(self, 12)
    }
    #[doc = "Bit 13 - SMB5 Reset Lock"]
    #[inline(always)]
    pub fn smb5_rst_lk(&mut self) -> Smb5RstLkW<SwrstCtl2LkSpec> {
        Smb5RstLkW::new(self, 13)
    }
    #[doc = "Bit 14 - SMB6 Reset Lock"]
    #[inline(always)]
    pub fn smb6_rst_lk(&mut self) -> Smb6RstLkW<SwrstCtl2LkSpec> {
        Smb6RstLkW::new(self, 14)
    }
    #[doc = "Bit 15 - TWD Reset Lock"]
    #[inline(always)]
    pub fn twd_rst_lk(&mut self) -> TwdRstLkW<SwrstCtl2LkSpec> {
        TwdRstLkW::new(self, 15)
    }
    #[doc = "Bit 16 - PWM0 Reset Lock"]
    #[inline(always)]
    pub fn pwm0_rst_lk(&mut self) -> Pwm0RstLkW<SwrstCtl2LkSpec> {
        Pwm0RstLkW::new(self, 16)
    }
    #[doc = "Bit 17 - PWM1 Reset Lock"]
    #[inline(always)]
    pub fn pwm1_rst_lk(&mut self) -> Pwm1RstLkW<SwrstCtl2LkSpec> {
        Pwm1RstLkW::new(self, 17)
    }
    #[doc = "Bit 18 - PWM2 Reset Lock"]
    #[inline(always)]
    pub fn pwm2_rst_lk(&mut self) -> Pwm2RstLkW<SwrstCtl2LkSpec> {
        Pwm2RstLkW::new(self, 18)
    }
    #[doc = "Bit 19 - PWM3 Reset Lock"]
    #[inline(always)]
    pub fn pwm3_rst_lk(&mut self) -> Pwm3RstLkW<SwrstCtl2LkSpec> {
        Pwm3RstLkW::new(self, 19)
    }
    #[doc = "Bit 20 - PWM4 Reset Lock"]
    #[inline(always)]
    pub fn pwm4_rst_lk(&mut self) -> Pwm4RstLkW<SwrstCtl2LkSpec> {
        Pwm4RstLkW::new(self, 20)
    }
    #[doc = "Bit 21 - PWM5 Reset Lock"]
    #[inline(always)]
    pub fn pwm5_rst_lk(&mut self) -> Pwm5RstLkW<SwrstCtl2LkSpec> {
        Pwm5RstLkW::new(self, 21)
    }
    #[doc = "Bit 22 - PWM6 Reset Lock"]
    #[inline(always)]
    pub fn pwm6_rst_lk(&mut self) -> Pwm6RstLkW<SwrstCtl2LkSpec> {
        Pwm6RstLkW::new(self, 22)
    }
    #[doc = "Bit 23 - PWM7 Reset Lock"]
    #[inline(always)]
    pub fn pwm7_rst_lk(&mut self) -> Pwm7RstLkW<SwrstCtl2LkSpec> {
        Pwm7RstLkW::new(self, 23)
    }
    #[doc = "Bit 24 - MFT16-1 Reset Lock"]
    #[inline(always)]
    pub fn mft16_1_rst_lk(&mut self) -> Mft16_1RstLkW<SwrstCtl2LkSpec> {
        Mft16_1RstLkW::new(self, 24)
    }
    #[doc = "Bit 25 - MFT16-2 Reset Lock"]
    #[inline(always)]
    pub fn mft16_2_rst_lk(&mut self) -> Mft16_2RstLkW<SwrstCtl2LkSpec> {
        Mft16_2RstLkW::new(self, 25)
    }
    #[doc = "Bit 26 - MFT16-3 Reset Lock"]
    #[inline(always)]
    pub fn mft16_3_rst_lk(&mut self) -> Mft16_3RstLkW<SwrstCtl2LkSpec> {
        Mft16_3RstLkW::new(self, 26)
    }
    #[doc = "Bit 27 - SMB7 Reset Lock"]
    #[inline(always)]
    pub fn smb7_rst_lk(&mut self) -> Smb7RstLkW<SwrstCtl2LkSpec> {
        Smb7RstLkW::new(self, 27)
    }
    #[doc = "Bit 28 - CR_UART 1 Reset Lock"]
    #[inline(always)]
    pub fn crurt1_rst_lk(&mut self) -> Crurt1RstLkW<SwrstCtl2LkSpec> {
        Crurt1RstLkW::new(self, 28)
    }
    #[doc = "Bit 29 - PS/2 Reset Lock"]
    #[inline(always)]
    pub fn ps2_rst_lk(&mut self) -> Ps2RstLkW<SwrstCtl2LkSpec> {
        Ps2RstLkW::new(self, 29)
    }
    #[doc = "Bit 30 - SDP Reset Lock"]
    #[inline(always)]
    pub fn sdp_rst_lk(&mut self) -> SdpRstLkW<SwrstCtl2LkSpec> {
        SdpRstLkW::new(self, 30)
    }
    #[doc = "Bit 31 - Keyboard Scan Reset Lock"]
    #[inline(always)]
    pub fn kbs_rst_lk(&mut self) -> KbsRstLkW<SwrstCtl2LkSpec> {
        KbsRstLkW::new(self, 31)
    }
}
#[doc = "Software Reset Control 2 Lock Register (SWRST_CTL2_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl2_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl2_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCtl2LkSpec;
impl crate::RegisterSpec for SwrstCtl2LkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_ctl2_lk::R`](R) reader structure"]
impl crate::Readable for SwrstCtl2LkSpec {}
#[doc = "`write(|w| ..)` method takes [`swrst_ctl2_lk::W`](W) writer structure"]
impl crate::Writable for SwrstCtl2LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRST_CTL2_LK to value 0"]
impl crate::Resettable for SwrstCtl2LkSpec {
    const RESET_VALUE: u32 = 0;
}
