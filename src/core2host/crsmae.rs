#[doc = "Register `CRSMAE` reader"]
pub type R = crate::R<CrsmaeSpec>;
#[doc = "Register `CRSMAE` writer"]
pub type W = crate::W<CrsmaeSpec>;
#[doc = "Field `CFGAE` reader - Configuration Registers Core Access Enable"]
pub type CfgaeR = crate::BitReader;
#[doc = "Field `CFGAE` writer - Configuration Registers Core Access Enable"]
pub type CfgaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPAE` reader - Serial Port Core Access Enable"]
pub type SpaeR = crate::BitReader;
#[doc = "Field `SPAE` writer - Serial Port Core Access Enable"]
pub type SpaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESMEMAE` reader - ESHM Core Access Enable"]
pub type EsmemaeR = crate::BitReader;
#[doc = "Field `ESMEMAE` writer - ESHM Core Access Enable"]
pub type EsmemaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSWCAE` reader - Mobile System Wake-Up Control Access Enable"]
pub type MswcaeR = crate::BitReader;
#[doc = "Field `MSWCAE` writer - Mobile System Wake-Up Control Access Enable"]
pub type MswcaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEMAE` reader - Shared Memory Core Access Enable"]
pub type SmemaeR = crate::BitReader;
#[doc = "Field `SMEMAE` writer - Shared Memory Core Access Enable"]
pub type SmemaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configuration Registers Core Access Enable"]
    #[inline(always)]
    pub fn cfgae(&self) -> CfgaeR {
        CfgaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Serial Port Core Access Enable"]
    #[inline(always)]
    pub fn spae(&self) -> SpaeR {
        SpaeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ESHM Core Access Enable"]
    #[inline(always)]
    pub fn esmemae(&self) -> EsmemaeR {
        EsmemaeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Mobile System Wake-Up Control Access Enable"]
    #[inline(always)]
    pub fn mswcae(&self) -> MswcaeR {
        MswcaeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Shared Memory Core Access Enable"]
    #[inline(always)]
    pub fn smemae(&self) -> SmemaeR {
        SmemaeR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRSMAE")
            .field("cfgae", &self.cfgae())
            .field("spae", &self.spae())
            .field("esmemae", &self.esmemae())
            .field("mswcae", &self.mswcae())
            .field("smemae", &self.smemae())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Registers Core Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfgae(&mut self) -> CfgaeW<CrsmaeSpec> {
        CfgaeW::new(self, 0)
    }
    #[doc = "Bit 2 - Serial Port Core Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spae(&mut self) -> SpaeW<CrsmaeSpec> {
        SpaeW::new(self, 2)
    }
    #[doc = "Bit 3 - ESHM Core Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn esmemae(&mut self) -> EsmemaeW<CrsmaeSpec> {
        EsmemaeW::new(self, 3)
    }
    #[doc = "Bit 8 - Mobile System Wake-Up Control Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mswcae(&mut self) -> MswcaeW<CrsmaeSpec> {
        MswcaeW::new(self, 8)
    }
    #[doc = "Bit 10 - Shared Memory Core Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smemae(&mut self) -> SmemaeW<CrsmaeSpec> {
        SmemaeW::new(self, 10)
    }
}
#[doc = "Core-to-Host Modules Access Enable Register (CRSMAE)\n\nYou can [`read`](crate::Reg::read) this register and get [`crsmae::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsmae::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsmaeSpec;
impl crate::RegisterSpec for CrsmaeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crsmae::R`](R) reader structure"]
impl crate::Readable for CrsmaeSpec {}
#[doc = "`write(|w| ..)` method takes [`crsmae::W`](W) writer structure"]
impl crate::Writable for CrsmaeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRSMAE to value 0"]
impl crate::Resettable for CrsmaeSpec {
    const RESET_VALUE: u16 = 0;
}
