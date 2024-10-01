#[doc = "Register `DEVALT2_LK` reader"]
pub type R = crate::R<Devalt2LkSpec>;
#[doc = "Register `DEVALT2_LK` writer"]
pub type W = crate::W<Devalt2LkSpec>;
#[doc = "Field `I2C0_0_SL_LK` reader - SMBus/I2C Module 0, Bus 0 Select Lock"]
pub type I2c0_0SlLkR = crate::BitReader;
#[doc = "Field `I2C0_0_SL_LK` writer - SMBus/I2C Module 0, Bus 0 Select Lock"]
pub type I2c0_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C7_0_SL_LK` reader - SMBus/I2C Module 7, Bus 0 Select Lock"]
pub type I2c7_0SlLkR = crate::BitReader;
#[doc = "Field `I2C7_0_SL_LK` writer - SMBus/I2C Module 7, Bus 0 Select Lock"]
pub type I2c7_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_0_SL_LK` reader - SMBus/I2C Module 1, Bus 0 Select Lock"]
pub type I2c1_0SlLkR = crate::BitReader;
#[doc = "Field `I2C1_0_SL_LK` writer - SMBus/I2C Module 1, Bus 0 Select Lock"]
pub type I2c1_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6_0_SL_LK` reader - SMBus/I2C Module 6, Bus 0 Select Lock"]
pub type I2c6_0SlLkR = crate::BitReader;
#[doc = "Field `I2C6_0_SL_LK` writer - SMBus/I2C Module 6, Bus 0 Select Lock"]
pub type I2c6_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_0_SL_LK` reader - SMBus/I2C Module 2, Bus 0 Select Lock"]
pub type I2c2_0SlLkR = crate::BitReader;
#[doc = "Field `I2C2_0_SL_LK` writer - SMBus/I2C Module 2, Bus 0 Select Lock"]
pub type I2c2_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C5_0_SL_LK` reader - SMBus/I2C Module 5, Bus 0 Select Lock"]
pub type I2c5_0SlLkR = crate::BitReader;
#[doc = "Field `I2C5_0_SL_LK` writer - SMBus/I2C Module 5, Bus 0 Select Lock"]
pub type I2c5_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_0_SL_LK` reader - SMBus/I2C Module 3, Bus 0 Select Lock"]
pub type I2c3_0SlLkR = crate::BitReader;
#[doc = "Field `I2C3_0_SL_LK` writer - SMBus/I2C Module 3, Bus 0 Select Lock"]
pub type I2c3_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_0_SL_LK` reader - SMBus/I2C Module 4, Bus 0 Select Lock"]
pub type I2c4_0SlLkR = crate::BitReader;
#[doc = "Field `I2C4_0_SL_LK` writer - SMBus/I2C Module 4, Bus 0 Select Lock"]
pub type I2c4_0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMBus/I2C Module 0, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c0_0_sl_lk(&self) -> I2c0_0SlLkR {
        I2c0_0SlLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 7, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c7_0_sl_lk(&self) -> I2c7_0SlLkR {
        I2c7_0SlLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 1, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c1_0_sl_lk(&self) -> I2c1_0SlLkR {
        I2c1_0SlLkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 6, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c6_0_sl_lk(&self) -> I2c6_0SlLkR {
        I2c6_0SlLkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 2, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c2_0_sl_lk(&self) -> I2c2_0SlLkR {
        I2c2_0SlLkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c5_0_sl_lk(&self) -> I2c5_0SlLkR {
        I2c5_0SlLkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 3, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c3_0_sl_lk(&self) -> I2c3_0SlLkR {
        I2c3_0SlLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 0 Select Lock"]
    #[inline(always)]
    pub fn i2c4_0_sl_lk(&self) -> I2c4_0SlLkR {
        I2c4_0SlLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT2_LK")
            .field("i2c0_0_sl_lk", &self.i2c0_0_sl_lk())
            .field("i2c7_0_sl_lk", &self.i2c7_0_sl_lk())
            .field("i2c1_0_sl_lk", &self.i2c1_0_sl_lk())
            .field("i2c6_0_sl_lk", &self.i2c6_0_sl_lk())
            .field("i2c2_0_sl_lk", &self.i2c2_0_sl_lk())
            .field("i2c5_0_sl_lk", &self.i2c5_0_sl_lk())
            .field("i2c3_0_sl_lk", &self.i2c3_0_sl_lk())
            .field("i2c4_0_sl_lk", &self.i2c4_0_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMBus/I2C Module 0, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_0_sl_lk(&mut self) -> I2c0_0SlLkW<Devalt2LkSpec> {
        I2c0_0SlLkW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 7, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c7_0_sl_lk(&mut self) -> I2c7_0SlLkW<Devalt2LkSpec> {
        I2c7_0SlLkW::new(self, 1)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 1, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_0_sl_lk(&mut self) -> I2c1_0SlLkW<Devalt2LkSpec> {
        I2c1_0SlLkW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 6, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6_0_sl_lk(&mut self) -> I2c6_0SlLkW<Devalt2LkSpec> {
        I2c6_0SlLkW::new(self, 3)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 2, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_0_sl_lk(&mut self) -> I2c2_0SlLkW<Devalt2LkSpec> {
        I2c2_0SlLkW::new(self, 4)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5_0_sl_lk(&mut self) -> I2c5_0SlLkW<Devalt2LkSpec> {
        I2c5_0SlLkW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 3, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_0_sl_lk(&mut self) -> I2c3_0SlLkW<Devalt2LkSpec> {
        I2c3_0SlLkW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_0_sl_lk(&mut self) -> I2c4_0SlLkW<Devalt2LkSpec> {
        I2c4_0SlLkW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 2 Lock Register (DEVALT2_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt2_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt2_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt2LkSpec;
impl crate::RegisterSpec for Devalt2LkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt2_lk::R`](R) reader structure"]
impl crate::Readable for Devalt2LkSpec {}
#[doc = "`write(|w| ..)` method takes [`devalt2_lk::W`](W) writer structure"]
impl crate::Writable for Devalt2LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT2_LK to value 0"]
impl crate::Resettable for Devalt2LkSpec {
    const RESET_VALUE: u8 = 0;
}
