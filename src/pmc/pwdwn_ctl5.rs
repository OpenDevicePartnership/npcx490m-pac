#[doc = "Register `PWDWN_CTL5` reader"]
pub type R = crate::R<PwdwnCtl5Spec>;
#[doc = "Register `PWDWN_CTL5` writer"]
pub type W = crate::W<PwdwnCtl5Spec>;
#[doc = "Field `C2HACC_PD` reader - Core-to-Host Module Power-Down"]
pub type C2haccPdR = crate::BitReader;
#[doc = "Field `C2HACC_PD` writer - Core-to-Host Module Power-Down"]
pub type C2haccPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHM_REG_PD` reader - SHM Registers Power-Down"]
pub type ShmRegPdR = crate::BitReader;
#[doc = "Field `SHM_REG_PD` writer - SHM Registers Power-Down"]
pub type ShmRegPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHM_PD` reader - SHM Power-Down"]
pub type ShmPdR = crate::BitReader;
#[doc = "Field `SHM_PD` writer - SHM Power-Down"]
pub type ShmPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP80_PD` reader - Debug Port 80 Power-Down"]
pub type Dp80PdR = crate::BitReader;
#[doc = "Field `DP80_PD` writer - Debug Port 80 Power-Down"]
pub type Dp80PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSWC_PD` reader - MSWC Power-Down"]
pub type MswcPdR = crate::BitReader;
#[doc = "Field `MSWC_PD` writer - MSWC Power-Down"]
pub type MswcPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Core-to-Host Module Power-Down"]
    #[inline(always)]
    pub fn c2hacc_pd(&self) -> C2haccPdR {
        C2haccPdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SHM Registers Power-Down"]
    #[inline(always)]
    pub fn shm_reg_pd(&self) -> ShmRegPdR {
        ShmRegPdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SHM Power-Down"]
    #[inline(always)]
    pub fn shm_pd(&self) -> ShmPdR {
        ShmPdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Port 80 Power-Down"]
    #[inline(always)]
    pub fn dp80_pd(&self) -> Dp80PdR {
        Dp80PdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MSWC Power-Down"]
    #[inline(always)]
    pub fn mswc_pd(&self) -> MswcPdR {
        MswcPdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL5")
            .field("c2hacc_pd", &self.c2hacc_pd())
            .field("shm_reg_pd", &self.shm_reg_pd())
            .field("shm_pd", &self.shm_pd())
            .field("dp80_pd", &self.dp80_pd())
            .field("mswc_pd", &self.mswc_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Core-to-Host Module Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn c2hacc_pd(&mut self) -> C2haccPdW<PwdwnCtl5Spec> {
        C2haccPdW::new(self, 3)
    }
    #[doc = "Bit 4 - SHM Registers Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn shm_reg_pd(&mut self) -> ShmRegPdW<PwdwnCtl5Spec> {
        ShmRegPdW::new(self, 4)
    }
    #[doc = "Bit 5 - SHM Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn shm_pd(&mut self) -> ShmPdW<PwdwnCtl5Spec> {
        ShmPdW::new(self, 5)
    }
    #[doc = "Bit 6 - Debug Port 80 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn dp80_pd(&mut self) -> Dp80PdW<PwdwnCtl5Spec> {
        Dp80PdW::new(self, 6)
    }
    #[doc = "Bit 7 - MSWC Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn mswc_pd(&mut self) -> MswcPdW<PwdwnCtl5Spec> {
        MswcPdW::new(self, 7)
    }
}
#[doc = "Power-Down Control 5 Register (PWDWN_CTL5)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl5Spec;
impl crate::RegisterSpec for PwdwnCtl5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl5::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl5Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl5::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL5 to value 0"]
impl crate::Resettable for PwdwnCtl5Spec {
    const RESET_VALUE: u8 = 0;
}
