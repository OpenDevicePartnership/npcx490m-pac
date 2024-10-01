#[doc = "Register `DEVALT7` reader"]
pub type R = crate::R<Devalt7Spec>;
#[doc = "Register `DEVALT7` writer"]
pub type W = crate::W<Devalt7Spec>;
#[doc = "Field `NO_KSI0_SL` reader - No KSI0 Select"]
pub type NoKsi0SlR = crate::BitReader;
#[doc = "Field `NO_KSI0_SL` writer - No KSI0 Select"]
pub type NoKsi0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSI1_SL` reader - No KSI1 Select"]
pub type NoKsi1SlR = crate::BitReader;
#[doc = "Field `NO_KSI1_SL` writer - No KSI1 Select"]
pub type NoKsi1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSI2_SL` reader - No KSI2 Select"]
pub type NoKsi2SlR = crate::BitReader;
#[doc = "Field `NO_KSI2_SL` writer - No KSI2 Select"]
pub type NoKsi2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSI3_SL` reader - No KSI3 Select"]
pub type NoKsi3SlR = crate::BitReader;
#[doc = "Field `NO_KSI3_SL` writer - No KSI3 Select"]
pub type NoKsi3SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSI4_SL` reader - No KSI4 Select"]
pub type NoKsi4SlR = crate::BitReader;
#[doc = "Field `NO_KSI4_SL` writer - No KSI4 Select"]
pub type NoKsi4SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSI5_SL` reader - No KSI5 Select"]
pub type NoKsi5SlR = crate::BitReader;
#[doc = "Field `NO_KSI5_SL` writer - No KSI5 Select"]
pub type NoKsi5SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSI6_SL` reader - No KSI6 Select"]
pub type NoKsi6SlR = crate::BitReader;
#[doc = "Field `NO_KSI6_SL` writer - No KSI6 Select"]
pub type NoKsi6SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_KSI7_SL` reader - No KSI7 Select"]
pub type NoKsi7SlR = crate::BitReader;
#[doc = "Field `NO_KSI7_SL` writer - No KSI7 Select"]
pub type NoKsi7SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No KSI0 Select"]
    #[inline(always)]
    pub fn no_ksi0_sl(&self) -> NoKsi0SlR {
        NoKsi0SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No KSI1 Select"]
    #[inline(always)]
    pub fn no_ksi1_sl(&self) -> NoKsi1SlR {
        NoKsi1SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No KSI2 Select"]
    #[inline(always)]
    pub fn no_ksi2_sl(&self) -> NoKsi2SlR {
        NoKsi2SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No KSI3 Select"]
    #[inline(always)]
    pub fn no_ksi3_sl(&self) -> NoKsi3SlR {
        NoKsi3SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No KSI4 Select"]
    #[inline(always)]
    pub fn no_ksi4_sl(&self) -> NoKsi4SlR {
        NoKsi4SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No KSI5 Select"]
    #[inline(always)]
    pub fn no_ksi5_sl(&self) -> NoKsi5SlR {
        NoKsi5SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - No KSI6 Select"]
    #[inline(always)]
    pub fn no_ksi6_sl(&self) -> NoKsi6SlR {
        NoKsi6SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No KSI7 Select"]
    #[inline(always)]
    pub fn no_ksi7_sl(&self) -> NoKsi7SlR {
        NoKsi7SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT7")
            .field("no_ksi0_sl", &self.no_ksi0_sl())
            .field("no_ksi1_sl", &self.no_ksi1_sl())
            .field("no_ksi2_sl", &self.no_ksi2_sl())
            .field("no_ksi3_sl", &self.no_ksi3_sl())
            .field("no_ksi4_sl", &self.no_ksi4_sl())
            .field("no_ksi5_sl", &self.no_ksi5_sl())
            .field("no_ksi6_sl", &self.no_ksi6_sl())
            .field("no_ksi7_sl", &self.no_ksi7_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - No KSI0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi0_sl(&mut self) -> NoKsi0SlW<Devalt7Spec> {
        NoKsi0SlW::new(self, 0)
    }
    #[doc = "Bit 1 - No KSI1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi1_sl(&mut self) -> NoKsi1SlW<Devalt7Spec> {
        NoKsi1SlW::new(self, 1)
    }
    #[doc = "Bit 2 - No KSI2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi2_sl(&mut self) -> NoKsi2SlW<Devalt7Spec> {
        NoKsi2SlW::new(self, 2)
    }
    #[doc = "Bit 3 - No KSI3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi3_sl(&mut self) -> NoKsi3SlW<Devalt7Spec> {
        NoKsi3SlW::new(self, 3)
    }
    #[doc = "Bit 4 - No KSI4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi4_sl(&mut self) -> NoKsi4SlW<Devalt7Spec> {
        NoKsi4SlW::new(self, 4)
    }
    #[doc = "Bit 5 - No KSI5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi5_sl(&mut self) -> NoKsi5SlW<Devalt7Spec> {
        NoKsi5SlW::new(self, 5)
    }
    #[doc = "Bit 6 - No KSI6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi6_sl(&mut self) -> NoKsi6SlW<Devalt7Spec> {
        NoKsi6SlW::new(self, 6)
    }
    #[doc = "Bit 7 - No KSI7 Select"]
    #[inline(always)]
    #[must_use]
    pub fn no_ksi7_sl(&mut self) -> NoKsi7SlW<Devalt7Spec> {
        NoKsi7SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 7 Register (DEVALT7)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt7Spec;
impl crate::RegisterSpec for Devalt7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt7::R`](R) reader structure"]
impl crate::Readable for Devalt7Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt7::W`](W) writer structure"]
impl crate::Writable for Devalt7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT7 to value 0"]
impl crate::Resettable for Devalt7Spec {
    const RESET_VALUE: u8 = 0;
}
