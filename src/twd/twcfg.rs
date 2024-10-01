#[doc = "Register `TWCFG` reader"]
pub type R = crate::R<TwcfgSpec>;
#[doc = "Register `TWCFG` writer"]
pub type W = crate::W<TwcfgSpec>;
#[doc = "Field `LTWCFG` reader - Lock Watchdog Configuration"]
pub type LtwcfgR = crate::BitReader;
#[doc = "Field `LTWCFG` writer - Lock Watchdog Configuration"]
pub type LtwcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTWCP` reader - Lock Prescalers"]
pub type LtwcpR = crate::BitReader;
#[doc = "Field `LTWCP` writer - Lock Prescalers"]
pub type LtwcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTWDT0` reader - Lock T0 Timer"]
pub type Ltwdt0R = crate::BitReader;
#[doc = "Field `LTWDT0` writer - Lock T0 Timer"]
pub type Ltwdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LWDCNT` reader - Lock Watchdog Counter"]
pub type LwdcntR = crate::BitReader;
#[doc = "Field `LWDCNT` writer - Lock Watchdog Counter"]
pub type LwdcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDCT0I` reader - Watchdog Clock Select"]
pub type Wdct0iR = crate::BitReader;
#[doc = "Field `WDCT0I` writer - Watchdog Clock Select"]
pub type Wdct0iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDSDME` reader - Watchdog Touch Select"]
pub type WdsdmeR = crate::BitReader;
#[doc = "Field `WDSDME` writer - Watchdog Touch Select"]
pub type WdsdmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUNLCK` reader - Disable Unlock"]
pub type DisunlckR = crate::BitReader;
#[doc = "Field `DISUNLCK` writer - Disable Unlock"]
pub type DisunlckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock Watchdog Configuration"]
    #[inline(always)]
    pub fn ltwcfg(&self) -> LtwcfgR {
        LtwcfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Prescalers"]
    #[inline(always)]
    pub fn ltwcp(&self) -> LtwcpR {
        LtwcpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock T0 Timer"]
    #[inline(always)]
    pub fn ltwdt0(&self) -> Ltwdt0R {
        Ltwdt0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Watchdog Counter"]
    #[inline(always)]
    pub fn lwdcnt(&self) -> LwdcntR {
        LwdcntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Clock Select"]
    #[inline(always)]
    pub fn wdct0i(&self) -> Wdct0iR {
        Wdct0iR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog Touch Select"]
    #[inline(always)]
    pub fn wdsdme(&self) -> WdsdmeR {
        WdsdmeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Unlock"]
    #[inline(always)]
    pub fn disunlck(&self) -> DisunlckR {
        DisunlckR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWCFG")
            .field("ltwcfg", &self.ltwcfg())
            .field("ltwcp", &self.ltwcp())
            .field("ltwdt0", &self.ltwdt0())
            .field("lwdcnt", &self.lwdcnt())
            .field("wdct0i", &self.wdct0i())
            .field("wdsdme", &self.wdsdme())
            .field("disunlck", &self.disunlck())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock Watchdog Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ltwcfg(&mut self) -> LtwcfgW<TwcfgSpec> {
        LtwcfgW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock Prescalers"]
    #[inline(always)]
    #[must_use]
    pub fn ltwcp(&mut self) -> LtwcpW<TwcfgSpec> {
        LtwcpW::new(self, 1)
    }
    #[doc = "Bit 2 - Lock T0 Timer"]
    #[inline(always)]
    #[must_use]
    pub fn ltwdt0(&mut self) -> Ltwdt0W<TwcfgSpec> {
        Ltwdt0W::new(self, 2)
    }
    #[doc = "Bit 3 - Lock Watchdog Counter"]
    #[inline(always)]
    #[must_use]
    pub fn lwdcnt(&mut self) -> LwdcntW<TwcfgSpec> {
        LwdcntW::new(self, 3)
    }
    #[doc = "Bit 4 - Watchdog Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn wdct0i(&mut self) -> Wdct0iW<TwcfgSpec> {
        Wdct0iW::new(self, 4)
    }
    #[doc = "Bit 5 - Watchdog Touch Select"]
    #[inline(always)]
    #[must_use]
    pub fn wdsdme(&mut self) -> WdsdmeW<TwcfgSpec> {
        WdsdmeW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn disunlck(&mut self) -> DisunlckW<TwcfgSpec> {
        DisunlckW::new(self, 6)
    }
}
#[doc = "Timer and Watchdog Configuration Register (TWCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`twcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwcfgSpec;
impl crate::RegisterSpec for TwcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twcfg::R`](R) reader structure"]
impl crate::Readable for TwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`twcfg::W`](W) writer structure"]
impl crate::Writable for TwcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TWCFG to value 0"]
impl crate::Resettable for TwcfgSpec {
    const RESET_VALUE: u8 = 0;
}
