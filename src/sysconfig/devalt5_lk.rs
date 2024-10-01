#[doc = "Register `DEVALT5_LK` reader"]
pub type R = crate::R<Devalt5LkSpec>;
#[doc = "Register `DEVALT5_LK` writer"]
pub type W = crate::W<Devalt5LkSpec>;
#[doc = "Field `JEN_LK_LK` reader - JTAG Enable Lock, Lock"]
pub type JenLkLkR = crate::BitReader;
#[doc = "Field `JEN_LK_LK` writer - JTAG Enable Lock, Lock"]
pub type JenLkLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD1_SL_LK` reader - Intruder 1 Detect Select Lock"]
pub type Intrud1SlLkR = crate::BitReader;
#[doc = "Field `INTRUD1_SL_LK` writer - Intruder 1 Detect Select Lock"]
pub type Intrud1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD2_SL_LK` reader - Intruder 2 Detect Select Lock"]
pub type Intrud2SlLkR = crate::BitReader;
#[doc = "Field `INTRUD2_SL_LK` writer - Intruder 2 Detect Select Lock"]
pub type Intrud2SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - JTAG Enable Lock, Lock"]
    #[inline(always)]
    pub fn jen_lk_lk(&self) -> JenLkLkR {
        JenLkLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Intruder 1 Detect Select Lock"]
    #[inline(always)]
    pub fn intrud1_sl_lk(&self) -> Intrud1SlLkR {
        Intrud1SlLkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Intruder 2 Detect Select Lock"]
    #[inline(always)]
    pub fn intrud2_sl_lk(&self) -> Intrud2SlLkR {
        Intrud2SlLkR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT5_LK")
            .field("jen_lk_lk", &self.jen_lk_lk())
            .field("intrud1_sl_lk", &self.intrud1_sl_lk())
            .field("intrud2_sl_lk", &self.intrud2_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - JTAG Enable Lock, Lock"]
    #[inline(always)]
    #[must_use]
    pub fn jen_lk_lk(&mut self) -> JenLkLkW<Devalt5LkSpec> {
        JenLkLkW::new(self, 1)
    }
    #[doc = "Bit 2 - Intruder 1 Detect Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn intrud1_sl_lk(&mut self) -> Intrud1SlLkW<Devalt5LkSpec> {
        Intrud1SlLkW::new(self, 2)
    }
    #[doc = "Bit 3 - Intruder 2 Detect Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn intrud2_sl_lk(&mut self) -> Intrud2SlLkW<Devalt5LkSpec> {
        Intrud2SlLkW::new(self, 3)
    }
}
#[doc = "Device Alternate Function 5 Lock Register (DEVALT5_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt5_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt5_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt5LkSpec;
impl crate::RegisterSpec for Devalt5LkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt5_lk::R`](R) reader structure"]
impl crate::Readable for Devalt5LkSpec {}
#[doc = "`write(|w| ..)` method takes [`devalt5_lk::W`](W) writer structure"]
impl crate::Writable for Devalt5LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT5_LK to value 0"]
impl crate::Resettable for Devalt5LkSpec {
    const RESET_VALUE: u8 = 0;
}
