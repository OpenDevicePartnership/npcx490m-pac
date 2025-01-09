#[doc = "Register `DEVALTD_LK` reader"]
pub type R = crate::R<DevaltdLkSpec>;
#[doc = "Register `DEVALTD_LK` writer"]
pub type W = crate::W<DevaltdLkSpec>;
#[doc = "Field `PSL_IN1_AHI_LK` reader - PSL_IN1 Active-High Lock"]
pub type PslIn1AhiLkR = crate::BitReader;
#[doc = "Field `PSL_IN1_AHI_LK` writer - PSL_IN1 Active-High Lock"]
pub type PslIn1AhiLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nPSL_IN1_SL_LK` reader - PSL_IN1 Select Lock"]
pub type NPslIn1SlLkR = crate::BitReader;
#[doc = "Field `nPSL_IN1_SL_LK` writer - PSL_IN1 Select Lock"]
pub type NPslIn1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN2_AHI_LK` reader - PSL_IN2 Active-High Lock"]
pub type PslIn2AhiLkR = crate::BitReader;
#[doc = "Field `PSL_IN2_AHI_LK` writer - PSL_IN2 Active-High Lock"]
pub type PslIn2AhiLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nPSL_IN2_SL_LK` reader - PSL_IN2 Select Lock"]
pub type NPslIn2SlLkR = crate::BitReader;
#[doc = "Field `nPSL_IN2_SL_LK` writer - PSL_IN2 Select Lock"]
pub type NPslIn2SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_AHI_LK` reader - PSL_IN3 Active-High Lock"]
pub type PslIn3AhiLkR = crate::BitReader;
#[doc = "Field `PSL_IN3_AHI_LK` writer - PSL_IN3 Active-High Lock"]
pub type PslIn3AhiLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_SL_LK` reader - PSL_IN3 Select Lock"]
pub type PslIn3SlLkR = crate::BitReader;
#[doc = "Field `PSL_IN3_SL_LK` writer - PSL_IN3 Select Lock"]
pub type PslIn3SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_AHI_LK` reader - PSL_IN4 Active-High Lock"]
pub type PslIn4AhiLkR = crate::BitReader;
#[doc = "Field `PSL_IN4_AHI_LK` writer - PSL_IN4 Active-High Lock"]
pub type PslIn4AhiLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_SL_LK` reader - PSL_IN4 Select Lock"]
pub type PslIn4SlLkR = crate::BitReader;
#[doc = "Field `PSL_IN4_SL_LK` writer - PSL_IN4 Select Lock"]
pub type PslIn4SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PSL_IN1 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in1_ahi_lk(&self) -> PslIn1AhiLkR {
        PslIn1AhiLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSL_IN1 Select Lock"]
    #[inline(always)]
    pub fn n_psl_in1_sl_lk(&self) -> NPslIn1SlLkR {
        NPslIn1SlLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSL_IN2 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in2_ahi_lk(&self) -> PslIn2AhiLkR {
        PslIn2AhiLkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PSL_IN2 Select Lock"]
    #[inline(always)]
    pub fn n_psl_in2_sl_lk(&self) -> NPslIn2SlLkR {
        NPslIn2SlLkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSL_IN3 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in3_ahi_lk(&self) -> PslIn3AhiLkR {
        PslIn3AhiLkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSL_IN3 Select Lock"]
    #[inline(always)]
    pub fn psl_in3_sl_lk(&self) -> PslIn3SlLkR {
        PslIn3SlLkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSL_IN4 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in4_ahi_lk(&self) -> PslIn4AhiLkR {
        PslIn4AhiLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSL_IN4 Select Lock"]
    #[inline(always)]
    pub fn psl_in4_sl_lk(&self) -> PslIn4SlLkR {
        PslIn4SlLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTD_LK")
            .field("psl_in4_sl_lk", &self.psl_in4_sl_lk())
            .field("psl_in4_ahi_lk", &self.psl_in4_ahi_lk())
            .field("psl_in3_sl_lk", &self.psl_in3_sl_lk())
            .field("psl_in3_ahi_lk", &self.psl_in3_ahi_lk())
            .field("n_psl_in2_sl_lk", &self.n_psl_in2_sl_lk())
            .field("psl_in2_ahi_lk", &self.psl_in2_ahi_lk())
            .field("n_psl_in1_sl_lk", &self.n_psl_in1_sl_lk())
            .field("psl_in1_ahi_lk", &self.psl_in1_ahi_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PSL_IN1 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in1_ahi_lk(&mut self) -> PslIn1AhiLkW<DevaltdLkSpec> {
        PslIn1AhiLkW::new(self, 0)
    }
    #[doc = "Bit 1 - PSL_IN1 Select Lock"]
    #[inline(always)]
    pub fn n_psl_in1_sl_lk(&mut self) -> NPslIn1SlLkW<DevaltdLkSpec> {
        NPslIn1SlLkW::new(self, 1)
    }
    #[doc = "Bit 2 - PSL_IN2 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in2_ahi_lk(&mut self) -> PslIn2AhiLkW<DevaltdLkSpec> {
        PslIn2AhiLkW::new(self, 2)
    }
    #[doc = "Bit 3 - PSL_IN2 Select Lock"]
    #[inline(always)]
    pub fn n_psl_in2_sl_lk(&mut self) -> NPslIn2SlLkW<DevaltdLkSpec> {
        NPslIn2SlLkW::new(self, 3)
    }
    #[doc = "Bit 4 - PSL_IN3 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in3_ahi_lk(&mut self) -> PslIn3AhiLkW<DevaltdLkSpec> {
        PslIn3AhiLkW::new(self, 4)
    }
    #[doc = "Bit 5 - PSL_IN3 Select Lock"]
    #[inline(always)]
    pub fn psl_in3_sl_lk(&mut self) -> PslIn3SlLkW<DevaltdLkSpec> {
        PslIn3SlLkW::new(self, 5)
    }
    #[doc = "Bit 6 - PSL_IN4 Active-High Lock"]
    #[inline(always)]
    pub fn psl_in4_ahi_lk(&mut self) -> PslIn4AhiLkW<DevaltdLkSpec> {
        PslIn4AhiLkW::new(self, 6)
    }
    #[doc = "Bit 7 - PSL_IN4 Select Lock"]
    #[inline(always)]
    pub fn psl_in4_sl_lk(&mut self) -> PslIn4SlLkW<DevaltdLkSpec> {
        PslIn4SlLkW::new(self, 7)
    }
}
#[doc = "Device Alternate Function D Lock Register (DEVALTD_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltd_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltd_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltdLkSpec;
impl crate::RegisterSpec for DevaltdLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltd_lk::R`](R) reader structure"]
impl crate::Readable for DevaltdLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltd_lk::W`](W) writer structure"]
impl crate::Writable for DevaltdLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTD_LK to value 0"]
impl crate::Resettable for DevaltdLkSpec {
    const RESET_VALUE: u8 = 0;
}
