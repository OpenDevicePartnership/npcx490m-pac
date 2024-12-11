#[doc = "Register `PSL_CTS` reader"]
pub type R = crate::R<PslCtsSpec>;
#[doc = "Register `PSL_CTS` writer"]
pub type W = crate::W<PslCtsSpec>;
#[doc = "Field `PSL_IN1_EV` reader - PSL_IN1 Event"]
pub type PslIn1EvR = crate::BitReader;
#[doc = "Field `PSL_IN1_EV` writer - PSL_IN1 Event"]
pub type PslIn1EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN2_EV` reader - PSL_IN2 Event"]
pub type PslIn2EvR = crate::BitReader;
#[doc = "Field `PSL_IN2_EV` writer - PSL_IN2 Event"]
pub type PslIn2EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_EV` reader - PSL_IN3 Event"]
pub type PslIn3EvR = crate::BitReader;
#[doc = "Field `PSL_IN3_EV` writer - PSL_IN3 Event"]
pub type PslIn3EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_EV` reader - PSL_IN4 Event"]
pub type PslIn4EvR = crate::BitReader;
#[doc = "Field `PSL_IN4_EV` writer - PSL_IN4 Event"]
pub type PslIn4EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN1_EDG` reader - PSL_IN1 Edge Mode"]
pub type PslIn1EdgR = crate::BitReader;
#[doc = "Field `PSL_IN1_EDG` writer - PSL_IN1 Edge Mode"]
pub type PslIn1EdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN2_EDG` reader - PSL_IN2 Edge Mode"]
pub type PslIn2EdgR = crate::BitReader;
#[doc = "Field `PSL_IN2_EDG` writer - PSL_IN2 Edge Mode"]
pub type PslIn2EdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_EDG` reader - PSL_IN3 Edge Mode"]
pub type PslIn3EdgR = crate::BitReader;
#[doc = "Field `PSL_IN3_EDG` writer - PSL_IN3 Edge Mode"]
pub type PslIn3EdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_EDG` reader - PSL_IN4 Edge Mode"]
pub type PslIn4EdgR = crate::BitReader;
#[doc = "Field `PSL_IN4_EDG` writer - PSL_IN4 Edge Mode"]
pub type PslIn4EdgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PSL_IN1 Event"]
    #[inline(always)]
    pub fn psl_in1_ev(&self) -> PslIn1EvR {
        PslIn1EvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSL_IN2 Event"]
    #[inline(always)]
    pub fn psl_in2_ev(&self) -> PslIn2EvR {
        PslIn2EvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSL_IN3 Event"]
    #[inline(always)]
    pub fn psl_in3_ev(&self) -> PslIn3EvR {
        PslIn3EvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PSL_IN4 Event"]
    #[inline(always)]
    pub fn psl_in4_ev(&self) -> PslIn4EvR {
        PslIn4EvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSL_IN1 Edge Mode"]
    #[inline(always)]
    pub fn psl_in1_edg(&self) -> PslIn1EdgR {
        PslIn1EdgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSL_IN2 Edge Mode"]
    #[inline(always)]
    pub fn psl_in2_edg(&self) -> PslIn2EdgR {
        PslIn2EdgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSL_IN3 Edge Mode"]
    #[inline(always)]
    pub fn psl_in3_edg(&self) -> PslIn3EdgR {
        PslIn3EdgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSL_IN4 Edge Mode"]
    #[inline(always)]
    pub fn psl_in4_edg(&self) -> PslIn4EdgR {
        PslIn4EdgR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSL_CTS")
            .field("psl_in1_ev", &self.psl_in1_ev())
            .field("psl_in2_ev", &self.psl_in2_ev())
            .field("psl_in3_ev", &self.psl_in3_ev())
            .field("psl_in4_ev", &self.psl_in4_ev())
            .field("psl_in1_edg", &self.psl_in1_edg())
            .field("psl_in2_edg", &self.psl_in2_edg())
            .field("psl_in3_edg", &self.psl_in3_edg())
            .field("psl_in4_edg", &self.psl_in4_edg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PSL_IN1 Event"]
    #[inline(always)]
    pub fn psl_in1_ev(&mut self) -> PslIn1EvW<PslCtsSpec> {
        PslIn1EvW::new(self, 0)
    }
    #[doc = "Bit 1 - PSL_IN2 Event"]
    #[inline(always)]
    pub fn psl_in2_ev(&mut self) -> PslIn2EvW<PslCtsSpec> {
        PslIn2EvW::new(self, 1)
    }
    #[doc = "Bit 2 - PSL_IN3 Event"]
    #[inline(always)]
    pub fn psl_in3_ev(&mut self) -> PslIn3EvW<PslCtsSpec> {
        PslIn3EvW::new(self, 2)
    }
    #[doc = "Bit 3 - PSL_IN4 Event"]
    #[inline(always)]
    pub fn psl_in4_ev(&mut self) -> PslIn4EvW<PslCtsSpec> {
        PslIn4EvW::new(self, 3)
    }
    #[doc = "Bit 4 - PSL_IN1 Edge Mode"]
    #[inline(always)]
    pub fn psl_in1_edg(&mut self) -> PslIn1EdgW<PslCtsSpec> {
        PslIn1EdgW::new(self, 4)
    }
    #[doc = "Bit 5 - PSL_IN2 Edge Mode"]
    #[inline(always)]
    pub fn psl_in2_edg(&mut self) -> PslIn2EdgW<PslCtsSpec> {
        PslIn2EdgW::new(self, 5)
    }
    #[doc = "Bit 6 - PSL_IN3 Edge Mode"]
    #[inline(always)]
    pub fn psl_in3_edg(&mut self) -> PslIn3EdgW<PslCtsSpec> {
        PslIn3EdgW::new(self, 6)
    }
    #[doc = "Bit 7 - PSL_IN4 Edge Mode"]
    #[inline(always)]
    pub fn psl_in4_edg(&mut self) -> PslIn4EdgW<PslCtsSpec> {
        PslIn4EdgW::new(self, 7)
    }
}
#[doc = "PSL Control and Status Register (PSL_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_cts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_cts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PslCtsSpec;
impl crate::RegisterSpec for PslCtsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psl_cts::R`](R) reader structure"]
impl crate::Readable for PslCtsSpec {}
#[doc = "`write(|w| ..)` method takes [`psl_cts::W`](W) writer structure"]
impl crate::Writable for PslCtsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSL_CTS to value 0"]
impl crate::Resettable for PslCtsSpec {
    const RESET_VALUE: u8 = 0;
}
