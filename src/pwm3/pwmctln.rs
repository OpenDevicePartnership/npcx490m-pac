#[doc = "Register `PWMCTLn` reader"]
pub type R = crate::R<PwmctlnSpec>;
#[doc = "Register `PWMCTLn` writer"]
pub type W = crate::W<PwmctlnSpec>;
#[doc = "Field `INVP` reader - Inverse PWM Output"]
pub type InvpR = crate::BitReader;
#[doc = "Field `INVP` writer - Inverse PWM Output"]
pub type InvpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSEL` reader - Input Clock Select"]
pub type CkselR = crate::BitReader;
#[doc = "Field `CKSEL` writer - Input Clock Select"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HB_DC_CTL` reader - Heartbeat Duty Cycle Control"]
pub type HbDcCtlR = crate::FieldReader;
#[doc = "Field `HB_DC_CTL` writer - Heartbeat Duty Cycle Control"]
pub type HbDcCtlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC` reader - Operation Synchronization"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - Operation Synchronization"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBNK_SEL` reader - Heartbeat Bank Select"]
pub type HbnkSelR = crate::BitReader;
#[doc = "Field `HBNK_SEL` writer - Heartbeat Bank Select"]
pub type HbnkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR` reader - Power Mode"]
pub type PwrR = crate::BitReader;
#[doc = "Field `PWR` writer - Power Mode"]
pub type PwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Inverse PWM Output"]
    #[inline(always)]
    pub fn invp(&self) -> InvpR {
        InvpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Clock Select"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Heartbeat Duty Cycle Control"]
    #[inline(always)]
    pub fn hb_dc_ctl(&self) -> HbDcCtlR {
        HbDcCtlR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Operation Synchronization"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Heartbeat Bank Select"]
    #[inline(always)]
    pub fn hbnk_sel(&self) -> HbnkSelR {
        HbnkSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power Mode"]
    #[inline(always)]
    pub fn pwr(&self) -> PwrR {
        PwrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMCTLn")
            .field("invp", &self.invp())
            .field("cksel", &self.cksel())
            .field("hb_dc_ctl", &self.hb_dc_ctl())
            .field("sync", &self.sync())
            .field("hbnk_sel", &self.hbnk_sel())
            .field("pwr", &self.pwr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Inverse PWM Output"]
    #[inline(always)]
    #[must_use]
    pub fn invp(&mut self) -> InvpW<PwmctlnSpec> {
        InvpW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CkselW<PwmctlnSpec> {
        CkselW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Heartbeat Duty Cycle Control"]
    #[inline(always)]
    #[must_use]
    pub fn hb_dc_ctl(&mut self) -> HbDcCtlW<PwmctlnSpec> {
        HbDcCtlW::new(self, 2)
    }
    #[doc = "Bit 4 - Operation Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<PwmctlnSpec> {
        SyncW::new(self, 4)
    }
    #[doc = "Bit 6 - Heartbeat Bank Select"]
    #[inline(always)]
    #[must_use]
    pub fn hbnk_sel(&mut self) -> HbnkSelW<PwmctlnSpec> {
        HbnkSelW::new(self, 6)
    }
    #[doc = "Bit 7 - Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwr(&mut self) -> PwrW<PwmctlnSpec> {
        PwrW::new(self, 7)
    }
}
#[doc = "PWM Control Register (PWMCTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmctln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmctln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmctlnSpec;
impl crate::RegisterSpec for PwmctlnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwmctln::R`](R) reader structure"]
impl crate::Readable for PwmctlnSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmctln::W`](W) writer structure"]
impl crate::Writable for PwmctlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWMCTLn to value 0"]
impl crate::Resettable for PwmctlnSpec {
    const RESET_VALUE: u8 = 0;
}
