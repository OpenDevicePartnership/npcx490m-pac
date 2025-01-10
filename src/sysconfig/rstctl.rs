#[doc = "Register `RSTCTL` reader"]
pub type R = crate::R<RstctlSpec>;
#[doc = "Register `RSTCTL` writer"]
pub type W = crate::W<RstctlSpec>;
#[doc = "Field `VCC1_RST_STS` reader - VCC1_RST Status"]
pub type Vcc1RstStsR = crate::BitReader;
#[doc = "Field `VCC1_RST_STS` writer - VCC1_RST Status"]
pub type Vcc1RstStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRST_STS` reader - Debugger Reset Status"]
pub type DbgrstStsR = crate::BitReader;
#[doc = "Field `DBGRST_STS` writer - Debugger Reset Status"]
pub type DbgrstStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VCC1_PURST` reader - Force VCC1 Power-Up Reset"]
pub type ForceVcc1PurstR = crate::BitReader;
#[doc = "Field `FORCE_VCC1_PURST` writer - Force VCC1 Power-Up Reset"]
pub type ForceVcc1PurstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRATCH` reader - Scratch"]
pub type ScratchR = crate::BitReader;
#[doc = "Field `SCRATCH` writer - Scratch"]
pub type ScratchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LRESET_PLTRST_MODE` reader - LRESET or PLTRST Mode Select"]
pub type LresetPltrstModeR = crate::BitReader;
#[doc = "Field `LRESET_PLTRST_MODE` writer - LRESET or PLTRST Mode Select"]
pub type LresetPltrstModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIPRST_MODE` reader - Host Interface Modules Reset Mode Select"]
pub type HiprstModeR = crate::BitReader;
#[doc = "Field `HIPRST_MODE` writer - Host Interface Modules Reset Mode Select"]
pub type HiprstModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VCC1_RST Status"]
    #[inline(always)]
    pub fn vcc1_rst_sts(&self) -> Vcc1RstStsR {
        Vcc1RstStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debugger Reset Status"]
    #[inline(always)]
    pub fn dbgrst_sts(&self) -> DbgrstStsR {
        DbgrstStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force VCC1 Power-Up Reset"]
    #[inline(always)]
    pub fn force_vcc1_purst(&self) -> ForceVcc1PurstR {
        ForceVcc1PurstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scratch"]
    #[inline(always)]
    pub fn scratch(&self) -> ScratchR {
        ScratchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LRESET or PLTRST Mode Select"]
    #[inline(always)]
    pub fn lreset_pltrst_mode(&self) -> LresetPltrstModeR {
        LresetPltrstModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host Interface Modules Reset Mode Select"]
    #[inline(always)]
    pub fn hiprst_mode(&self) -> HiprstModeR {
        HiprstModeR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCTL")
            .field("vcc1_rst_sts", &self.vcc1_rst_sts())
            .field("force_vcc1_purst", &self.force_vcc1_purst())
            .field("scratch", &self.scratch())
            .field("lreset_pltrst_mode", &self.lreset_pltrst_mode())
            .field("hiprst_mode", &self.hiprst_mode())
            .field("dbgrst_sts", &self.dbgrst_sts())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VCC1_RST Status"]
    #[inline(always)]
    pub fn vcc1_rst_sts(&mut self) -> Vcc1RstStsW<RstctlSpec> {
        Vcc1RstStsW::new(self, 0)
    }
    #[doc = "Bit 1 - Debugger Reset Status"]
    #[inline(always)]
    pub fn dbgrst_sts(&mut self) -> DbgrstStsW<RstctlSpec> {
        DbgrstStsW::new(self, 1)
    }
    #[doc = "Bit 2 - Force VCC1 Power-Up Reset"]
    #[inline(always)]
    pub fn force_vcc1_purst(&mut self) -> ForceVcc1PurstW<RstctlSpec> {
        ForceVcc1PurstW::new(self, 2)
    }
    #[doc = "Bit 3 - Scratch"]
    #[inline(always)]
    pub fn scratch(&mut self) -> ScratchW<RstctlSpec> {
        ScratchW::new(self, 3)
    }
    #[doc = "Bit 5 - LRESET or PLTRST Mode Select"]
    #[inline(always)]
    pub fn lreset_pltrst_mode(&mut self) -> LresetPltrstModeW<RstctlSpec> {
        LresetPltrstModeW::new(self, 5)
    }
    #[doc = "Bit 6 - Host Interface Modules Reset Mode Select"]
    #[inline(always)]
    pub fn hiprst_mode(&mut self) -> HiprstModeW<RstctlSpec> {
        HiprstModeW::new(self, 6)
    }
}
#[doc = "Reset Control and Status Register (RSTCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlSpec;
impl crate::RegisterSpec for RstctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rstctl::R`](R) reader structure"]
impl crate::Readable for RstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl::W`](W) writer structure"]
impl crate::Writable for RstctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RSTCTL to value 0"]
impl crate::Resettable for RstctlSpec {
    const RESET_VALUE: u8 = 0;
}
