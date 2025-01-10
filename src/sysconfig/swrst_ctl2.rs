#[doc = "Register `SWRST_CTL2` reader"]
pub type R = crate::R<SwrstCtl2Spec>;
#[doc = "Register `SWRST_CTL2` writer"]
pub type W = crate::W<SwrstCtl2Spec>;
#[doc = "Field `PMC_RST` reader - PMC Reset"]
pub type PmcRstR = crate::BitReader;
#[doc = "Field `PMC_RST` writer - PMC Reset"]
pub type PmcRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHI_RST` reader - SHI Reset"]
pub type ShiRstR = crate::BitReader;
#[doc = "Field `SHI_RST` writer - SHI Reset"]
pub type ShiRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIP_RST` reader - SPI Peripheral Reset"]
pub type SpipRstR = crate::BitReader;
#[doc = "Field `SPIP_RST` writer - SPI Peripheral Reset"]
pub type SpipRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCE_RST` reader - ADC_EREF Reset"]
pub type AdceRstR = crate::BitReader;
#[doc = "Field `ADCE_RST` writer - ADC_EREF Reset"]
pub type AdceRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECI_RST` reader - PECI Reset"]
pub type PeciRstR = crate::BitReader;
#[doc = "Field `PECI_RST` writer - PECI Reset"]
pub type PeciRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT2_RST` reader - CR_UART 2 Reset"]
pub type Crurt2RstR = crate::BitReader;
#[doc = "Field `CRURT2_RST` writer - CR_UART 2 Reset"]
pub type Crurt2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCI_RST` reader - ADC_IREF Reset"]
pub type AdciRstR = crate::BitReader;
#[doc = "Field `ADCI_RST` writer - ADC_IREF Reset"]
pub type AdciRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB0_RST` reader - SMB0 Reset"]
pub type Smb0RstR = crate::BitReader;
#[doc = "Field `SMB0_RST` writer - SMB0 Reset"]
pub type Smb0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB1_RST` reader - SMB1 Reset"]
pub type Smb1RstR = crate::BitReader;
#[doc = "Field `SMB1_RST` writer - SMB1 Reset"]
pub type Smb1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB2_RST` reader - SMB2 Reset"]
pub type Smb2RstR = crate::BitReader;
#[doc = "Field `SMB2_RST` writer - SMB2 Reset"]
pub type Smb2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB3_RST` reader - SMB3 Reset"]
pub type Smb3RstR = crate::BitReader;
#[doc = "Field `SMB3_RST` writer - SMB3 Reset"]
pub type Smb3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB4_RST` reader - SMB4 Reset"]
pub type Smb4RstR = crate::BitReader;
#[doc = "Field `SMB4_RST` writer - SMB4 Reset"]
pub type Smb4RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB5_RST` reader - SMB5 Reset"]
pub type Smb5RstR = crate::BitReader;
#[doc = "Field `SMB5_RST` writer - SMB5 Reset"]
pub type Smb5RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB6_RST` reader - SMB6 Reset"]
pub type Smb6RstR = crate::BitReader;
#[doc = "Field `SMB6_RST` writer - SMB6 Reset"]
pub type Smb6RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWD_RST` reader - TWD Reset"]
pub type TwdRstR = crate::BitReader;
#[doc = "Field `TWD_RST` writer - TWD Reset"]
pub type TwdRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_RST` reader - PWM0 Reset"]
pub type Pwm0RstR = crate::BitReader;
#[doc = "Field `PWM0_RST` writer - PWM0 Reset"]
pub type Pwm0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_RST` reader - PWM1 Reset"]
pub type Pwm1RstR = crate::BitReader;
#[doc = "Field `PWM1_RST` writer - PWM1 Reset"]
pub type Pwm1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_RST` reader - PWM2 Reset"]
pub type Pwm2RstR = crate::BitReader;
#[doc = "Field `PWM2_RST` writer - PWM2 Reset"]
pub type Pwm2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_RST` reader - PWM3 Reset"]
pub type Pwm3RstR = crate::BitReader;
#[doc = "Field `PWM3_RST` writer - PWM3 Reset"]
pub type Pwm3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4_RST` reader - PWM4 Reset"]
pub type Pwm4RstR = crate::BitReader;
#[doc = "Field `PWM4_RST` writer - PWM4 Reset"]
pub type Pwm4RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM5_RST` reader - PWM5 Reset"]
pub type Pwm5RstR = crate::BitReader;
#[doc = "Field `PWM5_RST` writer - PWM5 Reset"]
pub type Pwm5RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM6_RST` reader - PWM6 Reset"]
pub type Pwm6RstR = crate::BitReader;
#[doc = "Field `PWM6_RST` writer - PWM6 Reset"]
pub type Pwm6RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM7_RST` reader - PWM7 Reset"]
pub type Pwm7RstR = crate::BitReader;
#[doc = "Field `PWM7_RST` writer - PWM7 Reset"]
pub type Pwm7RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT16_1_RST` reader - MFT16-1 Reset"]
pub type Mft16_1RstR = crate::BitReader;
#[doc = "Field `MFT16_1_RST` writer - MFT16-1 Reset"]
pub type Mft16_1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT16_2_RST` reader - MFT16-2 Reset"]
pub type Mft16_2RstR = crate::BitReader;
#[doc = "Field `MFT16_2_RST` writer - MFT16-2 Reset"]
pub type Mft16_2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT16_3_RST` reader - MFT16-3 Reset"]
pub type Mft16_3RstR = crate::BitReader;
#[doc = "Field `MFT16_3_RST` writer - MFT16-3 Reset"]
pub type Mft16_3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB7_RST` reader - SMB7 Reset"]
pub type Smb7RstR = crate::BitReader;
#[doc = "Field `SMB7_RST` writer - SMB7 Reset"]
pub type Smb7RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT1_RST` reader - CR_UART 1 Reset"]
pub type Crurt1RstR = crate::BitReader;
#[doc = "Field `CRURT1_RST` writer - CR_UART 1 Reset"]
pub type Crurt1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_RST` reader - PS/2 Reset"]
pub type Ps2RstR = crate::BitReader;
#[doc = "Field `PS2_RST` writer - PS/2 Reset"]
pub type Ps2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDP_RST` reader - SDP Reset"]
pub type SdpRstR = crate::BitReader;
#[doc = "Field `SDP_RST` writer - SDP Reset"]
pub type SdpRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBS_RST` reader - Keyboard Scan Reset"]
pub type KbsRstR = crate::BitReader;
#[doc = "Field `KBS_RST` writer - Keyboard Scan Reset"]
pub type KbsRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PMC Reset"]
    #[inline(always)]
    pub fn pmc_rst(&self) -> PmcRstR {
        PmcRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SHI Reset"]
    #[inline(always)]
    pub fn shi_rst(&self) -> ShiRstR {
        ShiRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI Peripheral Reset"]
    #[inline(always)]
    pub fn spip_rst(&self) -> SpipRstR {
        SpipRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC_EREF Reset"]
    #[inline(always)]
    pub fn adce_rst(&self) -> AdceRstR {
        AdceRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PECI Reset"]
    #[inline(always)]
    pub fn peci_rst(&self) -> PeciRstR {
        PeciRstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CR_UART 2 Reset"]
    #[inline(always)]
    pub fn crurt2_rst(&self) -> Crurt2RstR {
        Crurt2RstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC_IREF Reset"]
    #[inline(always)]
    pub fn adci_rst(&self) -> AdciRstR {
        AdciRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMB0 Reset"]
    #[inline(always)]
    pub fn smb0_rst(&self) -> Smb0RstR {
        Smb0RstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SMB1 Reset"]
    #[inline(always)]
    pub fn smb1_rst(&self) -> Smb1RstR {
        Smb1RstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMB2 Reset"]
    #[inline(always)]
    pub fn smb2_rst(&self) -> Smb2RstR {
        Smb2RstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SMB3 Reset"]
    #[inline(always)]
    pub fn smb3_rst(&self) -> Smb3RstR {
        Smb3RstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SMB4 Reset"]
    #[inline(always)]
    pub fn smb4_rst(&self) -> Smb4RstR {
        Smb4RstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMB5 Reset"]
    #[inline(always)]
    pub fn smb5_rst(&self) -> Smb5RstR {
        Smb5RstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMB6 Reset"]
    #[inline(always)]
    pub fn smb6_rst(&self) -> Smb6RstR {
        Smb6RstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TWD Reset"]
    #[inline(always)]
    pub fn twd_rst(&self) -> TwdRstR {
        TwdRstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PWM0 Reset"]
    #[inline(always)]
    pub fn pwm0_rst(&self) -> Pwm0RstR {
        Pwm0RstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM1 Reset"]
    #[inline(always)]
    pub fn pwm1_rst(&self) -> Pwm1RstR {
        Pwm1RstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWM2 Reset"]
    #[inline(always)]
    pub fn pwm2_rst(&self) -> Pwm2RstR {
        Pwm2RstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWM3 Reset"]
    #[inline(always)]
    pub fn pwm3_rst(&self) -> Pwm3RstR {
        Pwm3RstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM4 Reset"]
    #[inline(always)]
    pub fn pwm4_rst(&self) -> Pwm4RstR {
        Pwm4RstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PWM5 Reset"]
    #[inline(always)]
    pub fn pwm5_rst(&self) -> Pwm5RstR {
        Pwm5RstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PWM6 Reset"]
    #[inline(always)]
    pub fn pwm6_rst(&self) -> Pwm6RstR {
        Pwm6RstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PWM7 Reset"]
    #[inline(always)]
    pub fn pwm7_rst(&self) -> Pwm7RstR {
        Pwm7RstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MFT16-1 Reset"]
    #[inline(always)]
    pub fn mft16_1_rst(&self) -> Mft16_1RstR {
        Mft16_1RstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MFT16-2 Reset"]
    #[inline(always)]
    pub fn mft16_2_rst(&self) -> Mft16_2RstR {
        Mft16_2RstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MFT16-3 Reset"]
    #[inline(always)]
    pub fn mft16_3_rst(&self) -> Mft16_3RstR {
        Mft16_3RstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SMB7 Reset"]
    #[inline(always)]
    pub fn smb7_rst(&self) -> Smb7RstR {
        Smb7RstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CR_UART 1 Reset"]
    #[inline(always)]
    pub fn crurt1_rst(&self) -> Crurt1RstR {
        Crurt1RstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PS/2 Reset"]
    #[inline(always)]
    pub fn ps2_rst(&self) -> Ps2RstR {
        Ps2RstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SDP Reset"]
    #[inline(always)]
    pub fn sdp_rst(&self) -> SdpRstR {
        SdpRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Keyboard Scan Reset"]
    #[inline(always)]
    pub fn kbs_rst(&self) -> KbsRstR {
        KbsRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRST_CTL2")
            .field("pmc_rst", &self.pmc_rst())
            .field("spip_rst", &self.spip_rst())
            .field("adce_rst", &self.adce_rst())
            .field("peci_rst", &self.peci_rst())
            .field("crurt2_rst", &self.crurt2_rst())
            .field("adci_rst", &self.adci_rst())
            .field("smb0_rst", &self.smb0_rst())
            .field("smb1_rst", &self.smb1_rst())
            .field("smb2_rst", &self.smb2_rst())
            .field("smb3_rst", &self.smb3_rst())
            .field("smb4_rst", &self.smb4_rst())
            .field("smb5_rst", &self.smb5_rst())
            .field("smb6_rst", &self.smb6_rst())
            .field("twd_rst", &self.twd_rst())
            .field("pwm0_rst", &self.pwm0_rst())
            .field("pwm1_rst", &self.pwm1_rst())
            .field("pwm2_rst", &self.pwm2_rst())
            .field("pwm3_rst", &self.pwm3_rst())
            .field("pwm4_rst", &self.pwm4_rst())
            .field("pwm5_rst", &self.pwm5_rst())
            .field("pwm6_rst", &self.pwm6_rst())
            .field("pwm7_rst", &self.pwm7_rst())
            .field("mft16_1_rst", &self.mft16_1_rst())
            .field("mft16_2_rst", &self.mft16_2_rst())
            .field("mft16_3_rst", &self.mft16_3_rst())
            .field("smb7_rst", &self.smb7_rst())
            .field("crurt1_rst", &self.crurt1_rst())
            .field("ps2_rst", &self.ps2_rst())
            .field("sdp_rst", &self.sdp_rst())
            .field("kbs_rst", &self.kbs_rst())
            .field("shi_rst", &self.shi_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PMC Reset"]
    #[inline(always)]
    pub fn pmc_rst(&mut self) -> PmcRstW<SwrstCtl2Spec> {
        PmcRstW::new(self, 0)
    }
    #[doc = "Bit 2 - SHI Reset"]
    #[inline(always)]
    pub fn shi_rst(&mut self) -> ShiRstW<SwrstCtl2Spec> {
        ShiRstW::new(self, 2)
    }
    #[doc = "Bit 3 - SPI Peripheral Reset"]
    #[inline(always)]
    pub fn spip_rst(&mut self) -> SpipRstW<SwrstCtl2Spec> {
        SpipRstW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC_EREF Reset"]
    #[inline(always)]
    pub fn adce_rst(&mut self) -> AdceRstW<SwrstCtl2Spec> {
        AdceRstW::new(self, 4)
    }
    #[doc = "Bit 5 - PECI Reset"]
    #[inline(always)]
    pub fn peci_rst(&mut self) -> PeciRstW<SwrstCtl2Spec> {
        PeciRstW::new(self, 5)
    }
    #[doc = "Bit 6 - CR_UART 2 Reset"]
    #[inline(always)]
    pub fn crurt2_rst(&mut self) -> Crurt2RstW<SwrstCtl2Spec> {
        Crurt2RstW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC_IREF Reset"]
    #[inline(always)]
    pub fn adci_rst(&mut self) -> AdciRstW<SwrstCtl2Spec> {
        AdciRstW::new(self, 7)
    }
    #[doc = "Bit 8 - SMB0 Reset"]
    #[inline(always)]
    pub fn smb0_rst(&mut self) -> Smb0RstW<SwrstCtl2Spec> {
        Smb0RstW::new(self, 8)
    }
    #[doc = "Bit 9 - SMB1 Reset"]
    #[inline(always)]
    pub fn smb1_rst(&mut self) -> Smb1RstW<SwrstCtl2Spec> {
        Smb1RstW::new(self, 9)
    }
    #[doc = "Bit 10 - SMB2 Reset"]
    #[inline(always)]
    pub fn smb2_rst(&mut self) -> Smb2RstW<SwrstCtl2Spec> {
        Smb2RstW::new(self, 10)
    }
    #[doc = "Bit 11 - SMB3 Reset"]
    #[inline(always)]
    pub fn smb3_rst(&mut self) -> Smb3RstW<SwrstCtl2Spec> {
        Smb3RstW::new(self, 11)
    }
    #[doc = "Bit 12 - SMB4 Reset"]
    #[inline(always)]
    pub fn smb4_rst(&mut self) -> Smb4RstW<SwrstCtl2Spec> {
        Smb4RstW::new(self, 12)
    }
    #[doc = "Bit 13 - SMB5 Reset"]
    #[inline(always)]
    pub fn smb5_rst(&mut self) -> Smb5RstW<SwrstCtl2Spec> {
        Smb5RstW::new(self, 13)
    }
    #[doc = "Bit 14 - SMB6 Reset"]
    #[inline(always)]
    pub fn smb6_rst(&mut self) -> Smb6RstW<SwrstCtl2Spec> {
        Smb6RstW::new(self, 14)
    }
    #[doc = "Bit 15 - TWD Reset"]
    #[inline(always)]
    pub fn twd_rst(&mut self) -> TwdRstW<SwrstCtl2Spec> {
        TwdRstW::new(self, 15)
    }
    #[doc = "Bit 16 - PWM0 Reset"]
    #[inline(always)]
    pub fn pwm0_rst(&mut self) -> Pwm0RstW<SwrstCtl2Spec> {
        Pwm0RstW::new(self, 16)
    }
    #[doc = "Bit 17 - PWM1 Reset"]
    #[inline(always)]
    pub fn pwm1_rst(&mut self) -> Pwm1RstW<SwrstCtl2Spec> {
        Pwm1RstW::new(self, 17)
    }
    #[doc = "Bit 18 - PWM2 Reset"]
    #[inline(always)]
    pub fn pwm2_rst(&mut self) -> Pwm2RstW<SwrstCtl2Spec> {
        Pwm2RstW::new(self, 18)
    }
    #[doc = "Bit 19 - PWM3 Reset"]
    #[inline(always)]
    pub fn pwm3_rst(&mut self) -> Pwm3RstW<SwrstCtl2Spec> {
        Pwm3RstW::new(self, 19)
    }
    #[doc = "Bit 20 - PWM4 Reset"]
    #[inline(always)]
    pub fn pwm4_rst(&mut self) -> Pwm4RstW<SwrstCtl2Spec> {
        Pwm4RstW::new(self, 20)
    }
    #[doc = "Bit 21 - PWM5 Reset"]
    #[inline(always)]
    pub fn pwm5_rst(&mut self) -> Pwm5RstW<SwrstCtl2Spec> {
        Pwm5RstW::new(self, 21)
    }
    #[doc = "Bit 22 - PWM6 Reset"]
    #[inline(always)]
    pub fn pwm6_rst(&mut self) -> Pwm6RstW<SwrstCtl2Spec> {
        Pwm6RstW::new(self, 22)
    }
    #[doc = "Bit 23 - PWM7 Reset"]
    #[inline(always)]
    pub fn pwm7_rst(&mut self) -> Pwm7RstW<SwrstCtl2Spec> {
        Pwm7RstW::new(self, 23)
    }
    #[doc = "Bit 24 - MFT16-1 Reset"]
    #[inline(always)]
    pub fn mft16_1_rst(&mut self) -> Mft16_1RstW<SwrstCtl2Spec> {
        Mft16_1RstW::new(self, 24)
    }
    #[doc = "Bit 25 - MFT16-2 Reset"]
    #[inline(always)]
    pub fn mft16_2_rst(&mut self) -> Mft16_2RstW<SwrstCtl2Spec> {
        Mft16_2RstW::new(self, 25)
    }
    #[doc = "Bit 26 - MFT16-3 Reset"]
    #[inline(always)]
    pub fn mft16_3_rst(&mut self) -> Mft16_3RstW<SwrstCtl2Spec> {
        Mft16_3RstW::new(self, 26)
    }
    #[doc = "Bit 27 - SMB7 Reset"]
    #[inline(always)]
    pub fn smb7_rst(&mut self) -> Smb7RstW<SwrstCtl2Spec> {
        Smb7RstW::new(self, 27)
    }
    #[doc = "Bit 28 - CR_UART 1 Reset"]
    #[inline(always)]
    pub fn crurt1_rst(&mut self) -> Crurt1RstW<SwrstCtl2Spec> {
        Crurt1RstW::new(self, 28)
    }
    #[doc = "Bit 29 - PS/2 Reset"]
    #[inline(always)]
    pub fn ps2_rst(&mut self) -> Ps2RstW<SwrstCtl2Spec> {
        Ps2RstW::new(self, 29)
    }
    #[doc = "Bit 30 - SDP Reset"]
    #[inline(always)]
    pub fn sdp_rst(&mut self) -> SdpRstW<SwrstCtl2Spec> {
        SdpRstW::new(self, 30)
    }
    #[doc = "Bit 31 - Keyboard Scan Reset"]
    #[inline(always)]
    pub fn kbs_rst(&mut self) -> KbsRstW<SwrstCtl2Spec> {
        KbsRstW::new(self, 31)
    }
}
#[doc = "Software Reset Control 2 Register (SWRST_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCtl2Spec;
impl crate::RegisterSpec for SwrstCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_ctl2::R`](R) reader structure"]
impl crate::Readable for SwrstCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`swrst_ctl2::W`](W) writer structure"]
impl crate::Writable for SwrstCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRST_CTL2 to value 0"]
impl crate::Resettable for SwrstCtl2Spec {
    const RESET_VALUE: u32 = 0;
}
