#[doc = "Register `ADCCNF` reader"]
pub type R = crate::R<AdccnfSpec>;
#[doc = "Register `ADCCNF` writer"]
pub type W = crate::W<AdccnfSpec>;
#[doc = "Field `ADCEN` reader - ADC Module Enable"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC Module Enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCMD` reader - ADC Conversion Mode"]
pub type AdcmdR = crate::FieldReader;
#[doc = "Field `ADCMD` writer - ADC Conversion Mode"]
pub type AdcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCRPTC` reader - ADC Repetitive Mode"]
pub type AdcrptcR = crate::BitReader;
#[doc = "Field `ADCRPTC` writer - ADC Repetitive Mode"]
pub type AdcrptcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - ADC Start Conversion"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - ADC Start Conversion"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCTTE` reader - ADC Timer-Triggered Conversion Enable"]
pub type AdctteR = crate::BitReader;
#[doc = "Field `ADCTTE` writer - ADC Timer-Triggered Conversion Enable"]
pub type AdctteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTECEN` reader - Interrupt from End Of Conversion Event Enable"]
pub type IntecenR = crate::BitReader;
#[doc = "Field `INTECEN` writer - Interrupt from End Of Conversion Event Enable"]
pub type IntecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTECCEN` reader - Interrupt from End Of Cyclic-Conversion Event Enable"]
pub type InteccenR = crate::BitReader;
#[doc = "Field `INTECCEN` writer - Interrupt from End Of Cyclic-Conversion Event Enable"]
pub type InteccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTETCEN` reader - Interrupt from End Of Timer-Triggered Conversion Event Enable"]
pub type IntetcenR = crate::BitReader;
#[doc = "Field `INTETCEN` writer - Interrupt from End Of Timer-Triggered Conversion Event Enable"]
pub type IntetcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTOVFEN` reader - Interrupt from Data Overflow Event Enable"]
pub type IntovfenR = crate::BitReader;
#[doc = "Field `INTOVFEN` writer - Interrupt from Data Overflow Event Enable"]
pub type IntovfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - ADC Stop Conversion"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - ADC Stop Conversion"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Module Enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ADC Conversion Mode"]
    #[inline(always)]
    pub fn adcmd(&self) -> AdcmdR {
        AdcmdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - ADC Repetitive Mode"]
    #[inline(always)]
    pub fn adcrptc(&self) -> AdcrptcR {
        AdcrptcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Start Conversion"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC Timer-Triggered Conversion Enable"]
    #[inline(always)]
    pub fn adctte(&self) -> AdctteR {
        AdctteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt from End Of Conversion Event Enable"]
    #[inline(always)]
    pub fn intecen(&self) -> IntecenR {
        IntecenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt from End Of Cyclic-Conversion Event Enable"]
    #[inline(always)]
    pub fn inteccen(&self) -> InteccenR {
        InteccenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt from End Of Timer-Triggered Conversion Event Enable"]
    #[inline(always)]
    pub fn intetcen(&self) -> IntetcenR {
        IntetcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt from Data Overflow Event Enable"]
    #[inline(always)]
    pub fn intovfen(&self) -> IntovfenR {
        IntovfenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC Stop Conversion"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCNF")
            .field("adcen", &self.adcen())
            .field("adcmd", &self.adcmd())
            .field("adcrptc", &self.adcrptc())
            .field("start", &self.start())
            .field("adctte", &self.adctte())
            .field("intecen", &self.intecen())
            .field("inteccen", &self.inteccen())
            .field("intetcen", &self.intetcen())
            .field("intovfen", &self.intovfen())
            .field("stop", &self.stop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module Enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<AdccnfSpec> {
        AdcenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - ADC Conversion Mode"]
    #[inline(always)]
    pub fn adcmd(&mut self) -> AdcmdW<AdccnfSpec> {
        AdcmdW::new(self, 1)
    }
    #[doc = "Bit 3 - ADC Repetitive Mode"]
    #[inline(always)]
    pub fn adcrptc(&mut self) -> AdcrptcW<AdccnfSpec> {
        AdcrptcW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Start Conversion"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<AdccnfSpec> {
        StartW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC Timer-Triggered Conversion Enable"]
    #[inline(always)]
    pub fn adctte(&mut self) -> AdctteW<AdccnfSpec> {
        AdctteW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt from End Of Conversion Event Enable"]
    #[inline(always)]
    pub fn intecen(&mut self) -> IntecenW<AdccnfSpec> {
        IntecenW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt from End Of Cyclic-Conversion Event Enable"]
    #[inline(always)]
    pub fn inteccen(&mut self) -> InteccenW<AdccnfSpec> {
        InteccenW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt from End Of Timer-Triggered Conversion Event Enable"]
    #[inline(always)]
    pub fn intetcen(&mut self) -> IntetcenW<AdccnfSpec> {
        IntetcenW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt from Data Overflow Event Enable"]
    #[inline(always)]
    pub fn intovfen(&mut self) -> IntovfenW<AdccnfSpec> {
        IntovfenW::new(self, 9)
    }
    #[doc = "Bit 11 - ADC Stop Conversion"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<AdccnfSpec> {
        StopW::new(self, 11)
    }
}
#[doc = "ADC Configuration Register (ADCCNF)\n\nYou can [`read`](crate::Reg::read) this register and get [`adccnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdccnfSpec;
impl crate::RegisterSpec for AdccnfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adccnf::R`](R) reader structure"]
impl crate::Readable for AdccnfSpec {}
#[doc = "`write(|w| ..)` method takes [`adccnf::W`](W) writer structure"]
impl crate::Writable for AdccnfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADCCNF to value 0"]
impl crate::Resettable for AdccnfSpec {
    const RESET_VALUE: u16 = 0;
}
