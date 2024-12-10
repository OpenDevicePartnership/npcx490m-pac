#[doc = "Register `DEVALT3` reader"]
pub type R = crate::R<Devalt3Spec>;
#[doc = "Register `DEVALT3` writer"]
pub type W = crate::W<Devalt3Spec>;
#[doc = "Field `PS2_0_SL` reader - PS/2 #0 Select"]
pub type Ps2_0SlR = crate::BitReader;
#[doc = "Field `PS2_0_SL` writer - PS/2 #0 Select"]
pub type Ps2_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_1_SL` reader - PS/2 #1 Select"]
pub type Ps2_1SlR = crate::BitReader;
#[doc = "Field `PS2_1_SL` writer - PS/2 #1 Select"]
pub type Ps2_1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_2_SL` reader - PS/2 #2 Select"]
pub type Ps2_2SlR = crate::BitReader;
#[doc = "Field `PS2_2_SL` writer - PS/2 #2 Select"]
pub type Ps2_2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_3_SL1` reader - PS/2 #3 Select-1"]
pub type Ps2_3Sl1R = crate::BitReader;
#[doc = "Field `PS2_3_SL1` writer - PS/2 #3 Select-1"]
pub type Ps2_3Sl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA1_TACH1_SL1` reader - TA1_TACH1 Select-1"]
pub type Ta1Tach1Sl1R = crate::BitReader;
#[doc = "Field `TA1_TACH1_SL1` writer - TA1_TACH1 Select-1"]
pub type Ta1Tach1Sl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1_TACH2_SL1` reader - TB1_TACH2PWM_IN Select-1"]
pub type Tb1Tach2Sl1R = crate::BitReader;
#[doc = "Field `TB1_TACH2_SL1` writer - TB1_TACH2PWM_IN Select-1"]
pub type Tb1Tach2Sl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2_SL1` reader - TA2 Select-1"]
pub type Ta2Sl1R = crate::BitReader;
#[doc = "Field `TA2_SL1` writer - TA2 Select-1"]
pub type Ta2Sl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2_SL1` reader - TB2 Select-1"]
pub type Tb2Sl1R = crate::BitReader;
#[doc = "Field `TB2_SL1` writer - TB2 Select-1"]
pub type Tb2Sl1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PS/2 #0 Select"]
    #[inline(always)]
    pub fn ps2_0_sl(&self) -> Ps2_0SlR {
        Ps2_0SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PS/2 #1 Select"]
    #[inline(always)]
    pub fn ps2_1_sl(&self) -> Ps2_1SlR {
        Ps2_1SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PS/2 #2 Select"]
    #[inline(always)]
    pub fn ps2_2_sl(&self) -> Ps2_2SlR {
        Ps2_2SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PS/2 #3 Select-1"]
    #[inline(always)]
    pub fn ps2_3_sl1(&self) -> Ps2_3Sl1R {
        Ps2_3Sl1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-1"]
    #[inline(always)]
    pub fn ta1_tach1_sl1(&self) -> Ta1Tach1Sl1R {
        Ta1Tach1Sl1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TB1_TACH2PWM_IN Select-1"]
    #[inline(always)]
    pub fn tb1_tach2_sl1(&self) -> Tb1Tach2Sl1R {
        Tb1Tach2Sl1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TA2 Select-1"]
    #[inline(always)]
    pub fn ta2_sl1(&self) -> Ta2Sl1R {
        Ta2Sl1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TB2 Select-1"]
    #[inline(always)]
    pub fn tb2_sl1(&self) -> Tb2Sl1R {
        Tb2Sl1R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT3")
            .field("ps2_0_sl", &self.ps2_0_sl())
            .field("ps2_1_sl", &self.ps2_1_sl())
            .field("ps2_2_sl", &self.ps2_2_sl())
            .field("ps2_3_sl1", &self.ps2_3_sl1())
            .field("ta1_tach1_sl1", &self.ta1_tach1_sl1())
            .field("tb1_tach2_sl1", &self.tb1_tach2_sl1())
            .field("ta2_sl1", &self.ta2_sl1())
            .field("tb2_sl1", &self.tb2_sl1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PS/2 #0 Select"]
    #[inline(always)]
    pub fn ps2_0_sl(&mut self) -> Ps2_0SlW<Devalt3Spec> {
        Ps2_0SlW::new(self, 0)
    }
    #[doc = "Bit 1 - PS/2 #1 Select"]
    #[inline(always)]
    pub fn ps2_1_sl(&mut self) -> Ps2_1SlW<Devalt3Spec> {
        Ps2_1SlW::new(self, 1)
    }
    #[doc = "Bit 2 - PS/2 #2 Select"]
    #[inline(always)]
    pub fn ps2_2_sl(&mut self) -> Ps2_2SlW<Devalt3Spec> {
        Ps2_2SlW::new(self, 2)
    }
    #[doc = "Bit 3 - PS/2 #3 Select-1"]
    #[inline(always)]
    pub fn ps2_3_sl1(&mut self) -> Ps2_3Sl1W<Devalt3Spec> {
        Ps2_3Sl1W::new(self, 3)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-1"]
    #[inline(always)]
    pub fn ta1_tach1_sl1(&mut self) -> Ta1Tach1Sl1W<Devalt3Spec> {
        Ta1Tach1Sl1W::new(self, 4)
    }
    #[doc = "Bit 5 - TB1_TACH2PWM_IN Select-1"]
    #[inline(always)]
    pub fn tb1_tach2_sl1(&mut self) -> Tb1Tach2Sl1W<Devalt3Spec> {
        Tb1Tach2Sl1W::new(self, 5)
    }
    #[doc = "Bit 6 - TA2 Select-1"]
    #[inline(always)]
    pub fn ta2_sl1(&mut self) -> Ta2Sl1W<Devalt3Spec> {
        Ta2Sl1W::new(self, 6)
    }
    #[doc = "Bit 7 - TB2 Select-1"]
    #[inline(always)]
    pub fn tb2_sl1(&mut self) -> Tb2Sl1W<Devalt3Spec> {
        Tb2Sl1W::new(self, 7)
    }
}
#[doc = "Device Alternate Function 3 Register (DEVALT3)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt3Spec;
impl crate::RegisterSpec for Devalt3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt3::R`](R) reader structure"]
impl crate::Readable for Devalt3Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt3::W`](W) writer structure"]
impl crate::Writable for Devalt3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT3 to value 0"]
impl crate::Resettable for Devalt3Spec {
    const RESET_VALUE: u8 = 0;
}
