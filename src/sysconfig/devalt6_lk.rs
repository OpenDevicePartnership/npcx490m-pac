#[doc = "Register `DEVALT6_LK` reader"]
pub type R = crate::R<Devalt6LkSpec>;
#[doc = "Register `DEVALT6_LK` writer"]
pub type W = crate::W<Devalt6LkSpec>;
#[doc = "Field `AD0_SL_LK` reader - AD0 Select Lock"]
pub type Ad0SlLkR = crate::BitReader;
#[doc = "Field `AD0_SL_LK` writer - AD0 Select Lock"]
pub type Ad0SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6_1_SL_LK` reader - SMBus/I2C Module 6, Bus 1 Select Lock"]
pub type I2c6_1SlLkR = crate::BitReader;
#[doc = "Field `I2C6_1_SL_LK` writer - SMBus/I2C Module 6, Bus 1 Select Lock"]
pub type I2c6_1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C5_1_SL_LK` reader - SMBus/I2C Module 5, Bus 1 Select Lock"]
pub type I2c5_1SlLkR = crate::BitReader;
#[doc = "Field `I2C5_1_SL_LK` writer - SMBus/I2C Module 5, Bus 1 Select Lock"]
pub type I2c5_1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_1_SL_LK` reader - SMBus/I2C Module 4, Bus 1 Select Lock"]
pub type I2c4_1SlLkR = crate::BitReader;
#[doc = "Field `I2C4_1_SL_LK` writer - SMBus/I2C Module 4, Bus 1 Select Lock"]
pub type I2c4_1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AD0 Select Lock"]
    #[inline(always)]
    pub fn ad0_sl_lk(&self) -> Ad0SlLkR {
        Ad0SlLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 6, Bus 1 Select Lock"]
    #[inline(always)]
    pub fn i2c6_1_sl_lk(&self) -> I2c6_1SlLkR {
        I2c6_1SlLkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 5, Bus 1 Select Lock"]
    #[inline(always)]
    pub fn i2c5_1_sl_lk(&self) -> I2c5_1SlLkR {
        I2c5_1SlLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 1 Select Lock"]
    #[inline(always)]
    pub fn i2c4_1_sl_lk(&self) -> I2c4_1SlLkR {
        I2c4_1SlLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT6_LK")
            .field("ad0_sl_lk", &self.ad0_sl_lk())
            .field("i2c6_1_sl_lk", &self.i2c6_1_sl_lk())
            .field("i2c5_1_sl_lk", &self.i2c5_1_sl_lk())
            .field("i2c4_1_sl_lk", &self.i2c4_1_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AD0 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn ad0_sl_lk(&mut self) -> Ad0SlLkW<Devalt6LkSpec> {
        Ad0SlLkW::new(self, 0)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 6, Bus 1 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6_1_sl_lk(&mut self) -> I2c6_1SlLkW<Devalt6LkSpec> {
        I2c6_1SlLkW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 5, Bus 1 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5_1_sl_lk(&mut self) -> I2c5_1SlLkW<Devalt6LkSpec> {
        I2c5_1SlLkW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 1 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_1_sl_lk(&mut self) -> I2c4_1SlLkW<Devalt6LkSpec> {
        I2c4_1SlLkW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 6 Lock Register (DEVALT6_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt6_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt6_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt6LkSpec;
impl crate::RegisterSpec for Devalt6LkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt6_lk::R`](R) reader structure"]
impl crate::Readable for Devalt6LkSpec {}
#[doc = "`write(|w| ..)` method takes [`devalt6_lk::W`](W) writer structure"]
impl crate::Writable for Devalt6LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT6_LK to value 0"]
impl crate::Resettable for Devalt6LkSpec {
    const RESET_VALUE: u8 = 0;
}
