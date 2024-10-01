#[doc = "Register `DEVALT2` reader"]
pub type R = crate::R<Devalt2Spec>;
#[doc = "Register `DEVALT2` writer"]
pub type W = crate::W<Devalt2Spec>;
#[doc = "Field `I2C0_0_SL` reader - SMBus/I2C Module 0, Bus 0 Select"]
pub type I2c0_0SlR = crate::BitReader;
#[doc = "Field `I2C0_0_SL` writer - SMBus/I2C Module 0, Bus 0 Select"]
pub type I2c0_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C7_0_SL` reader - SMBus/I2C Module 7, Bus 0 Select"]
pub type I2c7_0SlR = crate::BitReader;
#[doc = "Field `I2C7_0_SL` writer - SMBus/I2C Module 7, Bus 0 Select"]
pub type I2c7_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_0_SL` reader - SMBus/I2C Module 1, Bus 0 Select"]
pub type I2c1_0SlR = crate::BitReader;
#[doc = "Field `I2C1_0_SL` writer - SMBus/I2C Module 1, Bus 0 Select"]
pub type I2c1_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6_0_SL` reader - SMBus/I2C Module 6, Bus 0 Select"]
pub type I2c6_0SlR = crate::BitReader;
#[doc = "Field `I2C6_0_SL` writer - SMBus/I2C Module 6, Bus 0 Select"]
pub type I2c6_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_0_SL` reader - SMBus/I2C Module 2, Bus 0 Select"]
pub type I2c2_0SlR = crate::BitReader;
#[doc = "Field `I2C2_0_SL` writer - SMBus/I2C Module 2, Bus 0 Select"]
pub type I2c2_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C5_0_SL` reader - SMBus/I2C Module 5, Bus 0 Select"]
pub type I2c5_0SlR = crate::BitReader;
#[doc = "Field `I2C5_0_SL` writer - SMBus/I2C Module 5, Bus 0 Select"]
pub type I2c5_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_0_SL` reader - SMBus/I2C Module 3, Bus 0 Select"]
pub type I2c3_0SlR = crate::BitReader;
#[doc = "Field `I2C3_0_SL` writer - SMBus/I2C Module 3, Bus 0 Select"]
pub type I2c3_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_0_SL` reader - SMBus/I2C Module 4, Bus 0 Select"]
pub type I2c4_0SlR = crate::BitReader;
#[doc = "Field `I2C4_0_SL` writer - SMBus/I2C Module 4, Bus 0 Select"]
pub type I2c4_0SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMBus/I2C Module 0, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c0_0_sl(&self) -> I2c0_0SlR {
        I2c0_0SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 7, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c7_0_sl(&self) -> I2c7_0SlR {
        I2c7_0SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 1, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c1_0_sl(&self) -> I2c1_0SlR {
        I2c1_0SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 6, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c6_0_sl(&self) -> I2c6_0SlR {
        I2c6_0SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 2, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c2_0_sl(&self) -> I2c2_0SlR {
        I2c2_0SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c5_0_sl(&self) -> I2c5_0SlR {
        I2c5_0SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 3, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c3_0_sl(&self) -> I2c3_0SlR {
        I2c3_0SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 0 Select"]
    #[inline(always)]
    pub fn i2c4_0_sl(&self) -> I2c4_0SlR {
        I2c4_0SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT2")
            .field("i2c0_0_sl", &self.i2c0_0_sl())
            .field("i2c7_0_sl", &self.i2c7_0_sl())
            .field("i2c1_0_sl", &self.i2c1_0_sl())
            .field("i2c6_0_sl", &self.i2c6_0_sl())
            .field("i2c2_0_sl", &self.i2c2_0_sl())
            .field("i2c5_0_sl", &self.i2c5_0_sl())
            .field("i2c3_0_sl", &self.i2c3_0_sl())
            .field("i2c4_0_sl", &self.i2c4_0_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMBus/I2C Module 0, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_0_sl(&mut self) -> I2c0_0SlW<Devalt2Spec> {
        I2c0_0SlW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 7, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c7_0_sl(&mut self) -> I2c7_0SlW<Devalt2Spec> {
        I2c7_0SlW::new(self, 1)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 1, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_0_sl(&mut self) -> I2c1_0SlW<Devalt2Spec> {
        I2c1_0SlW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 6, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6_0_sl(&mut self) -> I2c6_0SlW<Devalt2Spec> {
        I2c6_0SlW::new(self, 3)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 2, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_0_sl(&mut self) -> I2c2_0SlW<Devalt2Spec> {
        I2c2_0SlW::new(self, 4)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5_0_sl(&mut self) -> I2c5_0SlW<Devalt2Spec> {
        I2c5_0SlW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 3, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_0_sl(&mut self) -> I2c3_0SlW<Devalt2Spec> {
        I2c3_0SlW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_0_sl(&mut self) -> I2c4_0SlW<Devalt2Spec> {
        I2c4_0SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 2 Register (DEVALT2)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt2Spec;
impl crate::RegisterSpec for Devalt2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt2::R`](R) reader structure"]
impl crate::Readable for Devalt2Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt2::W`](W) writer structure"]
impl crate::Writable for Devalt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT2 to value 0"]
impl crate::Resettable for Devalt2Spec {
    const RESET_VALUE: u8 = 0;
}
