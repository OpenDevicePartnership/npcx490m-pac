#[doc = "Register `ATCTL` reader"]
pub type R = crate::R<AtctlSpec>;
#[doc = "Register `ATCTL` writer"]
pub type W = crate::W<AtctlSpec>;
#[doc = "Field `SCLKDIV` reader - Basic Clock Division Factor"]
pub type SclkdivR = crate::FieldReader;
#[doc = "Field `SCLKDIV` writer - Basic Clock Division Factor"]
pub type SclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DLY` reader - Channel Delay"]
pub type DlyR = crate::FieldReader;
#[doc = "Field `DLY` writer - Channel Delay"]
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADC_MFT_SEL` reader - ADC MFT Select"]
pub type AdcMftSelR = crate::FieldReader;
#[doc = "Field `ADC_MFT_SEL` writer - ADC MFT Select"]
pub type AdcMftSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Basic Clock Division Factor"]
    #[inline(always)]
    pub fn sclkdiv(&self) -> SclkdivR {
        SclkdivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Channel Delay"]
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - ADC MFT Select"]
    #[inline(always)]
    pub fn adc_mft_sel(&self) -> AdcMftSelR {
        AdcMftSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATCTL")
            .field("sclkdiv", &self.sclkdiv())
            .field("dly", &self.dly())
            .field("adc_mft_sel", &self.adc_mft_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Basic Clock Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn sclkdiv(&mut self) -> SclkdivW<AtctlSpec> {
        SclkdivW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Channel Delay"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DlyW<AtctlSpec> {
        DlyW::new(self, 8)
    }
    #[doc = "Bits 12:15 - ADC MFT Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_mft_sel(&mut self) -> AdcMftSelW<AtctlSpec> {
        AdcMftSelW::new(self, 12)
    }
}
#[doc = "ADC Timing Control Register (ATCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`atctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtctlSpec;
impl crate::RegisterSpec for AtctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`atctl::R`](R) reader structure"]
impl crate::Readable for AtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`atctl::W`](W) writer structure"]
impl crate::Writable for AtctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ATCTL to value 0"]
impl crate::Resettable for AtctlSpec {
    const RESET_VALUE: u16 = 0;
}
