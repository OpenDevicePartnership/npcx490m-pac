#[doc = "Register `ENSLP_CTL` reader"]
pub type R = crate::R<EnslpCtlSpec>;
#[doc = "Register `ENSLP_CTL` writer"]
pub type W = crate::W<EnslpCtlSpec>;
#[doc = "Field `ESPI_FMCLK_ENSLP` reader - eSPI_SIF FMCLK Enable in Sleep"]
pub type EspiFmclkEnslpR = crate::BitReader;
#[doc = "Field `ESPI_FMCLK_ENSLP` writer - eSPI_SIF FMCLK Enable in Sleep"]
pub type EspiFmclkEnslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_ACC_DIS` reader - ADC Access is Disabled"]
pub type AdcAccDisR = crate::BitReader;
#[doc = "Field `ADC_ACC_DIS` writer - ADC Access is Disabled"]
pub type AdcAccDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECI_ENSLP` reader - PECI Enable in Sleep"]
pub type PeciEnslpR = crate::BitReader;
#[doc = "Field `PECI_ENSLP` writer - PECI Enable in Sleep"]
pub type PeciEnslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_WK_CTLX` reader - Low-Power Wake-Up Control"]
pub type LpWkCtlxR = crate::FieldReader;
#[doc = "Field `LP_WK_CTLX` writer - Low-Power Wake-Up Control"]
pub type LpWkCtlxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_EREF_LFSL` reader - ADC_EREF Low-Frequency Clock Select"]
pub type AdcErefLfslR = crate::BitReader;
#[doc = "Field `ADC_EREF_LFSL` writer - ADC_EREF Low-Frequency Clock Select"]
pub type AdcErefLfslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_IREF_LFSL` reader - ADC_IREF Low-Frequency Clock Select"]
pub type AdcIrefLfslR = crate::BitReader;
#[doc = "Field `ADC_IREF_LFSL` writer - ADC_IREF Low-Frequency Clock Select"]
pub type AdcIrefLfslW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - eSPI_SIF FMCLK Enable in Sleep"]
    #[inline(always)]
    pub fn espi_fmclk_enslp(&self) -> EspiFmclkEnslpR {
        EspiFmclkEnslpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Access is Disabled"]
    #[inline(always)]
    pub fn adc_acc_dis(&self) -> AdcAccDisR {
        AdcAccDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PECI Enable in Sleep"]
    #[inline(always)]
    pub fn peci_enslp(&self) -> PeciEnslpR {
        PeciEnslpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low-Power Wake-Up Control"]
    #[inline(always)]
    pub fn lp_wk_ctlx(&self) -> LpWkCtlxR {
        LpWkCtlxR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - ADC_EREF Low-Frequency Clock Select"]
    #[inline(always)]
    pub fn adc_eref_lfsl(&self) -> AdcErefLfslR {
        AdcErefLfslR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC_IREF Low-Frequency Clock Select"]
    #[inline(always)]
    pub fn adc_iref_lfsl(&self) -> AdcIrefLfslR {
        AdcIrefLfslR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENSLP_CTL")
            .field("espi_fmclk_enslp", &self.espi_fmclk_enslp())
            .field("adc_acc_dis", &self.adc_acc_dis())
            .field("peci_enslp", &self.peci_enslp())
            .field("lp_wk_ctlx", &self.lp_wk_ctlx())
            .field("adc_eref_lfsl", &self.adc_eref_lfsl())
            .field("adc_iref_lfsl", &self.adc_iref_lfsl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - eSPI_SIF FMCLK Enable in Sleep"]
    #[inline(always)]
    pub fn espi_fmclk_enslp(&mut self) -> EspiFmclkEnslpW<EnslpCtlSpec> {
        EspiFmclkEnslpW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Access is Disabled"]
    #[inline(always)]
    pub fn adc_acc_dis(&mut self) -> AdcAccDisW<EnslpCtlSpec> {
        AdcAccDisW::new(self, 1)
    }
    #[doc = "Bit 2 - PECI Enable in Sleep"]
    #[inline(always)]
    pub fn peci_enslp(&mut self) -> PeciEnslpW<EnslpCtlSpec> {
        PeciEnslpW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Low-Power Wake-Up Control"]
    #[inline(always)]
    pub fn lp_wk_ctlx(&mut self) -> LpWkCtlxW<EnslpCtlSpec> {
        LpWkCtlxW::new(self, 4)
    }
    #[doc = "Bit 6 - ADC_EREF Low-Frequency Clock Select"]
    #[inline(always)]
    pub fn adc_eref_lfsl(&mut self) -> AdcErefLfslW<EnslpCtlSpec> {
        AdcErefLfslW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC_IREF Low-Frequency Clock Select"]
    #[inline(always)]
    pub fn adc_iref_lfsl(&mut self) -> AdcIrefLfslW<EnslpCtlSpec> {
        AdcIrefLfslW::new(self, 7)
    }
}
#[doc = "Enable in Sleep Control Register (ENSLP_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`enslp_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enslp_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnslpCtlSpec;
impl crate::RegisterSpec for EnslpCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`enslp_ctl::R`](R) reader structure"]
impl crate::Readable for EnslpCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`enslp_ctl::W`](W) writer structure"]
impl crate::Writable for EnslpCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ENSLP_CTL to value 0"]
impl crate::Resettable for EnslpCtlSpec {
    const RESET_VALUE: u8 = 0;
}
