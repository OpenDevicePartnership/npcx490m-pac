#[doc = "Register `DEVALT4` reader"]
pub type R = crate::R<Devalt4Spec>;
#[doc = "Register `DEVALT4` writer"]
pub type W = crate::W<Devalt4Spec>;
#[doc = "Field `PWM0_SL` reader - PWM0 Select"]
pub type Pwm0SlR = crate::BitReader;
#[doc = "Field `PWM0_SL` writer - PWM0 Select"]
pub type Pwm0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_SL` reader - PWM1 Select"]
pub type Pwm1SlR = crate::BitReader;
#[doc = "Field `PWM1_SL` writer - PWM1 Select"]
pub type Pwm1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_SL` reader - PWM2_TACH2PWM_OUT Select"]
pub type Pwm2SlR = crate::BitReader;
#[doc = "Field `PWM2_SL` writer - PWM2_TACH2PWM_OUT Select"]
pub type Pwm2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_SL` reader - PWM3_LED3 Select"]
pub type Pwm3SlR = crate::BitReader;
#[doc = "Field `PWM3_SL` writer - PWM3_LED3 Select"]
pub type Pwm3SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4_SL` reader - PWM4_LED0 Select"]
pub type Pwm4SlR = crate::BitReader;
#[doc = "Field `PWM4_SL` writer - PWM4_LED0 Select"]
pub type Pwm4SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM5_SL` reader - PWM5_LED1 Select"]
pub type Pwm5SlR = crate::BitReader;
#[doc = "Field `PWM5_SL` writer - PWM5_LED1 Select"]
pub type Pwm5SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM6_SL` reader - PWM6_LED2 Select"]
pub type Pwm6SlR = crate::BitReader;
#[doc = "Field `PWM6_SL` writer - PWM6_LED2 Select"]
pub type Pwm6SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM7_SL` reader - PWM7 Select"]
pub type Pwm7SlR = crate::BitReader;
#[doc = "Field `PWM7_SL` writer - PWM7 Select"]
pub type Pwm7SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM0 Select"]
    #[inline(always)]
    pub fn pwm0_sl(&self) -> Pwm0SlR {
        Pwm0SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Select"]
    #[inline(always)]
    pub fn pwm1_sl(&self) -> Pwm1SlR {
        Pwm1SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2_TACH2PWM_OUT Select"]
    #[inline(always)]
    pub fn pwm2_sl(&self) -> Pwm2SlR {
        Pwm2SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3_LED3 Select"]
    #[inline(always)]
    pub fn pwm3_sl(&self) -> Pwm3SlR {
        Pwm3SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4_LED0 Select"]
    #[inline(always)]
    pub fn pwm4_sl(&self) -> Pwm4SlR {
        Pwm4SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM5_LED1 Select"]
    #[inline(always)]
    pub fn pwm5_sl(&self) -> Pwm5SlR {
        Pwm5SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6_LED2 Select"]
    #[inline(always)]
    pub fn pwm6_sl(&self) -> Pwm6SlR {
        Pwm6SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM7 Select"]
    #[inline(always)]
    pub fn pwm7_sl(&self) -> Pwm7SlR {
        Pwm7SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT4")
            .field("pwm0_sl", &self.pwm0_sl())
            .field("pwm1_sl", &self.pwm1_sl())
            .field("pwm2_sl", &self.pwm2_sl())
            .field("pwm3_sl", &self.pwm3_sl())
            .field("pwm4_sl", &self.pwm4_sl())
            .field("pwm5_sl", &self.pwm5_sl())
            .field("pwm6_sl", &self.pwm6_sl())
            .field("pwm7_sl", &self.pwm7_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Select"]
    #[inline(always)]
    pub fn pwm0_sl(&mut self) -> Pwm0SlW<Devalt4Spec> {
        Pwm0SlW::new(self, 0)
    }
    #[doc = "Bit 1 - PWM1 Select"]
    #[inline(always)]
    pub fn pwm1_sl(&mut self) -> Pwm1SlW<Devalt4Spec> {
        Pwm1SlW::new(self, 1)
    }
    #[doc = "Bit 2 - PWM2_TACH2PWM_OUT Select"]
    #[inline(always)]
    pub fn pwm2_sl(&mut self) -> Pwm2SlW<Devalt4Spec> {
        Pwm2SlW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM3_LED3 Select"]
    #[inline(always)]
    pub fn pwm3_sl(&mut self) -> Pwm3SlW<Devalt4Spec> {
        Pwm3SlW::new(self, 3)
    }
    #[doc = "Bit 4 - PWM4_LED0 Select"]
    #[inline(always)]
    pub fn pwm4_sl(&mut self) -> Pwm4SlW<Devalt4Spec> {
        Pwm4SlW::new(self, 4)
    }
    #[doc = "Bit 5 - PWM5_LED1 Select"]
    #[inline(always)]
    pub fn pwm5_sl(&mut self) -> Pwm5SlW<Devalt4Spec> {
        Pwm5SlW::new(self, 5)
    }
    #[doc = "Bit 6 - PWM6_LED2 Select"]
    #[inline(always)]
    pub fn pwm6_sl(&mut self) -> Pwm6SlW<Devalt4Spec> {
        Pwm6SlW::new(self, 6)
    }
    #[doc = "Bit 7 - PWM7 Select"]
    #[inline(always)]
    pub fn pwm7_sl(&mut self) -> Pwm7SlW<Devalt4Spec> {
        Pwm7SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 4 Register (DEVALT4)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt4Spec;
impl crate::RegisterSpec for Devalt4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt4::R`](R) reader structure"]
impl crate::Readable for Devalt4Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt4::W`](W) writer structure"]
impl crate::Writable for Devalt4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT4 to value 0"]
impl crate::Resettable for Devalt4Spec {
    const RESET_VALUE: u8 = 0;
}
