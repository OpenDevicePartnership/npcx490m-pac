#[doc = "Register `DEVALT4_LK` reader"]
pub type R = crate::R<Devalt4LkSpec>;
#[doc = "Register `DEVALT4_LK` writer"]
pub type W = crate::W<Devalt4LkSpec>;
#[doc = "Field `PWM1_SL_LK` reader - PWM1 Select Lock"]
pub type Pwm1SlLkR = crate::BitReader;
#[doc = "Field `PWM1_SL_LK` writer - PWM1 Select Lock"]
pub type Pwm1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_SL_LK` reader - PWM2_TACH2PWM_OUT Select Lock"]
pub type Pwm2SlLkR = crate::BitReader;
#[doc = "Field `PWM2_SL_LK` writer - PWM2_TACH2PWM_OUT Select Lock"]
pub type Pwm2SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_SL_LK` reader - PWM3_LED3 Select Lock"]
pub type Pwm3SlLkR = crate::BitReader;
#[doc = "Field `PWM3_SL_LK` writer - PWM3_LED3 Select Lock"]
pub type Pwm3SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PWM1 Select Lock"]
    #[inline(always)]
    pub fn pwm1_sl_lk(&self) -> Pwm1SlLkR {
        Pwm1SlLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2_TACH2PWM_OUT Select Lock"]
    #[inline(always)]
    pub fn pwm2_sl_lk(&self) -> Pwm2SlLkR {
        Pwm2SlLkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3_LED3 Select Lock"]
    #[inline(always)]
    pub fn pwm3_sl_lk(&self) -> Pwm3SlLkR {
        Pwm3SlLkR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT4_LK")
            .field("pwm1_sl_lk", &self.pwm1_sl_lk())
            .field("pwm2_sl_lk", &self.pwm2_sl_lk())
            .field("pwm3_sl_lk", &self.pwm3_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - PWM1 Select Lock"]
    #[inline(always)]
    pub fn pwm1_sl_lk(&mut self) -> Pwm1SlLkW<Devalt4LkSpec> {
        Pwm1SlLkW::new(self, 1)
    }
    #[doc = "Bit 2 - PWM2_TACH2PWM_OUT Select Lock"]
    #[inline(always)]
    pub fn pwm2_sl_lk(&mut self) -> Pwm2SlLkW<Devalt4LkSpec> {
        Pwm2SlLkW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM3_LED3 Select Lock"]
    #[inline(always)]
    pub fn pwm3_sl_lk(&mut self) -> Pwm3SlLkW<Devalt4LkSpec> {
        Pwm3SlLkW::new(self, 3)
    }
}
#[doc = "Device Alternate Function 4 Lock Register (DEVALT4_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt4_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt4_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt4LkSpec;
impl crate::RegisterSpec for Devalt4LkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt4_lk::R`](R) reader structure"]
impl crate::Readable for Devalt4LkSpec {}
#[doc = "`write(|w| ..)` method takes [`devalt4_lk::W`](W) writer structure"]
impl crate::Writable for Devalt4LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT4_LK to value 0"]
impl crate::Resettable for Devalt4LkSpec {
    const RESET_VALUE: u8 = 0;
}
