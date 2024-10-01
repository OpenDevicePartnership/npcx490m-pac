#[doc = "Register `DEVALTC` reader"]
pub type R = crate::R<DevaltcSpec>;
#[doc = "Register `DEVALTC` writer"]
pub type W = crate::W<DevaltcSpec>;
#[doc = "Field `NO_PVT_CS0` reader - No Private SPI Flash Chip-Select 0 Select"]
pub type NoPvtCs0R = crate::BitReader;
#[doc = "Field `NO_PVT_CS0` writer - No Private SPI Flash Chip-Select 0 Select"]
pub type NoPvtCs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_3_SL2` reader - PS/2 #3 Select-2"]
pub type Ps2_3Sl2R = crate::BitReader;
#[doc = "Field `PS2_3_SL2` writer - PS/2 #3 Select-2"]
pub type Ps2_3Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA1_TACH1_SL2` reader - TA1_TACH1 Select-2"]
pub type Ta1Tach1Sl2R = crate::BitReader;
#[doc = "Field `TA1_TACH1_SL2` writer - TA1_TACH1 Select-2"]
pub type Ta1Tach1Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1_TACH2_SL2` reader - TB1_TACH2PWM_IN Select-2"]
pub type Tb1Tach2Sl2R = crate::BitReader;
#[doc = "Field `TB1_TACH2_SL2` writer - TB1_TACH2PWM_IN Select-2"]
pub type Tb1Tach2Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2_SL2` reader - TA2 Select-2"]
pub type Ta2Sl2R = crate::BitReader;
#[doc = "Field `TA2_SL2` writer - TA2 Select-2"]
pub type Ta2Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2_SL2` reader - TB2 Select-2"]
pub type Tb2Sl2R = crate::BitReader;
#[doc = "Field `TB2_SL2` writer - TB2 Select-2"]
pub type Tb2Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - No Private SPI Flash Chip-Select 0 Select"]
    #[inline(always)]
    pub fn no_pvt_cs0(&self) -> NoPvtCs0R {
        NoPvtCs0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PS/2 #3 Select-2"]
    #[inline(always)]
    pub fn ps2_3_sl2(&self) -> Ps2_3Sl2R {
        Ps2_3Sl2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-2"]
    #[inline(always)]
    pub fn ta1_tach1_sl2(&self) -> Ta1Tach1Sl2R {
        Ta1Tach1Sl2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TB1_TACH2PWM_IN Select-2"]
    #[inline(always)]
    pub fn tb1_tach2_sl2(&self) -> Tb1Tach2Sl2R {
        Tb1Tach2Sl2R::new(((self.bits >> 5) & 1) != 0)
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
            .field("no_pvt_cs0", &self.no_pvt_cs0())
            .field("ps2_3_sl2", &self.ps2_3_sl2())
            .field("ta1_tach1_sl2", &self.ta1_tach1_sl2())
            .field("tb1_tach2_sl2", &self.tb1_tach2_sl2())
            .field("ta2_sl2", &self.ta2_sl2())
            .field("tb2_sl2", &self.tb2_sl2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - No Private SPI Flash Chip-Select 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_pvt_cs0(&mut self) -> NoPvtCs0W<DevaltcSpec> {
        NoPvtCs0W::new(self, 2)
    }
    #[doc = "Bit 3 - PS/2 #3 Select-2"]
    #[inline(always)]
    #[must_use]
    pub fn ps2_3_sl2(&mut self) -> Ps2_3Sl2W<DevaltcSpec> {
        Ps2_3Sl2W::new(self, 3)
    }
    #[doc = "Bit 4 - TA1_TACH1 Select-2"]
    #[inline(always)]
    #[must_use]
    pub fn ta1_tach1_sl2(&mut self) -> Ta1Tach1Sl2W<DevaltcSpec> {
        Ta1Tach1Sl2W::new(self, 4)
    }
    #[doc = "Bit 5 - TB1_TACH2PWM_IN Select-2"]
    #[inline(always)]
    #[must_use]
    pub fn tb1_tach2_sl2(&mut self) -> Tb1Tach2Sl2W<DevaltcSpec> {
        Tb1Tach2Sl2W::new(self, 5)
    }
    #[doc = "Bit 6 - TA2 Select-2"]
    #[inline(always)]
    #[must_use]
    pub fn ta2_sl2(&mut self) -> Ta2Sl2W<DevaltcSpec> {
        Ta2Sl2W::new(self, 6)
    }
    #[doc = "Bit 7 - TB2 Select-2"]
    #[inline(always)]
    #[must_use]
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
