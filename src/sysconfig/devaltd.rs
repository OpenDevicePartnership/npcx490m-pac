#[doc = "Register `DEVALTD` reader"]
pub type R = crate::R<DevaltdSpec>;
#[doc = "Register `DEVALTD` writer"]
pub type W = crate::W<DevaltdSpec>;
#[doc = "Field `PSL_IN1_AHI` reader - PSL_IN1 Active-High"]
pub type PslIn1AhiR = crate::BitReader;
#[doc = "Field `PSL_IN1_AHI` writer - PSL_IN1 Active-High"]
pub type PslIn1AhiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nPSL_IN1_SL` reader - PSL_IN1 Select"]
pub type NPslIn1SlR = crate::BitReader;
#[doc = "Field `nPSL_IN1_SL` writer - PSL_IN1 Select"]
pub type NPslIn1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN2_AHI` reader - PSL_IN2 Active-High"]
pub type PslIn2AhiR = crate::BitReader;
#[doc = "Field `PSL_IN2_AHI` writer - PSL_IN2 Active-High"]
pub type PslIn2AhiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nPSL_IN2_SL` reader - PSL_IN2 Select"]
pub type NPslIn2SlR = crate::BitReader;
#[doc = "Field `nPSL_IN2_SL` writer - PSL_IN2 Select"]
pub type NPslIn2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_AHI` reader - PSL_IN3 Active-High"]
pub type PslIn3AhiR = crate::BitReader;
#[doc = "Field `PSL_IN3_AHI` writer - PSL_IN3 Active-High"]
pub type PslIn3AhiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_SL` reader - PSL_IN3 Select"]
pub type PslIn3SlR = crate::BitReader;
#[doc = "Field `PSL_IN3_SL` writer - PSL_IN3 Select"]
pub type PslIn3SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_AHI` reader - PSL4_IN4 Active-High"]
pub type PslIn4AhiR = crate::BitReader;
#[doc = "Field `PSL_IN4_AHI` writer - PSL4_IN4 Active-High"]
pub type PslIn4AhiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_SL` reader - PSL4_IN4 Select"]
pub type PslIn4SlR = crate::BitReader;
#[doc = "Field `PSL_IN4_SL` writer - PSL4_IN4 Select"]
pub type PslIn4SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PSL_IN1 Active-High"]
    #[inline(always)]
    pub fn psl_in1_ahi(&self) -> PslIn1AhiR {
        PslIn1AhiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSL_IN1 Select"]
    #[inline(always)]
    pub fn n_psl_in1_sl(&self) -> NPslIn1SlR {
        NPslIn1SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSL_IN2 Active-High"]
    #[inline(always)]
    pub fn psl_in2_ahi(&self) -> PslIn2AhiR {
        PslIn2AhiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PSL_IN2 Select"]
    #[inline(always)]
    pub fn n_psl_in2_sl(&self) -> NPslIn2SlR {
        NPslIn2SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSL_IN3 Active-High"]
    #[inline(always)]
    pub fn psl_in3_ahi(&self) -> PslIn3AhiR {
        PslIn3AhiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSL_IN3 Select"]
    #[inline(always)]
    pub fn psl_in3_sl(&self) -> PslIn3SlR {
        PslIn3SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSL4_IN4 Active-High"]
    #[inline(always)]
    pub fn psl_in4_ahi(&self) -> PslIn4AhiR {
        PslIn4AhiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSL4_IN4 Select"]
    #[inline(always)]
    pub fn psl_in4_sl(&self) -> PslIn4SlR {
        PslIn4SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTD")
            .field("psl_in4_sl", &self.psl_in4_sl())
            .field("psl_in4_ahi", &self.psl_in4_ahi())
            .field("psl_in3_sl", &self.psl_in3_sl())
            .field("psl_in3_ahi", &self.psl_in3_ahi())
            .field("n_psl_in2_sl", &self.n_psl_in2_sl())
            .field("psl_in2_ahi", &self.psl_in2_ahi())
            .field("n_psl_in1_sl", &self.n_psl_in1_sl())
            .field("psl_in1_ahi", &self.psl_in1_ahi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PSL_IN1 Active-High"]
    #[inline(always)]
    pub fn psl_in1_ahi(&mut self) -> PslIn1AhiW<DevaltdSpec> {
        PslIn1AhiW::new(self, 0)
    }
    #[doc = "Bit 1 - PSL_IN1 Select"]
    #[inline(always)]
    pub fn n_psl_in1_sl(&mut self) -> NPslIn1SlW<DevaltdSpec> {
        NPslIn1SlW::new(self, 1)
    }
    #[doc = "Bit 2 - PSL_IN2 Active-High"]
    #[inline(always)]
    pub fn psl_in2_ahi(&mut self) -> PslIn2AhiW<DevaltdSpec> {
        PslIn2AhiW::new(self, 2)
    }
    #[doc = "Bit 3 - PSL_IN2 Select"]
    #[inline(always)]
    pub fn n_psl_in2_sl(&mut self) -> NPslIn2SlW<DevaltdSpec> {
        NPslIn2SlW::new(self, 3)
    }
    #[doc = "Bit 4 - PSL_IN3 Active-High"]
    #[inline(always)]
    pub fn psl_in3_ahi(&mut self) -> PslIn3AhiW<DevaltdSpec> {
        PslIn3AhiW::new(self, 4)
    }
    #[doc = "Bit 5 - PSL_IN3 Select"]
    #[inline(always)]
    pub fn psl_in3_sl(&mut self) -> PslIn3SlW<DevaltdSpec> {
        PslIn3SlW::new(self, 5)
    }
    #[doc = "Bit 6 - PSL4_IN4 Active-High"]
    #[inline(always)]
    pub fn psl_in4_ahi(&mut self) -> PslIn4AhiW<DevaltdSpec> {
        PslIn4AhiW::new(self, 6)
    }
    #[doc = "Bit 7 - PSL4_IN4 Select"]
    #[inline(always)]
    pub fn psl_in4_sl(&mut self) -> PslIn4SlW<DevaltdSpec> {
        PslIn4SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function D Register (DEVALTD)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltdSpec;
impl crate::RegisterSpec for DevaltdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltd::R`](R) reader structure"]
impl crate::Readable for DevaltdSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltd::W`](W) writer structure"]
impl crate::Writable for DevaltdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTD to value 0"]
impl crate::Resettable for DevaltdSpec {
    const RESET_VALUE: u8 = 0;
}
