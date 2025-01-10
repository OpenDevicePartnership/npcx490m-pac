#[doc = "Register `DEVALTC` reader"]
pub type R = crate::R<DevaltcSpec>;
#[doc = "Register `DEVALTC` writer"]
pub type W = crate::W<DevaltcSpec>;
#[doc = "Field `SHI_SL` reader - SHI Interface-Select"]
pub type ShiSlR = crate::BitReader;
#[doc = "Field `SHI_SL` writer - SHI Interface-Select"]
pub type ShiSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO97_SL` reader - GPIO97 Select"]
pub type Gpio97SlR = crate::BitReader;
#[doc = "Field `GPIO97_SL` writer - GPIO97 Select"]
pub type Gpio97SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_3_SL2` reader - PS/2 #3 Select-2"]
pub type Ps2_3Sl2R = crate::BitReader;
#[doc = "Field `PS2_3_SL2` writer - PS/2 #3 Select-2"]
pub type Ps2_3Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA1_SL2` reader - TA1_TACH1 Select-2"]
pub type Ta1Sl2R = crate::BitReader;
#[doc = "Field `TA1_SL2` writer - TA1_TACH1 Select-2"]
pub type Ta1Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1_SL2` reader - TB1_TACH2PWM_IN Select-2"]
pub type Tb1Sl2R = crate::BitReader;
#[doc = "Field `TB1_SL2` writer - TB1_TACH2PWM_IN Select-2"]
pub type Tb1Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2_SL2` reader - TA2 Select-2"]
pub type Ta2Sl2R = crate::BitReader;
#[doc = "Field `TA2_SL2` writer - TA2 Select-2"]
pub type Ta2Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2_SL2` reader - TB2 Select-2"]
pub type Tb2Sl2R = crate::BitReader;
#[doc = "Field `TB2_SL2` writer - TB2 Select-2"]
pub type Tb2Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SHI Interface-Select"]
    #[inline(always)]
    pub fn shi_sl(&self) -> ShiSlR {
        ShiSlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO97 Select"]
    #[inline(always)]
    pub fn gpio97_sl(&self) -> Gpio97SlR {
        Gpio97SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PS/2 #3 Select-2"]
    #[inline(always)]
    pub fn ps2_3_sl2(&self) -> Ps2_3Sl2R {
        Ps2_3Sl2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-2"]
    #[inline(always)]
    pub fn ta1_sl2(&self) -> Ta1Sl2R {
        Ta1Sl2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TB1_TACH2PWM_IN Select-2"]
    #[inline(always)]
    pub fn tb1_sl2(&self) -> Tb1Sl2R {
        Tb1Sl2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TA2 Select-2"]
    #[inline(always)]
    pub fn ta2_sl2(&self) -> Ta2Sl2R {
        Ta2Sl2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TB2 Select-2"]
    #[inline(always)]
    pub fn tb2_sl2(&self) -> Tb2Sl2R {
        Tb2Sl2R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTC")
            .field("ps2_3_sl2", &self.ps2_3_sl2())
            .field("ta1_sl2", &self.ta1_sl2())
            .field("tb1_sl2", &self.tb1_sl2())
            .field("ta2_sl2", &self.ta2_sl2())
            .field("tb2_sl2", &self.tb2_sl2())
            .field("gpio97_sl", &self.gpio97_sl())
            .field("shi_sl", &self.shi_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - SHI Interface-Select"]
    #[inline(always)]
    pub fn shi_sl(&mut self) -> ShiSlW<DevaltcSpec> {
        ShiSlW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO97 Select"]
    #[inline(always)]
    pub fn gpio97_sl(&mut self) -> Gpio97SlW<DevaltcSpec> {
        Gpio97SlW::new(self, 2)
    }
    #[doc = "Bit 3 - PS/2 #3 Select-2"]
    #[inline(always)]
    pub fn ps2_3_sl2(&mut self) -> Ps2_3Sl2W<DevaltcSpec> {
        Ps2_3Sl2W::new(self, 3)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-2"]
    #[inline(always)]
    pub fn ta1_sl2(&mut self) -> Ta1Sl2W<DevaltcSpec> {
        Ta1Sl2W::new(self, 4)
    }
    #[doc = "Bit 5 - TB1_TACH2PWM_IN Select-2"]
    #[inline(always)]
    pub fn tb1_sl2(&mut self) -> Tb1Sl2W<DevaltcSpec> {
        Tb1Sl2W::new(self, 5)
    }
    #[doc = "Bit 6 - TA2 Select-2"]
    #[inline(always)]
    pub fn ta2_sl2(&mut self) -> Ta2Sl2W<DevaltcSpec> {
        Ta2Sl2W::new(self, 6)
    }
    #[doc = "Bit 7 - TB2 Select-2"]
    #[inline(always)]
    pub fn tb2_sl2(&mut self) -> Tb2Sl2W<DevaltcSpec> {
        Tb2Sl2W::new(self, 7)
    }
}
#[doc = "Device Alternate Function C Register (DEVALTC)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltcSpec;
impl crate::RegisterSpec for DevaltcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltc::R`](R) reader structure"]
impl crate::Readable for DevaltcSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltc::W`](W) writer structure"]
impl crate::Writable for DevaltcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTC to value 0"]
impl crate::Resettable for DevaltcSpec {
    const RESET_VALUE: u8 = 0;
}
