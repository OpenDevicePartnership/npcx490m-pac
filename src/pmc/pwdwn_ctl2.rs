#[doc = "Register `PWDWN_CTL2` reader"]
pub type R = crate::R<PwdwnCtl2Spec>;
#[doc = "Register `PWDWN_CTL2` writer"]
pub type W = crate::W<PwdwnCtl2Spec>;
#[doc = "Field `PWM0_PD` reader - PWM0 Power-Down"]
pub type Pwm0PdR = crate::BitReader;
#[doc = "Field `PWM0_PD` writer - PWM0 Power-Down"]
pub type Pwm0PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_PD` reader - PWM1 Power-Down"]
pub type Pwm1PdR = crate::BitReader;
#[doc = "Field `PWM1_PD` writer - PWM1 Power-Down"]
pub type Pwm1PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_PD` reader - PWM2 Power-Down"]
pub type Pwm2PdR = crate::BitReader;
#[doc = "Field `PWM2_PD` writer - PWM2 Power-Down"]
pub type Pwm2PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_PD` reader - PWM3 Power-Down"]
pub type Pwm3PdR = crate::BitReader;
#[doc = "Field `PWM3_PD` writer - PWM3 Power-Down"]
pub type Pwm3PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4_PD` reader - PWM4 Power-Down"]
pub type Pwm4PdR = crate::BitReader;
#[doc = "Field `PWM4_PD` writer - PWM4 Power-Down"]
pub type Pwm4PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM5_PD` reader - PWM5 Power-Down"]
pub type Pwm5PdR = crate::BitReader;
#[doc = "Field `PWM5_PD` writer - PWM5 Power-Down"]
pub type Pwm5PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM6_PD` reader - PWM6 Power-Down"]
pub type Pwm6PdR = crate::BitReader;
#[doc = "Field `PWM6_PD` writer - PWM6 Power-Down"]
pub type Pwm6PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM7_PD` reader - PWM7 Power-Down"]
pub type Pwm7PdR = crate::BitReader;
#[doc = "Field `PWM7_PD` writer - PWM7 Power-Down"]
pub type Pwm7PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM0 Power-Down"]
    #[inline(always)]
    pub fn pwm0_pd(&self) -> Pwm0PdR {
        Pwm0PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Power-Down"]
    #[inline(always)]
    pub fn pwm1_pd(&self) -> Pwm1PdR {
        Pwm1PdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Power-Down"]
    #[inline(always)]
    pub fn pwm2_pd(&self) -> Pwm2PdR {
        Pwm2PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Power-Down"]
    #[inline(always)]
    pub fn pwm3_pd(&self) -> Pwm3PdR {
        Pwm3PdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4 Power-Down"]
    #[inline(always)]
    pub fn pwm4_pd(&self) -> Pwm4PdR {
        Pwm4PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM5 Power-Down"]
    #[inline(always)]
    pub fn pwm5_pd(&self) -> Pwm5PdR {
        Pwm5PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Power-Down"]
    #[inline(always)]
    pub fn pwm6_pd(&self) -> Pwm6PdR {
        Pwm6PdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM7 Power-Down"]
    #[inline(always)]
    pub fn pwm7_pd(&self) -> Pwm7PdR {
        Pwm7PdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL2")
            .field("pwm0_pd", &self.pwm0_pd())
            .field("pwm1_pd", &self.pwm1_pd())
            .field("pwm2_pd", &self.pwm2_pd())
            .field("pwm3_pd", &self.pwm3_pd())
            .field("pwm4_pd", &self.pwm4_pd())
            .field("pwm5_pd", &self.pwm5_pd())
            .field("pwm6_pd", &self.pwm6_pd())
            .field("pwm7_pd", &self.pwm7_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Power-Down"]
    #[inline(always)]
    pub fn pwm0_pd(&mut self) -> Pwm0PdW<PwdwnCtl2Spec> {
        Pwm0PdW::new(self, 0)
    }
    #[doc = "Bit 1 - PWM1 Power-Down"]
    #[inline(always)]
    pub fn pwm1_pd(&mut self) -> Pwm1PdW<PwdwnCtl2Spec> {
        Pwm1PdW::new(self, 1)
    }
    #[doc = "Bit 2 - PWM2 Power-Down"]
    #[inline(always)]
    pub fn pwm2_pd(&mut self) -> Pwm2PdW<PwdwnCtl2Spec> {
        Pwm2PdW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM3 Power-Down"]
    #[inline(always)]
    pub fn pwm3_pd(&mut self) -> Pwm3PdW<PwdwnCtl2Spec> {
        Pwm3PdW::new(self, 3)
    }
    #[doc = "Bit 4 - PWM4 Power-Down"]
    #[inline(always)]
    pub fn pwm4_pd(&mut self) -> Pwm4PdW<PwdwnCtl2Spec> {
        Pwm4PdW::new(self, 4)
    }
    #[doc = "Bit 5 - PWM5 Power-Down"]
    #[inline(always)]
    pub fn pwm5_pd(&mut self) -> Pwm5PdW<PwdwnCtl2Spec> {
        Pwm5PdW::new(self, 5)
    }
    #[doc = "Bit 6 - PWM6 Power-Down"]
    #[inline(always)]
    pub fn pwm6_pd(&mut self) -> Pwm6PdW<PwdwnCtl2Spec> {
        Pwm6PdW::new(self, 6)
    }
    #[doc = "Bit 7 - PWM7 Power-Down"]
    #[inline(always)]
    pub fn pwm7_pd(&mut self) -> Pwm7PdW<PwdwnCtl2Spec> {
        Pwm7PdW::new(self, 7)
    }
}
#[doc = "Power-Down Control 2 Register (PWDWN_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl2Spec;
impl crate::RegisterSpec for PwdwnCtl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl2::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl2::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL2 to value 0"]
impl crate::Resettable for PwdwnCtl2Spec {
    const RESET_VALUE: u8 = 0;
}
