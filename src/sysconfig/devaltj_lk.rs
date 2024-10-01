#[doc = "Register `DEVALTJ_LK` reader"]
pub type R = crate::R<DevaltjLkSpec>;
#[doc = "Register `DEVALTJ_LK` writer"]
pub type W = crate::W<DevaltjLkSpec>;
#[doc = "Field `CR_SIN2_SL_LK` reader - CR_SIN2 Select Lock"]
pub type CrSin2SlLkR = crate::BitReader;
#[doc = "Field `CR_SIN2_SL_LK` writer - CR_SIN2 Select Lock"]
pub type CrSin2SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT2_SL_LK` reader - CR_SOUT2 Select Lock"]
pub type CrSout2SlLkR = crate::BitReader;
#[doc = "Field `CR_SOUT2_SL_LK` writer - CR_SOUT2 Select Lock"]
pub type CrSout2SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - CR_SIN2 Select Lock"]
    #[inline(always)]
    pub fn cr_sin2_sl_lk(&self) -> CrSin2SlLkR {
        CrSin2SlLkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CR_SOUT2 Select Lock"]
    #[inline(always)]
    pub fn cr_sout2_sl_lk(&self) -> CrSout2SlLkR {
        CrSout2SlLkR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTJ_LK")
            .field("cr_sin2_sl_lk", &self.cr_sin2_sl_lk())
            .field("cr_sout2_sl_lk", &self.cr_sout2_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - CR_SIN2 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sin2_sl_lk(&mut self) -> CrSin2SlLkW<DevaltjLkSpec> {
        CrSin2SlLkW::new(self, 4)
    }
    #[doc = "Bit 5 - CR_SOUT2 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sout2_sl_lk(&mut self) -> CrSout2SlLkW<DevaltjLkSpec> {
        CrSout2SlLkW::new(self, 5)
    }
}
#[doc = "Device Alternate Function J Lock Register (DEVALTJ_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltj_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltj_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltjLkSpec;
impl crate::RegisterSpec for DevaltjLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltj_lk::R`](R) reader structure"]
impl crate::Readable for DevaltjLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltj_lk::W`](W) writer structure"]
impl crate::Writable for DevaltjLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTJ_LK to value 0"]
impl crate::Resettable for DevaltjLkSpec {
    const RESET_VALUE: u8 = 0;
}
