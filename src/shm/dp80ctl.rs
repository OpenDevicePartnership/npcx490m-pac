#[doc = "Register `DP80CTL` reader"]
pub type R = crate::R<Dp80ctlSpec>;
#[doc = "Register `DP80CTL` writer"]
pub type W = crate::W<Dp80ctlSpec>;
#[doc = "Field `DP80EN` reader - Debug Port 80 Enable"]
pub type Dp80enR = crate::BitReader;
#[doc = "Field `DP80EN` writer - Debug Port 80 Enable"]
pub type Dp80enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - Enable LPC SYNC"]
pub type SyncenR = crate::BitReader;
#[doc = "Field `SYNCEN` writer - Enable LPC SYNC"]
pub type SyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADV` reader - Read Advance"]
pub type AdvR = crate::BitReader;
#[doc = "Field `ADV` writer - Read Advance"]
pub type AdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAA` reader - Read Auto Advance"]
pub type RaaR = crate::BitReader;
#[doc = "Field `RAA` writer - Read Auto Advance"]
pub type RaaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO` reader - Reset FIFO"]
pub type RfifoR = crate::BitReader;
#[doc = "Field `RFIFO` writer - Reset FIFO"]
pub type RfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIEN` reader - Core Interrupt Enable"]
pub type CienR = crate::BitReader;
#[doc = "Field `CIEN` writer - Core Interrupt Enable"]
pub type CienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP80_HF_CFG` reader - Debug Port 80 Configure for High-Frequency Clock"]
pub type Dp80HfCfgR = crate::BitReader;
#[doc = "Field `DP80_HF_CFG` writer - Debug Port 80 Configure for High-Frequency Clock"]
pub type Dp80HfCfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Port 80 Enable"]
    #[inline(always)]
    pub fn dp80en(&self) -> Dp80enR {
        Dp80enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable LPC SYNC"]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Advance"]
    #[inline(always)]
    pub fn adv(&self) -> AdvR {
        AdvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read Auto Advance"]
    #[inline(always)]
    pub fn raa(&self) -> RaaR {
        RaaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset FIFO"]
    #[inline(always)]
    pub fn rfifo(&self) -> RfifoR {
        RfifoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core Interrupt Enable"]
    #[inline(always)]
    pub fn cien(&self) -> CienR {
        CienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Port 80 Configure for High-Frequency Clock"]
    #[inline(always)]
    pub fn dp80_hf_cfg(&self) -> Dp80HfCfgR {
        Dp80HfCfgR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DP80CTL")
            .field("dp80en", &self.dp80en())
            .field("syncen", &self.syncen())
            .field("adv", &self.adv())
            .field("raa", &self.raa())
            .field("rfifo", &self.rfifo())
            .field("cien", &self.cien())
            .field("dp80_hf_cfg", &self.dp80_hf_cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Debug Port 80 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dp80en(&mut self) -> Dp80enW<Dp80ctlSpec> {
        Dp80enW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable LPC SYNC"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SyncenW<Dp80ctlSpec> {
        SyncenW::new(self, 1)
    }
    #[doc = "Bit 2 - Read Advance"]
    #[inline(always)]
    #[must_use]
    pub fn adv(&mut self) -> AdvW<Dp80ctlSpec> {
        AdvW::new(self, 2)
    }
    #[doc = "Bit 3 - Read Auto Advance"]
    #[inline(always)]
    #[must_use]
    pub fn raa(&mut self) -> RaaW<Dp80ctlSpec> {
        RaaW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rfifo(&mut self) -> RfifoW<Dp80ctlSpec> {
        RfifoW::new(self, 4)
    }
    #[doc = "Bit 5 - Core Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cien(&mut self) -> CienW<Dp80ctlSpec> {
        CienW::new(self, 5)
    }
    #[doc = "Bit 7 - Debug Port 80 Configure for High-Frequency Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dp80_hf_cfg(&mut self) -> Dp80HfCfgW<Dp80ctlSpec> {
        Dp80HfCfgW::new(self, 7)
    }
}
#[doc = "Debug Port 80 Control Register (DP80CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dp80ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp80ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dp80ctlSpec;
impl crate::RegisterSpec for Dp80ctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dp80ctl::R`](R) reader structure"]
impl crate::Readable for Dp80ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp80ctl::W`](W) writer structure"]
impl crate::Writable for Dp80ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DP80CTL to value 0"]
impl crate::Resettable for Dp80ctlSpec {
    const RESET_VALUE: u8 = 0;
}
