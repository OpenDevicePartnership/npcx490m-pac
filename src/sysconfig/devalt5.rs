#[doc = "Register `DEVALT5` reader"]
pub type R = crate::R<Devalt5Spec>;
#[doc = "Register `DEVALT5` writer"]
pub type W = crate::W<Devalt5Spec>;
#[doc = "Field `TRACE_EN` reader - Parallel Trace Signals Enable"]
pub type TraceEnR = crate::BitReader;
#[doc = "Field `TRACE_EN` writer - Parallel Trace Signals Enable"]
pub type TraceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEN_LK` reader - JTAG Enable Lock"]
pub type JenLkR = crate::BitReader;
#[doc = "Field `JEN_LK` writer - JTAG Enable Lock"]
pub type JenLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD1_SL` reader - Intruder Detect 1 Select"]
pub type Intrud1SlR = crate::BitReader;
#[doc = "Field `INTRUD1_SL` writer - Intruder Detect 1 Select"]
pub type Intrud1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD2_SL` reader - Intruder Detect 2 Select"]
pub type Intrud2SlR = crate::BitReader;
#[doc = "Field `INTRUD2_SL` writer - Intruder Detect 2 Select"]
pub type Intrud2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRACE_EN` reader - Serial Trace Enable"]
pub type StraceEnR = crate::BitReader;
#[doc = "Field `STRACE_EN` writer - Serial Trace Enable"]
pub type StraceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GP_LK` reader - Gang Programmer Mode Lock"]
pub type GpLkR = crate::BitReader;
#[doc = "Field `GP_LK` writer - Gang Programmer Mode Lock"]
pub type GpLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parallel Trace Signals Enable"]
    #[inline(always)]
    pub fn trace_en(&self) -> TraceEnR {
        TraceEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - JTAG Enable Lock"]
    #[inline(always)]
    pub fn jen_lk(&self) -> JenLkR {
        JenLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Intruder Detect 1 Select"]
    #[inline(always)]
    pub fn intrud1_sl(&self) -> Intrud1SlR {
        Intrud1SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Intruder Detect 2 Select"]
    #[inline(always)]
    pub fn intrud2_sl(&self) -> Intrud2SlR {
        Intrud2SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial Trace Enable"]
    #[inline(always)]
    pub fn strace_en(&self) -> StraceEnR {
        StraceEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Gang Programmer Mode Lock"]
    #[inline(always)]
    pub fn gp_lk(&self) -> GpLkR {
        GpLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT5")
            .field("trace_en", &self.trace_en())
            .field("jen_lk", &self.jen_lk())
            .field("intrud1_sl", &self.intrud1_sl())
            .field("intrud2_sl", &self.intrud2_sl())
            .field("strace_en", &self.strace_en())
            .field("gp_lk", &self.gp_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Trace Signals Enable"]
    #[inline(always)]
    pub fn trace_en(&mut self) -> TraceEnW<Devalt5Spec> {
        TraceEnW::new(self, 0)
    }
    #[doc = "Bit 1 - JTAG Enable Lock"]
    #[inline(always)]
    pub fn jen_lk(&mut self) -> JenLkW<Devalt5Spec> {
        JenLkW::new(self, 1)
    }
    #[doc = "Bit 2 - Intruder Detect 1 Select"]
    #[inline(always)]
    pub fn intrud1_sl(&mut self) -> Intrud1SlW<Devalt5Spec> {
        Intrud1SlW::new(self, 2)
    }
    #[doc = "Bit 3 - Intruder Detect 2 Select"]
    #[inline(always)]
    pub fn intrud2_sl(&mut self) -> Intrud2SlW<Devalt5Spec> {
        Intrud2SlW::new(self, 3)
    }
    #[doc = "Bit 4 - Serial Trace Enable"]
    #[inline(always)]
    pub fn strace_en(&mut self) -> StraceEnW<Devalt5Spec> {
        StraceEnW::new(self, 4)
    }
    #[doc = "Bit 7 - Gang Programmer Mode Lock"]
    #[inline(always)]
    pub fn gp_lk(&mut self) -> GpLkW<Devalt5Spec> {
        GpLkW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 5 Register (DEVALT5)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt5Spec;
impl crate::RegisterSpec for Devalt5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt5::R`](R) reader structure"]
impl crate::Readable for Devalt5Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt5::W`](W) writer structure"]
impl crate::Writable for Devalt5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT5 to value 0"]
impl crate::Resettable for Devalt5Spec {
    const RESET_VALUE: u8 = 0;
}
