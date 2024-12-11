#[doc = "Register `DEVALT9` reader"]
pub type R = crate::R<Devalt9Spec>;
#[doc = "Register `DEVALT9` writer"]
pub type W = crate::W<Devalt9Spec>;
#[doc = "Field `NO_KSO08_SL` reader - No KSO08 Select"]
pub type NoKso08SlR = crate::BitReader;
#[doc = "Field `NO_KSO08_SL` writer - No KSO08 Select"]
pub type NoKso08SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO09_SL` reader - No KSO09 Select"]
pub type NoKso09SlR = crate::BitReader;
#[doc = "Field `NO_KSO09_SL` writer - No KSO09 Select"]
pub type NoKso09SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO10_SL` reader - No KSO10 and P80_CLK Select"]
pub type NoKso10SlR = crate::BitReader;
#[doc = "Field `NO_KSO10_SL` writer - No KSO10 and P80_CLK Select"]
pub type NoKso10SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO11_SL` reader - No KSO11 and P80_DAT Select"]
pub type NoKso11SlR = crate::BitReader;
#[doc = "Field `NO_KSO11_SL` writer - No KSO11 and P80_DAT Select"]
pub type NoKso11SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO12_SL` reader - No KSO12 Select"]
pub type NoKso12SlR = crate::BitReader;
#[doc = "Field `NO_KSO12_SL` writer - No KSO12 Select"]
pub type NoKso12SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO13_SL` reader - No KSO13 Select"]
pub type NoKso13SlR = crate::BitReader;
#[doc = "Field `NO_KSO13_SL` writer - No KSO13 Select"]
pub type NoKso13SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO14_SL` reader - No KSO14 Select"]
pub type NoKso14SlR = crate::BitReader;
#[doc = "Field `NO_KSO14_SL` writer - No KSO14 Select"]
pub type NoKso14SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO15_SL` reader - No KSO15 Select"]
pub type NoKso15SlR = crate::BitReader;
#[doc = "Field `NO_KSO15_SL` writer - No KSO15 Select"]
pub type NoKso15SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No KSO08 Select"]
    #[inline(always)]
    pub fn no_kso08_sl(&self) -> NoKso08SlR {
        NoKso08SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No KSO09 Select"]
    #[inline(always)]
    pub fn no_kso09_sl(&self) -> NoKso09SlR {
        NoKso09SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No KSO10 and P80_CLK Select"]
    #[inline(always)]
    pub fn no_kso10_sl(&self) -> NoKso10SlR {
        NoKso10SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No KSO11 and P80_DAT Select"]
    #[inline(always)]
    pub fn no_kso11_sl(&self) -> NoKso11SlR {
        NoKso11SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No KSO12 Select"]
    #[inline(always)]
    pub fn no_kso12_sl(&self) -> NoKso12SlR {
        NoKso12SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No KSO13 Select"]
    #[inline(always)]
    pub fn no_kso13_sl(&self) -> NoKso13SlR {
        NoKso13SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No KSO14 Select"]
    #[inline(always)]
    pub fn no_kso14_sl(&self) -> NoKso14SlR {
        NoKso14SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No KSO15 Select"]
    #[inline(always)]
    pub fn no_kso15_sl(&self) -> NoKso15SlR {
        NoKso15SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT9")
            .field("no_kso08_sl", &self.no_kso08_sl())
            .field("no_kso09_sl", &self.no_kso09_sl())
            .field("no_kso10_sl", &self.no_kso10_sl())
            .field("no_kso11_sl", &self.no_kso11_sl())
            .field("no_kso12_sl", &self.no_kso12_sl())
            .field("no_kso13_sl", &self.no_kso13_sl())
            .field("no_kso14_sl", &self.no_kso14_sl())
            .field("no_kso15_sl", &self.no_kso15_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - No KSO08 Select"]
    #[inline(always)]
    pub fn no_kso08_sl(&mut self) -> NoKso08SlW<Devalt9Spec> {
        NoKso08SlW::new(self, 0)
    }
    #[doc = "Bit 1 - No KSO09 Select"]
    #[inline(always)]
    pub fn no_kso09_sl(&mut self) -> NoKso09SlW<Devalt9Spec> {
        NoKso09SlW::new(self, 1)
    }
    #[doc = "Bit 2 - No KSO10 and P80_CLK Select"]
    #[inline(always)]
    pub fn no_kso10_sl(&mut self) -> NoKso10SlW<Devalt9Spec> {
        NoKso10SlW::new(self, 2)
    }
    #[doc = "Bit 3 - No KSO11 and P80_DAT Select"]
    #[inline(always)]
    pub fn no_kso11_sl(&mut self) -> NoKso11SlW<Devalt9Spec> {
        NoKso11SlW::new(self, 3)
    }
    #[doc = "Bit 4 - No KSO12 Select"]
    #[inline(always)]
    pub fn no_kso12_sl(&mut self) -> NoKso12SlW<Devalt9Spec> {
        NoKso12SlW::new(self, 4)
    }
    #[doc = "Bit 5 - No KSO13 Select"]
    #[inline(always)]
    pub fn no_kso13_sl(&mut self) -> NoKso13SlW<Devalt9Spec> {
        NoKso13SlW::new(self, 5)
    }
    #[doc = "Bit 6 - No KSO14 Select"]
    #[inline(always)]
    pub fn no_kso14_sl(&mut self) -> NoKso14SlW<Devalt9Spec> {
        NoKso14SlW::new(self, 6)
    }
    #[doc = "Bit 7 - No KSO15 Select"]
    #[inline(always)]
    pub fn no_kso15_sl(&mut self) -> NoKso15SlW<Devalt9Spec> {
        NoKso15SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 9 Register (DEVALT9)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt9Spec;
impl crate::RegisterSpec for Devalt9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt9::R`](R) reader structure"]
impl crate::Readable for Devalt9Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt9::W`](W) writer structure"]
impl crate::Writable for Devalt9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT9 to value 0"]
impl crate::Resettable for Devalt9Spec {
    const RESET_VALUE: u8 = 0;
}
