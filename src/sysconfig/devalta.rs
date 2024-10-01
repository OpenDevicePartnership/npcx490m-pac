#[doc = "Register `DEVALTA` reader"]
pub type R = crate::R<DevaltaSpec>;
#[doc = "Register `DEVALTA` writer"]
pub type W = crate::W<DevaltaSpec>;
#[doc = "Field `NO_KSO16_SL` reader - No KSO16 Select"]
pub type NoKso16SlR = crate::BitReader;
#[doc = "Field `NO_KSO16_SL` writer - No KSO16 Select"]
pub type NoKso16SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSO17_SL` reader - No KSO17 Select"]
pub type NoKso17SlR = crate::BitReader;
#[doc = "Field `NO_KSO17_SL` writer - No KSO17 Select"]
pub type NoKso17SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `32K_OUT_SL` reader - 32KHZ_OUT Select"]
pub type _32kOutSlR = crate::BitReader;
#[doc = "Field `32K_OUT_SL` writer - 32KHZ_OUT Select"]
pub type _32kOutSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `32KCLKIN_SL` reader - 32KCLKIN Select"]
pub type _32kclkinSlR = crate::BitReader;
#[doc = "Field `32KCLKIN_SL` writer - 32KCLKIN Select"]
pub type _32kclkinSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_VCC1_RST` reader - No VCC1_RST Select"]
pub type NoVcc1RstR = crate::BitReader;
#[doc = "Field `NO_VCC1_RST` writer - No VCC1_RST Select"]
pub type NoVcc1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_PECI_EN` reader - No PECI Enable"]
pub type NoPeciEnR = crate::BitReader;
#[doc = "Field `NO_PECI_EN` writer - No PECI Enable"]
pub type NoPeciEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No KSO16 Select"]
    #[inline(always)]
    pub fn no_kso16_sl(&self) -> NoKso16SlR {
        NoKso16SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No KSO17 Select"]
    #[inline(always)]
    pub fn no_kso17_sl(&self) -> NoKso17SlR {
        NoKso17SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32KHZ_OUT Select"]
    #[inline(always)]
    pub fn _32k_out_sl(&self) -> _32kOutSlR {
        _32kOutSlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 32KCLKIN Select"]
    #[inline(always)]
    pub fn _32kclkin_sl(&self) -> _32kclkinSlR {
        _32kclkinSlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No VCC1_RST Select"]
    #[inline(always)]
    pub fn no_vcc1_rst(&self) -> NoVcc1RstR {
        NoVcc1RstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - No PECI Enable"]
    #[inline(always)]
    pub fn no_peci_en(&self) -> NoPeciEnR {
        NoPeciEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTA")
            .field("no_kso16_sl", &self.no_kso16_sl())
            .field("no_kso17_sl", &self.no_kso17_sl())
            .field("_32k_out_sl", &self._32k_out_sl())
            .field("_32kclkin_sl", &self._32kclkin_sl())
            .field("no_vcc1_rst", &self.no_vcc1_rst())
            .field("no_peci_en", &self.no_peci_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - No KSO16 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_kso16_sl(&mut self) -> NoKso16SlW<DevaltaSpec> {
        NoKso16SlW::new(self, 0)
    }
    #[doc = "Bit 1 - No KSO17 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_kso17_sl(&mut self) -> NoKso17SlW<DevaltaSpec> {
        NoKso17SlW::new(self, 1)
    }
    #[doc = "Bit 2 - 32KHZ_OUT Select"]
    #[inline(always)]
    #[must_use]
    pub fn _32k_out_sl(&mut self) -> _32kOutSlW<DevaltaSpec> {
        _32kOutSlW::new(self, 2)
    }
    #[doc = "Bit 3 - 32KCLKIN Select"]
    #[inline(always)]
    #[must_use]
    pub fn _32kclkin_sl(&mut self) -> _32kclkinSlW<DevaltaSpec> {
        _32kclkinSlW::new(self, 3)
    }
    #[doc = "Bit 4 - No VCC1_RST Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_vcc1_rst(&mut self) -> NoVcc1RstW<DevaltaSpec> {
        NoVcc1RstW::new(self, 4)
    }
    #[doc = "Bit 6 - No PECI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn no_peci_en(&mut self) -> NoPeciEnW<DevaltaSpec> {
        NoPeciEnW::new(self, 6)
    }
}
#[doc = "Device Alternate Function A Register (DEVALTA)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltaSpec;
impl crate::RegisterSpec for DevaltaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalta::R`](R) reader structure"]
impl crate::Readable for DevaltaSpec {}
#[doc = "`write(|w| ..)` method takes [`devalta::W`](W) writer structure"]
impl crate::Writable for DevaltaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTA to value 0"]
impl crate::Resettable for DevaltaSpec {
    const RESET_VALUE: u8 = 0;
}
