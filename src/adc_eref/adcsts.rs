#[doc = "Register `ADCSTS` reader"]
pub type R = crate::R<AdcstsSpec>;
#[doc = "Register `ADCSTS` writer"]
pub type W = crate::W<AdcstsSpec>;
#[doc = "Field `EOCEV` reader - End Of Conversion Event"]
pub type EocevR = crate::BitReader;
#[doc = "Field `EOCEV` writer - End Of Conversion Event"]
pub type EocevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCCEV` reader - End Of Conversion Cycle Event"]
pub type EoccevR = crate::BitReader;
#[doc = "Field `EOCCEV` writer - End Of Conversion Cycle Event"]
pub type EoccevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTCEV` reader - End Of Timer-Triggered Conversion Event"]
pub type EotcevR = crate::BitReader;
#[doc = "Field `EOTCEV` writer - End Of Timer-Triggered Conversion Event"]
pub type EotcevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFEV` reader - Data Overflow Event"]
pub type OvfevR = crate::BitReader;
#[doc = "Field `OVFEV` writer - Data Overflow Event"]
pub type OvfevW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End Of Conversion Event"]
    #[inline(always)]
    pub fn eocev(&self) -> EocevR {
        EocevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Of Conversion Cycle Event"]
    #[inline(always)]
    pub fn eoccev(&self) -> EoccevR {
        EoccevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Of Timer-Triggered Conversion Event"]
    #[inline(always)]
    pub fn eotcev(&self) -> EotcevR {
        EotcevR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Overflow Event"]
    #[inline(always)]
    pub fn ovfev(&self) -> OvfevR {
        OvfevR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCSTS")
            .field("eocev", &self.eocev())
            .field("eoccev", &self.eoccev())
            .field("eotcev", &self.eotcev())
            .field("ovfev", &self.ovfev())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - End Of Conversion Event"]
    #[inline(always)]
    pub fn eocev(&mut self) -> EocevW<AdcstsSpec> {
        EocevW::new(self, 0)
    }
    #[doc = "Bit 1 - End Of Conversion Cycle Event"]
    #[inline(always)]
    pub fn eoccev(&mut self) -> EoccevW<AdcstsSpec> {
        EoccevW::new(self, 1)
    }
    #[doc = "Bit 2 - End Of Timer-Triggered Conversion Event"]
    #[inline(always)]
    pub fn eotcev(&mut self) -> EotcevW<AdcstsSpec> {
        EotcevW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Overflow Event"]
    #[inline(always)]
    pub fn ovfev(&mut self) -> OvfevW<AdcstsSpec> {
        OvfevW::new(self, 3)
    }
}
#[doc = "ADC Status Register (ADCSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcstsSpec;
impl crate::RegisterSpec for AdcstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcsts::R`](R) reader structure"]
impl crate::Readable for AdcstsSpec {}
#[doc = "`write(|w| ..)` method takes [`adcsts::W`](W) writer structure"]
impl crate::Writable for AdcstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADCSTS to value 0"]
impl crate::Resettable for AdcstsSpec {
    const RESET_VALUE: u16 = 0;
}
