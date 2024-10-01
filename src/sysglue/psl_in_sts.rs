#[doc = "Register `PSL_IN_STS` reader"]
pub type R = crate::R<PslInStsSpec>;
#[doc = "Register `PSL_IN_STS` writer"]
pub type W = crate::W<PslInStsSpec>;
#[doc = "Field `PSL_IN1_STS` reader - PSL_IN1 Status"]
pub type PslIn1StsR = crate::BitReader;
#[doc = "Field `PSL_IN1_STS` writer - PSL_IN1 Status"]
pub type PslIn1StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN2_STS` reader - PSL_IN2 Status"]
pub type PslIn2StsR = crate::BitReader;
#[doc = "Field `PSL_IN2_STS` writer - PSL_IN2 Status"]
pub type PslIn2StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_STS` reader - PSL_IN3 Status"]
pub type PslIn3StsR = crate::BitReader;
#[doc = "Field `PSL_IN3_STS` writer - PSL_IN3 Status"]
pub type PslIn3StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_STS` reader - PSL_IN4 Status"]
pub type PslIn4StsR = crate::BitReader;
#[doc = "Field `PSL_IN4_STS` writer - PSL_IN4 Status"]
pub type PslIn4StsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PSL_IN1 Status"]
    #[inline(always)]
    pub fn psl_in1_sts(&self) -> PslIn1StsR {
        PslIn1StsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSL_IN2 Status"]
    #[inline(always)]
    pub fn psl_in2_sts(&self) -> PslIn2StsR {
        PslIn2StsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSL_IN3 Status"]
    #[inline(always)]
    pub fn psl_in3_sts(&self) -> PslIn3StsR {
        PslIn3StsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PSL_IN4 Status"]
    #[inline(always)]
    pub fn psl_in4_sts(&self) -> PslIn4StsR {
        PslIn4StsR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSL_IN_STS")
            .field("psl_in1_sts", &self.psl_in1_sts())
            .field("psl_in2_sts", &self.psl_in2_sts())
            .field("psl_in3_sts", &self.psl_in3_sts())
            .field("psl_in4_sts", &self.psl_in4_sts())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PSL_IN1 Status"]
    #[inline(always)]
    #[must_use]
    pub fn psl_in1_sts(&mut self) -> PslIn1StsW<PslInStsSpec> {
        PslIn1StsW::new(self, 0)
    }
    #[doc = "Bit 1 - PSL_IN2 Status"]
    #[inline(always)]
    #[must_use]
    pub fn psl_in2_sts(&mut self) -> PslIn2StsW<PslInStsSpec> {
        PslIn2StsW::new(self, 1)
    }
    #[doc = "Bit 2 - PSL_IN3 Status"]
    #[inline(always)]
    #[must_use]
    pub fn psl_in3_sts(&mut self) -> PslIn3StsW<PslInStsSpec> {
        PslIn3StsW::new(self, 2)
    }
    #[doc = "Bit 3 - PSL_IN4 Status"]
    #[inline(always)]
    #[must_use]
    pub fn psl_in4_sts(&mut self) -> PslIn4StsW<PslInStsSpec> {
        PslIn4StsW::new(self, 3)
    }
}
#[doc = "PSL Input Status Register (PSL_IN_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_in_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_in_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PslInStsSpec;
impl crate::RegisterSpec for PslInStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psl_in_sts::R`](R) reader structure"]
impl crate::Readable for PslInStsSpec {}
#[doc = "`write(|w| ..)` method takes [`psl_in_sts::W`](W) writer structure"]
impl crate::Writable for PslInStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSL_IN_STS to value 0"]
impl crate::Resettable for PslInStsSpec {
    const RESET_VALUE: u8 = 0;
}
