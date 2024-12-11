#[doc = "Register `DEVALTJ` reader"]
pub type R = crate::R<DevaltjSpec>;
#[doc = "Register `DEVALTJ` writer"]
pub type W = crate::W<DevaltjSpec>;
#[doc = "Field `CR_SIN1_SL1` reader - CR_SIN1 Select-1"]
pub type CrSin1Sl1R = crate::BitReader;
#[doc = "Field `CR_SIN1_SL1` writer - CR_SIN1 Select-1"]
pub type CrSin1Sl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT1_SL1` reader - CR_SOUT1 Select-1"]
pub type CrSout1Sl1R = crate::BitReader;
#[doc = "Field `CR_SOUT1_SL1` writer - CR_SOUT1 Select-1"]
pub type CrSout1Sl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SIN1_SL2` reader - CR_SIN1 Select-2"]
pub type CrSin1Sl2R = crate::BitReader;
#[doc = "Field `CR_SIN1_SL2` writer - CR_SIN1 Select-2"]
pub type CrSin1Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT1_SL2` reader - CR_SOUT1 Select-2"]
pub type CrSout1Sl2R = crate::BitReader;
#[doc = "Field `CR_SOUT1_SL2` writer - CR_SOUT1 Select-2"]
pub type CrSout1Sl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SIN2_SL` reader - CR_SIN2 Select"]
pub type CrSin2SlR = crate::BitReader;
#[doc = "Field `CR_SIN2_SL` writer - CR_SIN2 Select"]
pub type CrSin2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT2_SL` reader - CR_SOUT2 Select"]
pub type CrSout2SlR = crate::BitReader;
#[doc = "Field `CR_SOUT2_SL` writer - CR_SOUT2 Select"]
pub type CrSout2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SIN3_SL` reader - CR_SIN3 Select"]
pub type CrSin3SlR = crate::BitReader;
#[doc = "Field `CR_SIN3_SL` writer - CR_SIN3 Select"]
pub type CrSin3SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT3_SL` reader - CR_SOUT3 Select"]
pub type CrSout3SlR = crate::BitReader;
#[doc = "Field `CR_SOUT3_SL` writer - CR_SOUT3 Select"]
pub type CrSout3SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CR_SIN1 Select-1"]
    #[inline(always)]
    pub fn cr_sin1_sl1(&self) -> CrSin1Sl1R {
        CrSin1Sl1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR_SOUT1 Select-1"]
    #[inline(always)]
    pub fn cr_sout1_sl1(&self) -> CrSout1Sl1R {
        CrSout1Sl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CR_SIN1 Select-2"]
    #[inline(always)]
    pub fn cr_sin1_sl2(&self) -> CrSin1Sl2R {
        CrSin1Sl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CR_SOUT1 Select-2"]
    #[inline(always)]
    pub fn cr_sout1_sl2(&self) -> CrSout1Sl2R {
        CrSout1Sl2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CR_SIN2 Select"]
    #[inline(always)]
    pub fn cr_sin2_sl(&self) -> CrSin2SlR {
        CrSin2SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CR_SOUT2 Select"]
    #[inline(always)]
    pub fn cr_sout2_sl(&self) -> CrSout2SlR {
        CrSout2SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CR_SIN3 Select"]
    #[inline(always)]
    pub fn cr_sin3_sl(&self) -> CrSin3SlR {
        CrSin3SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CR_SOUT3 Select"]
    #[inline(always)]
    pub fn cr_sout3_sl(&self) -> CrSout3SlR {
        CrSout3SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTJ")
            .field("cr_sin1_sl1", &self.cr_sin1_sl1())
            .field("cr_sout1_sl1", &self.cr_sout1_sl1())
            .field("cr_sin1_sl2", &self.cr_sin1_sl2())
            .field("cr_sout1_sl2", &self.cr_sout1_sl2())
            .field("cr_sin2_sl", &self.cr_sin2_sl())
            .field("cr_sout2_sl", &self.cr_sout2_sl())
            .field("cr_sin3_sl", &self.cr_sin3_sl())
            .field("cr_sout3_sl", &self.cr_sout3_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CR_SIN1 Select-1"]
    #[inline(always)]
    pub fn cr_sin1_sl1(&mut self) -> CrSin1Sl1W<DevaltjSpec> {
        CrSin1Sl1W::new(self, 0)
    }
    #[doc = "Bit 1 - CR_SOUT1 Select-1"]
    #[inline(always)]
    pub fn cr_sout1_sl1(&mut self) -> CrSout1Sl1W<DevaltjSpec> {
        CrSout1Sl1W::new(self, 1)
    }
    #[doc = "Bit 2 - CR_SIN1 Select-2"]
    #[inline(always)]
    pub fn cr_sin1_sl2(&mut self) -> CrSin1Sl2W<DevaltjSpec> {
        CrSin1Sl2W::new(self, 2)
    }
    #[doc = "Bit 3 - CR_SOUT1 Select-2"]
    #[inline(always)]
    pub fn cr_sout1_sl2(&mut self) -> CrSout1Sl2W<DevaltjSpec> {
        CrSout1Sl2W::new(self, 3)
    }
    #[doc = "Bit 4 - CR_SIN2 Select"]
    #[inline(always)]
    pub fn cr_sin2_sl(&mut self) -> CrSin2SlW<DevaltjSpec> {
        CrSin2SlW::new(self, 4)
    }
    #[doc = "Bit 5 - CR_SOUT2 Select"]
    #[inline(always)]
    pub fn cr_sout2_sl(&mut self) -> CrSout2SlW<DevaltjSpec> {
        CrSout2SlW::new(self, 5)
    }
    #[doc = "Bit 6 - CR_SIN3 Select"]
    #[inline(always)]
    pub fn cr_sin3_sl(&mut self) -> CrSin3SlW<DevaltjSpec> {
        CrSin3SlW::new(self, 6)
    }
    #[doc = "Bit 7 - CR_SOUT3 Select"]
    #[inline(always)]
    pub fn cr_sout3_sl(&mut self) -> CrSout3SlW<DevaltjSpec> {
        CrSout3SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function J Register (DEVALTJ)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltjSpec;
impl crate::RegisterSpec for DevaltjSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltj::R`](R) reader structure"]
impl crate::Readable for DevaltjSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltj::W`](W) writer structure"]
impl crate::Writable for DevaltjSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTJ to value 0"]
impl crate::Resettable for DevaltjSpec {
    const RESET_VALUE: u8 = 0;
}
