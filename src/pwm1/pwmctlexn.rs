#[doc = "Register `PWMCTLEXn` reader"]
pub type R = crate::R<PwmctlexnSpec>;
#[doc = "Register `PWMCTLEXn` writer"]
pub type W = crate::W<PwmctlexnSpec>;
#[doc = "Field `ONE_SHOT_TRG` reader - One-Shot Trigger"]
pub type OneShotTrgR = crate::BitReader;
#[doc = "Field `ONE_SHOT_TRG` writer - One-Shot Trigger"]
pub type OneShotTrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONE_SHOT` reader - One-Shot Mode"]
pub type OneShotR = crate::FieldReader;
#[doc = "Field `ONE_SHOT` writer - One-Shot Mode"]
pub type OneShotW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FCK_SEL` reader - Fixed Frequency Clock Select"]
pub type FckSelR = crate::FieldReader;
#[doc = "Field `FCK_SEL` writer - Fixed Frequency Clock Select"]
pub type FckSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OD_OUT` reader - Open-Drain Output"]
pub type OdOutR = crate::BitReader;
#[doc = "Field `OD_OUT` writer - Open-Drain Output"]
pub type OdOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - One-Shot Trigger"]
    #[inline(always)]
    pub fn one_shot_trg(&self) -> OneShotTrgR {
        OneShotTrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - One-Shot Mode"]
    #[inline(always)]
    pub fn one_shot(&self) -> OneShotR {
        OneShotR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Fixed Frequency Clock Select"]
    #[inline(always)]
    pub fn fck_sel(&self) -> FckSelR {
        FckSelR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Open-Drain Output"]
    #[inline(always)]
    pub fn od_out(&self) -> OdOutR {
        OdOutR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMCTLEXn")
            .field("one_shot_trg", &self.one_shot_trg())
            .field("one_shot", &self.one_shot())
            .field("fck_sel", &self.fck_sel())
            .field("od_out", &self.od_out())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - One-Shot Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn one_shot_trg(&mut self) -> OneShotTrgW<PwmctlexnSpec> {
        OneShotTrgW::new(self, 1)
    }
    #[doc = "Bits 2:3 - One-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn one_shot(&mut self) -> OneShotW<PwmctlexnSpec> {
        OneShotW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Fixed Frequency Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fck_sel(&mut self) -> FckSelW<PwmctlexnSpec> {
        FckSelW::new(self, 4)
    }
    #[doc = "Bit 7 - Open-Drain Output"]
    #[inline(always)]
    #[must_use]
    pub fn od_out(&mut self) -> OdOutW<PwmctlexnSpec> {
        OdOutW::new(self, 7)
    }
}
#[doc = "PWM Control Extended Register (PWMCTLEXn)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmctlexn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmctlexn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmctlexnSpec;
impl crate::RegisterSpec for PwmctlexnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwmctlexn::R`](R) reader structure"]
impl crate::Readable for PwmctlexnSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmctlexn::W`](W) writer structure"]
impl crate::Writable for PwmctlexnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWMCTLEXn to value 0"]
impl crate::Resettable for PwmctlexnSpec {
    const RESET_VALUE: u8 = 0;
}
