#[doc = "Register `LKSIOHA` reader"]
pub type R = crate::R<LksiohaSpec>;
#[doc = "Register `LKSIOHA` writer"]
pub type W = crate::W<LksiohaSpec>;
#[doc = "Field `LKCFG` reader - Lock Configuration Registers Host Access"]
pub type LkcfgR = crate::BitReader;
#[doc = "Field `LKCFG` writer - Lock Configuration Registers Host Access"]
pub type LkcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKSPHA` reader - Lock Serial Port Host Access"]
pub type LksphaR = crate::BitReader;
#[doc = "Field `LKSPHA` writer - Lock Serial Port Host Access"]
pub type LksphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKESMEM` reader - Lock Extended Shared Memory Host Access"]
pub type LkesmemR = crate::BitReader;
#[doc = "Field `LKESMEM` writer - Lock Extended Shared Memory Host Access"]
pub type LkesmemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKMSWC` reader - Lock Mobile System Wake-Up Control Host Access"]
pub type LkmswcR = crate::BitReader;
#[doc = "Field `LKMSWC` writer - Lock Mobile System Wake-Up Control Host Access"]
pub type LkmswcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKSMEM` reader - Lock Shared Memory Host Access"]
pub type LksmemR = crate::BitReader;
#[doc = "Field `LKSMEM` writer - Lock Shared Memory Host Access"]
pub type LksmemW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock Configuration Registers Host Access"]
    #[inline(always)]
    pub fn lkcfg(&self) -> LkcfgR {
        LkcfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Serial Port Host Access"]
    #[inline(always)]
    pub fn lkspha(&self) -> LksphaR {
        LksphaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Extended Shared Memory Host Access"]
    #[inline(always)]
    pub fn lkesmem(&self) -> LkesmemR {
        LkesmemR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock Mobile System Wake-Up Control Host Access"]
    #[inline(always)]
    pub fn lkmswc(&self) -> LkmswcR {
        LkmswcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock Shared Memory Host Access"]
    #[inline(always)]
    pub fn lksmem(&self) -> LksmemR {
        LksmemR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LKSIOHA")
            .field("lkcfg", &self.lkcfg())
            .field("lkspha", &self.lkspha())
            .field("lkesmem", &self.lkesmem())
            .field("lkmswc", &self.lkmswc())
            .field("lksmem", &self.lksmem())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock Configuration Registers Host Access"]
    #[inline(always)]
    #[must_use]
    pub fn lkcfg(&mut self) -> LkcfgW<LksiohaSpec> {
        LkcfgW::new(self, 0)
    }
    #[doc = "Bit 2 - Lock Serial Port Host Access"]
    #[inline(always)]
    #[must_use]
    pub fn lkspha(&mut self) -> LksphaW<LksiohaSpec> {
        LksphaW::new(self, 2)
    }
    #[doc = "Bit 3 - Lock Extended Shared Memory Host Access"]
    #[inline(always)]
    #[must_use]
    pub fn lkesmem(&mut self) -> LkesmemW<LksiohaSpec> {
        LkesmemW::new(self, 3)
    }
    #[doc = "Bit 8 - Lock Mobile System Wake-Up Control Host Access"]
    #[inline(always)]
    #[must_use]
    pub fn lkmswc(&mut self) -> LkmswcW<LksiohaSpec> {
        LkmswcW::new(self, 8)
    }
    #[doc = "Bit 10 - Lock Shared Memory Host Access"]
    #[inline(always)]
    #[must_use]
    pub fn lksmem(&mut self) -> LksmemW<LksiohaSpec> {
        LksmemW::new(self, 10)
    }
}
#[doc = "Lock Host Access Register (LKSIOHA)\n\nYou can [`read`](crate::Reg::read) this register and get [`lksioha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lksioha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LksiohaSpec;
impl crate::RegisterSpec for LksiohaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lksioha::R`](R) reader structure"]
impl crate::Readable for LksiohaSpec {}
#[doc = "`write(|w| ..)` method takes [`lksioha::W`](W) writer structure"]
impl crate::Writable for LksiohaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LKSIOHA to value 0"]
impl crate::Resettable for LksiohaSpec {
    const RESET_VALUE: u16 = 0;
}
