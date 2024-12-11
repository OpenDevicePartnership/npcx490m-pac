#[doc = "Register `PMCSR` reader"]
pub type R = crate::R<PmcsrSpec>;
#[doc = "Register `PMCSR` writer"]
pub type W = crate::W<PmcsrSpec>;
#[doc = "Field `DHF` reader - Disable High-Frequency Oscillator"]
pub type DhfR = crate::BitReader;
#[doc = "Field `DHF` writer - Disable High-Frequency Oscillator"]
pub type DhfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - Sleep Power States"]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - Sleep Power States"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OHFC` reader - Oscillating High-Frequency Clock"]
pub type OhfcR = crate::BitReader;
#[doc = "Field `OHFC` writer - Oscillating High-Frequency Clock"]
pub type OhfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OLFC` reader - Oscillating Low-Frequency Clock"]
pub type OlfcR = crate::BitReader;
#[doc = "Field `OLFC` writer - Oscillating Low-Frequency Clock"]
pub type OlfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Disable High-Frequency Oscillator"]
    #[inline(always)]
    pub fn dhf(&self) -> DhfR {
        DhfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep Power States"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillating High-Frequency Clock"]
    #[inline(always)]
    pub fn ohfc(&self) -> OhfcR {
        OhfcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillating Low-Frequency Clock"]
    #[inline(always)]
    pub fn olfc(&self) -> OlfcR {
        OlfcR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCSR")
            .field("dhf", &self.dhf())
            .field("sleep", &self.sleep())
            .field("ohfc", &self.ohfc())
            .field("olfc", &self.olfc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Disable High-Frequency Oscillator"]
    #[inline(always)]
    pub fn dhf(&mut self) -> DhfW<PmcsrSpec> {
        DhfW::new(self, 1)
    }
    #[doc = "Bit 2 - Sleep Power States"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SleepW<PmcsrSpec> {
        SleepW::new(self, 2)
    }
    #[doc = "Bit 6 - Oscillating High-Frequency Clock"]
    #[inline(always)]
    pub fn ohfc(&mut self) -> OhfcW<PmcsrSpec> {
        OhfcW::new(self, 6)
    }
    #[doc = "Bit 7 - Oscillating Low-Frequency Clock"]
    #[inline(always)]
    pub fn olfc(&mut self) -> OlfcW<PmcsrSpec> {
        OlfcW::new(self, 7)
    }
}
#[doc = "Power Management Controller Status Register (PMCSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcsrSpec;
impl crate::RegisterSpec for PmcsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmcsr::R`](R) reader structure"]
impl crate::Readable for PmcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmcsr::W`](W) writer structure"]
impl crate::Writable for PmcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PMCSR to value 0"]
impl crate::Resettable for PmcsrSpec {
    const RESET_VALUE: u8 = 0;
}
