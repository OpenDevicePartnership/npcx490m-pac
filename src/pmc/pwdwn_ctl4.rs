#[doc = "Register `PWDWN_CTL4` reader"]
pub type R = crate::R<PwdwnCtl4Spec>;
#[doc = "Register `PWDWN_CTL4` writer"]
pub type W = crate::W<PwdwnCtl4Spec>;
#[doc = "Field `ITIM1_PD` reader - ITIM32-1 Power-Down"]
pub type Itim1PdR = crate::BitReader;
#[doc = "Field `ITIM1_PD` writer - ITIM32-1 Power-Down"]
pub type Itim1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM2_PD` reader - ITIM32-2 Power-Down"]
pub type Itim2PdR = crate::BitReader;
#[doc = "Field `ITIM2_PD` writer - ITIM32-2 Power-Down"]
pub type Itim2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM3_PD` reader - ITIM32-3 Power-Down"]
pub type Itim3PdR = crate::BitReader;
#[doc = "Field `ITIM3_PD` writer - ITIM32-3 Power-Down"]
pub type Itim3PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EREF_PD` reader - ADC_EREF Power-Down"]
pub type AdcErefPdR = crate::BitReader;
#[doc = "Field `ADC_EREF_PD` writer - ADC_EREF Power-Down"]
pub type AdcErefPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_IREF_PD` reader - ADC_IREF Power-Down"]
pub type AdcIrefPdR = crate::BitReader;
#[doc = "Field `ADC_IREF_PD` writer - ADC_IREF Power-Down"]
pub type AdcIrefPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECI_PD` reader - PECI Power-Down"]
pub type PeciPdR = crate::BitReader;
#[doc = "Field `PECI_PD` writer - PECI Power-Down"]
pub type PeciPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIP_PD` reader - SPI Peripheral Power-Down"]
pub type SpipPdR = crate::BitReader;
#[doc = "Field `SPIP_PD` writer - SPI Peripheral Power-Down"]
pub type SpipPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ITIM32-1 Power-Down"]
    #[inline(always)]
    pub fn itim1_pd(&self) -> Itim1PdR {
        Itim1PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITIM32-2 Power-Down"]
    #[inline(always)]
    pub fn itim2_pd(&self) -> Itim2PdR {
        Itim2PdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ITIM32-3 Power-Down"]
    #[inline(always)]
    pub fn itim3_pd(&self) -> Itim3PdR {
        Itim3PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC_EREF Power-Down"]
    #[inline(always)]
    pub fn adc_eref_pd(&self) -> AdcErefPdR {
        AdcErefPdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC_IREF Power-Down"]
    #[inline(always)]
    pub fn adc_iref_pd(&self) -> AdcIrefPdR {
        AdcIrefPdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PECI Power-Down"]
    #[inline(always)]
    pub fn peci_pd(&self) -> PeciPdR {
        PeciPdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Peripheral Power-Down"]
    #[inline(always)]
    pub fn spip_pd(&self) -> SpipPdR {
        SpipPdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL4")
            .field("itim1_pd", &self.itim1_pd())
            .field("itim2_pd", &self.itim2_pd())
            .field("itim3_pd", &self.itim3_pd())
            .field("adc_eref_pd", &self.adc_eref_pd())
            .field("adc_iref_pd", &self.adc_iref_pd())
            .field("peci_pd", &self.peci_pd())
            .field("spip_pd", &self.spip_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ITIM32-1 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn itim1_pd(&mut self) -> Itim1PdW<PwdwnCtl4Spec> {
        Itim1PdW::new(self, 0)
    }
    #[doc = "Bit 1 - ITIM32-2 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn itim2_pd(&mut self) -> Itim2PdW<PwdwnCtl4Spec> {
        Itim2PdW::new(self, 1)
    }
    #[doc = "Bit 2 - ITIM32-3 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn itim3_pd(&mut self) -> Itim3PdW<PwdwnCtl4Spec> {
        Itim3PdW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC_EREF Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn adc_eref_pd(&mut self) -> AdcErefPdW<PwdwnCtl4Spec> {
        AdcErefPdW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC_IREF Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn adc_iref_pd(&mut self) -> AdcIrefPdW<PwdwnCtl4Spec> {
        AdcIrefPdW::new(self, 4)
    }
    #[doc = "Bit 5 - PECI Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn peci_pd(&mut self) -> PeciPdW<PwdwnCtl4Spec> {
        PeciPdW::new(self, 5)
    }
    #[doc = "Bit 7 - SPI Peripheral Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn spip_pd(&mut self) -> SpipPdW<PwdwnCtl4Spec> {
        SpipPdW::new(self, 7)
    }
}
#[doc = "Power-Down Control 4 Register (PWDWN_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl4Spec;
impl crate::RegisterSpec for PwdwnCtl4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl4::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl4::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL4 to value 0"]
impl crate::Resettable for PwdwnCtl4Spec {
    const RESET_VALUE: u8 = 0;
}
