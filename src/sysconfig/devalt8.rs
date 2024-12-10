#[doc = "Register `DEVALT8` reader"]
pub type R = crate::R<Devalt8Spec>;
#[doc = "Register `DEVALT8` writer"]
pub type W = crate::W<Devalt8Spec>;
#[doc = "Field `NO_KSO00_SL` reader - No KSO00 Select"]
pub type NoKso00SlR = crate::BitReader;
#[doc = "Field `NO_KSO00_SL` writer - No KSO00 Select"]
pub type NoKso00SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO01_SL` reader - No KSO01 Select"]
pub type NoKso01SlR = crate::BitReader;
#[doc = "Field `NO_KSO01_SL` writer - No KSO01 Select"]
pub type NoKso01SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO02_SL` reader - No KSO02 Select"]
pub type NoKso02SlR = crate::BitReader;
#[doc = "Field `NO_KSO02_SL` writer - No KSO02 Select"]
pub type NoKso02SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO03_SL` reader - No KSO03 Select"]
pub type NoKso03SlR = crate::BitReader;
#[doc = "Field `NO_KSO03_SL` writer - No KSO03 Select"]
pub type NoKso03SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO04_SL` reader - No KSO04 Select"]
pub type NoKso04SlR = crate::BitReader;
#[doc = "Field `NO_KSO04_SL` writer - No KSO04 Select"]
pub type NoKso04SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO05_SL` reader - No KSO05 Select"]
pub type NoKso05SlR = crate::BitReader;
#[doc = "Field `NO_KSO05_SL` writer - No KSO05 Select"]
pub type NoKso05SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO06_SL` reader - No KSO06 Select"]
pub type NoKso06SlR = crate::BitReader;
#[doc = "Field `NO_KSO06_SL` writer - No KSO06 Select"]
pub type NoKso06SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO07_SL` reader - No KSO07 Select"]
pub type NoKso07SlR = crate::BitReader;
#[doc = "Field `NO_KSO07_SL` writer - No KSO07 Select"]
pub type NoKso07SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No KSO00 Select"]
    #[inline(always)]
    pub fn no_kso00_sl(&self) -> NoKso00SlR {
        NoKso00SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No KSO01 Select"]
    #[inline(always)]
    pub fn no_kso01_sl(&self) -> NoKso01SlR {
        NoKso01SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No KSO02 Select"]
    #[inline(always)]
    pub fn no_kso02_sl(&self) -> NoKso02SlR {
        NoKso02SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No KSO03 Select"]
    #[inline(always)]
    pub fn no_kso03_sl(&self) -> NoKso03SlR {
        NoKso03SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No KSO04 Select"]
    #[inline(always)]
    pub fn no_kso04_sl(&self) -> NoKso04SlR {
        NoKso04SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No KSO05 Select"]
    #[inline(always)]
    pub fn no_kso05_sl(&self) -> NoKso05SlR {
        NoKso05SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No KSO06 Select"]
    #[inline(always)]
    pub fn no_kso06_sl(&self) -> NoKso06SlR {
        NoKso06SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No KSO07 Select"]
    #[inline(always)]
    pub fn no_kso07_sl(&self) -> NoKso07SlR {
        NoKso07SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT8")
            .field("no_kso00_sl", &self.no_kso00_sl())
            .field("no_kso01_sl", &self.no_kso01_sl())
            .field("no_kso02_sl", &self.no_kso02_sl())
            .field("no_kso03_sl", &self.no_kso03_sl())
            .field("no_kso04_sl", &self.no_kso04_sl())
            .field("no_kso05_sl", &self.no_kso05_sl())
            .field("no_kso06_sl", &self.no_kso06_sl())
            .field("no_kso07_sl", &self.no_kso07_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - No KSO00 Select"]
    #[inline(always)]
    pub fn no_kso00_sl(&mut self) -> NoKso00SlW<Devalt8Spec> {
        NoKso00SlW::new(self, 0)
    }
    #[doc = "Bit 1 - No KSO01 Select"]
    #[inline(always)]
    pub fn no_kso01_sl(&mut self) -> NoKso01SlW<Devalt8Spec> {
        NoKso01SlW::new(self, 1)
    }
    #[doc = "Bit 2 - No KSO02 Select"]
    #[inline(always)]
    pub fn no_kso02_sl(&mut self) -> NoKso02SlW<Devalt8Spec> {
        NoKso02SlW::new(self, 2)
    }
    #[doc = "Bit 3 - No KSO03 Select"]
    #[inline(always)]
    pub fn no_kso03_sl(&mut self) -> NoKso03SlW<Devalt8Spec> {
        NoKso03SlW::new(self, 3)
    }
    #[doc = "Bit 4 - No KSO04 Select"]
    #[inline(always)]
    pub fn no_kso04_sl(&mut self) -> NoKso04SlW<Devalt8Spec> {
        NoKso04SlW::new(self, 4)
    }
    #[doc = "Bit 5 - No KSO05 Select"]
    #[inline(always)]
    pub fn no_kso05_sl(&mut self) -> NoKso05SlW<Devalt8Spec> {
        NoKso05SlW::new(self, 5)
    }
    #[doc = "Bit 6 - No KSO06 Select"]
    #[inline(always)]
    pub fn no_kso06_sl(&mut self) -> NoKso06SlW<Devalt8Spec> {
        NoKso06SlW::new(self, 6)
    }
    #[doc = "Bit 7 - No KSO07 Select"]
    #[inline(always)]
    pub fn no_kso07_sl(&mut self) -> NoKso07SlW<Devalt8Spec> {
        NoKso07SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 8 Register (DEVALT8)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt8Spec;
impl crate::RegisterSpec for Devalt8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt8::R`](R) reader structure"]
impl crate::Readable for Devalt8Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt8::W`](W) writer structure"]
impl crate::Writable for Devalt8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT8 to value 0"]
impl crate::Resettable for Devalt8Spec {
    const RESET_VALUE: u8 = 0;
}
