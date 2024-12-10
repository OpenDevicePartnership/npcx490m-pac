#[doc = "Register `THRCTS` reader"]
pub type R = crate::R<ThrctsSpec>;
#[doc = "Register `THRCTS` writer"]
pub type W = crate::W<ThrctsSpec>;
#[doc = "Field `THR1_STS` reader - Threshold 1 Status"]
pub type Thr1StsR = crate::BitReader;
#[doc = "Field `THR1_STS` writer - Threshold 1 Status"]
pub type Thr1StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR2_STS` reader - Threshold 2 Status"]
pub type Thr2StsR = crate::BitReader;
#[doc = "Field `THR2_STS` writer - Threshold 2 Status"]
pub type Thr2StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR3_STS` reader - Threshold 3 Status"]
pub type Thr3StsR = crate::BitReader;
#[doc = "Field `THR3_STS` writer - Threshold 3 Status"]
pub type Thr3StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR4_STS` reader - Threshold 4 Status"]
pub type Thr4StsR = crate::BitReader;
#[doc = "Field `THR4_STS` writer - Threshold 4 Status"]
pub type Thr4StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR5_STS` reader - Threshold 5 Status"]
pub type Thr5StsR = crate::BitReader;
#[doc = "Field `THR5_STS` writer - Threshold 5 Status"]
pub type Thr5StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR6_STS` reader - Threshold 6 Status"]
pub type Thr6StsR = crate::BitReader;
#[doc = "Field `THR6_STS` writer - Threshold 6 Status"]
pub type Thr6StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVENT` reader - ADC Event Status"]
pub type AdcEventR = crate::BitReader;
#[doc = "Field `ADC_EVENT` writer - ADC Event Status"]
pub type AdcEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR1_IEN` reader - Threshold 1 Interrupt Enable"]
pub type Thr1IenR = crate::BitReader;
#[doc = "Field `THR1_IEN` writer - Threshold 1 Interrupt Enable"]
pub type Thr1IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR2_IEN` reader - Threshold 2 Interrupt Enable"]
pub type Thr2IenR = crate::BitReader;
#[doc = "Field `THR2_IEN` writer - Threshold 2 Interrupt Enable"]
pub type Thr2IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR3_IEN` reader - Threshold 3 Interrupt Enable"]
pub type Thr3IenR = crate::BitReader;
#[doc = "Field `THR3_IEN` writer - Threshold 3 Interrupt Enable"]
pub type Thr3IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR4_IEN` reader - Threshold 4 Interrupt Enable"]
pub type Thr4IenR = crate::BitReader;
#[doc = "Field `THR4_IEN` writer - Threshold 4 Interrupt Enable"]
pub type Thr4IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR5_IEN` reader - Threshold 5 Interrupt Enable"]
pub type Thr5IenR = crate::BitReader;
#[doc = "Field `THR5_IEN` writer - Threshold 5 Interrupt Enable"]
pub type Thr5IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR6_IEN` reader - Threshold 6 Interrupt Enable"]
pub type Thr6IenR = crate::BitReader;
#[doc = "Field `THR6_IEN` writer - Threshold 6 Interrupt Enable"]
pub type Thr6IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_WKEN` reader - ADC Wake-Up Enable"]
pub type AdcWkenR = crate::BitReader;
#[doc = "Field `ADC_WKEN` writer - ADC Wake-Up Enable"]
pub type AdcWkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Threshold 1 Status"]
    #[inline(always)]
    pub fn thr1_sts(&self) -> Thr1StsR {
        Thr1StsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Threshold 2 Status"]
    #[inline(always)]
    pub fn thr2_sts(&self) -> Thr2StsR {
        Thr2StsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Threshold 3 Status"]
    #[inline(always)]
    pub fn thr3_sts(&self) -> Thr3StsR {
        Thr3StsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Threshold 4 Status"]
    #[inline(always)]
    pub fn thr4_sts(&self) -> Thr4StsR {
        Thr4StsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Threshold 5 Status"]
    #[inline(always)]
    pub fn thr5_sts(&self) -> Thr5StsR {
        Thr5StsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Threshold 6 Status"]
    #[inline(always)]
    pub fn thr6_sts(&self) -> Thr6StsR {
        Thr6StsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Event Status"]
    #[inline(always)]
    pub fn adc_event(&self) -> AdcEventR {
        AdcEventR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Threshold 1 Interrupt Enable"]
    #[inline(always)]
    pub fn thr1_ien(&self) -> Thr1IenR {
        Thr1IenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Threshold 2 Interrupt Enable"]
    #[inline(always)]
    pub fn thr2_ien(&self) -> Thr2IenR {
        Thr2IenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Threshold 3 Interrupt Enable"]
    #[inline(always)]
    pub fn thr3_ien(&self) -> Thr3IenR {
        Thr3IenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Threshold 4 Interrupt Enable"]
    #[inline(always)]
    pub fn thr4_ien(&self) -> Thr4IenR {
        Thr4IenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Threshold 5 Interrupt Enable"]
    #[inline(always)]
    pub fn thr5_ien(&self) -> Thr5IenR {
        Thr5IenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Threshold 6 Interrupt Enable"]
    #[inline(always)]
    pub fn thr6_ien(&self) -> Thr6IenR {
        Thr6IenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC Wake-Up Enable"]
    #[inline(always)]
    pub fn adc_wken(&self) -> AdcWkenR {
        AdcWkenR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRCTS")
            .field("thr1_sts", &self.thr1_sts())
            .field("thr2_sts", &self.thr2_sts())
            .field("thr3_sts", &self.thr3_sts())
            .field("thr4_sts", &self.thr4_sts())
            .field("thr5_sts", &self.thr5_sts())
            .field("thr6_sts", &self.thr6_sts())
            .field("adc_event", &self.adc_event())
            .field("thr1_ien", &self.thr1_ien())
            .field("thr2_ien", &self.thr2_ien())
            .field("thr3_ien", &self.thr3_ien())
            .field("thr4_ien", &self.thr4_ien())
            .field("thr5_ien", &self.thr5_ien())
            .field("thr6_ien", &self.thr6_ien())
            .field("adc_wken", &self.adc_wken())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Threshold 1 Status"]
    #[inline(always)]
    pub fn thr1_sts(&mut self) -> Thr1StsW<ThrctsSpec> {
        Thr1StsW::new(self, 0)
    }
    #[doc = "Bit 1 - Threshold 2 Status"]
    #[inline(always)]
    pub fn thr2_sts(&mut self) -> Thr2StsW<ThrctsSpec> {
        Thr2StsW::new(self, 1)
    }
    #[doc = "Bit 2 - Threshold 3 Status"]
    #[inline(always)]
    pub fn thr3_sts(&mut self) -> Thr3StsW<ThrctsSpec> {
        Thr3StsW::new(self, 2)
    }
    #[doc = "Bit 3 - Threshold 4 Status"]
    #[inline(always)]
    pub fn thr4_sts(&mut self) -> Thr4StsW<ThrctsSpec> {
        Thr4StsW::new(self, 3)
    }
    #[doc = "Bit 4 - Threshold 5 Status"]
    #[inline(always)]
    pub fn thr5_sts(&mut self) -> Thr5StsW<ThrctsSpec> {
        Thr5StsW::new(self, 4)
    }
    #[doc = "Bit 5 - Threshold 6 Status"]
    #[inline(always)]
    pub fn thr6_sts(&mut self) -> Thr6StsW<ThrctsSpec> {
        Thr6StsW::new(self, 5)
    }
    #[doc = "Bit 7 - ADC Event Status"]
    #[inline(always)]
    pub fn adc_event(&mut self) -> AdcEventW<ThrctsSpec> {
        AdcEventW::new(self, 7)
    }
    #[doc = "Bit 8 - Threshold 1 Interrupt Enable"]
    #[inline(always)]
    pub fn thr1_ien(&mut self) -> Thr1IenW<ThrctsSpec> {
        Thr1IenW::new(self, 8)
    }
    #[doc = "Bit 9 - Threshold 2 Interrupt Enable"]
    #[inline(always)]
    pub fn thr2_ien(&mut self) -> Thr2IenW<ThrctsSpec> {
        Thr2IenW::new(self, 9)
    }
    #[doc = "Bit 10 - Threshold 3 Interrupt Enable"]
    #[inline(always)]
    pub fn thr3_ien(&mut self) -> Thr3IenW<ThrctsSpec> {
        Thr3IenW::new(self, 10)
    }
    #[doc = "Bit 11 - Threshold 4 Interrupt Enable"]
    #[inline(always)]
    pub fn thr4_ien(&mut self) -> Thr4IenW<ThrctsSpec> {
        Thr4IenW::new(self, 11)
    }
    #[doc = "Bit 12 - Threshold 5 Interrupt Enable"]
    #[inline(always)]
    pub fn thr5_ien(&mut self) -> Thr5IenW<ThrctsSpec> {
        Thr5IenW::new(self, 12)
    }
    #[doc = "Bit 13 - Threshold 6 Interrupt Enable"]
    #[inline(always)]
    pub fn thr6_ien(&mut self) -> Thr6IenW<ThrctsSpec> {
        Thr6IenW::new(self, 13)
    }
    #[doc = "Bit 15 - ADC Wake-Up Enable"]
    #[inline(always)]
    pub fn adc_wken(&mut self) -> AdcWkenW<ThrctsSpec> {
        AdcWkenW::new(self, 15)
    }
}
#[doc = "Threshold Status Register (THRCTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrcts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrcts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrctsSpec;
impl crate::RegisterSpec for ThrctsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`thrcts::R`](R) reader structure"]
impl crate::Readable for ThrctsSpec {}
#[doc = "`write(|w| ..)` method takes [`thrcts::W`](W) writer structure"]
impl crate::Writable for ThrctsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets THRCTS to value 0"]
impl crate::Resettable for ThrctsSpec {
    const RESET_VALUE: u16 = 0;
}
