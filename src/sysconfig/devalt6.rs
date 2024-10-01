#[doc = "Register `DEVALT6` reader"]
pub type R = crate::R<Devalt6Spec>;
#[doc = "Register `DEVALT6` writer"]
pub type W = crate::W<Devalt6Spec>;
#[doc = "Field `AD0_SL` reader - AD0 Select"]
pub type Ad0SlR = crate::BitReader;
#[doc = "Field `AD0_SL` writer - AD0 Select"]
pub type Ad0SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD1_SL` reader - AD1 Select"]
pub type Ad1SlR = crate::BitReader;
#[doc = "Field `AD1_SL` writer - AD1 Select"]
pub type Ad1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD2_SL` reader - AD2 Select"]
pub type Ad2SlR = crate::BitReader;
#[doc = "Field `AD2_SL` writer - AD2 Select"]
pub type Ad2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD3_SL` reader - AD3 Select"]
pub type Ad3SlR = crate::BitReader;
#[doc = "Field `AD3_SL` writer - AD3 Select"]
pub type Ad3SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD4_SL` reader - AD4 Select"]
pub type Ad4SlR = crate::BitReader;
#[doc = "Field `AD4_SL` writer - AD4 Select"]
pub type Ad4SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6_1_SL` reader - SMBus/I2C Module 6, Bus 1 Select"]
pub type I2c6_1SlR = crate::BitReader;
#[doc = "Field `I2C6_1_SL` writer - SMBus/I2C Module 6, Bus 1 Select"]
pub type I2c6_1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C5_1_SL` reader - SMBus/I2C Module 5, Bus 1 Select"]
pub type I2c5_1SlR = crate::BitReader;
#[doc = "Field `I2C5_1_SL` writer - SMBus/I2C Module 5, Bus 1 Select"]
pub type I2c5_1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_1_SL` reader - SMBus/I2C Module 4, Bus 1 Select"]
pub type I2c4_1SlR = crate::BitReader;
#[doc = "Field `I2C4_1_SL` writer - SMBus/I2C Module 4, Bus 1 Select"]
pub type I2c4_1SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AD0 Select"]
    #[inline(always)]
    pub fn ad0_sl(&self) -> Ad0SlR {
        Ad0SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1 Select"]
    #[inline(always)]
    pub fn ad1_sl(&self) -> Ad1SlR {
        Ad1SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD2 Select"]
    #[inline(always)]
    pub fn ad2_sl(&self) -> Ad2SlR {
        Ad2SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD3 Select"]
    #[inline(always)]
    pub fn ad3_sl(&self) -> Ad3SlR {
        Ad3SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD4 Select"]
    #[inline(always)]
    pub fn ad4_sl(&self) -> Ad4SlR {
        Ad4SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 6, Bus 1 Select"]
    #[inline(always)]
    pub fn i2c6_1_sl(&self) -> I2c6_1SlR {
        I2c6_1SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 5, Bus 1 Select"]
    #[inline(always)]
    pub fn i2c5_1_sl(&self) -> I2c5_1SlR {
        I2c5_1SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 1 Select"]
    #[inline(always)]
    pub fn i2c4_1_sl(&self) -> I2c4_1SlR {
        I2c4_1SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALT6")
            .field("ad0_sl", &self.ad0_sl())
            .field("ad1_sl", &self.ad1_sl())
            .field("ad2_sl", &self.ad2_sl())
            .field("ad3_sl", &self.ad3_sl())
            .field("ad4_sl", &self.ad4_sl())
            .field("i2c6_1_sl", &self.i2c6_1_sl())
            .field("i2c5_1_sl", &self.i2c5_1_sl())
            .field("i2c4_1_sl", &self.i2c4_1_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AD0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad0_sl(&mut self) -> Ad0SlW<Devalt6Spec> {
        Ad0SlW::new(self, 0)
    }
    #[doc = "Bit 1 - AD1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad1_sl(&mut self) -> Ad1SlW<Devalt6Spec> {
        Ad1SlW::new(self, 1)
    }
    #[doc = "Bit 2 - AD2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad2_sl(&mut self) -> Ad2SlW<Devalt6Spec> {
        Ad2SlW::new(self, 2)
    }
    #[doc = "Bit 3 - AD3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad3_sl(&mut self) -> Ad3SlW<Devalt6Spec> {
        Ad3SlW::new(self, 3)
    }
    #[doc = "Bit 4 - AD4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad4_sl(&mut self) -> Ad4SlW<Devalt6Spec> {
        Ad4SlW::new(self, 4)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 6, Bus 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6_1_sl(&mut self) -> I2c6_1SlW<Devalt6Spec> {
        I2c6_1SlW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 5, Bus 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5_1_sl(&mut self) -> I2c5_1SlW<Devalt6Spec> {
        I2c5_1SlW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_1_sl(&mut self) -> I2c4_1SlW<Devalt6Spec> {
        I2c4_1SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function 6 Register (DEVALT6)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalt6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalt6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devalt6Spec;
impl crate::RegisterSpec for Devalt6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalt6::R`](R) reader structure"]
impl crate::Readable for Devalt6Spec {}
#[doc = "`write(|w| ..)` method takes [`devalt6::W`](W) writer structure"]
impl crate::Writable for Devalt6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALT6 to value 0"]
impl crate::Resettable for Devalt6Spec {
    const RESET_VALUE: u8 = 0;
}
