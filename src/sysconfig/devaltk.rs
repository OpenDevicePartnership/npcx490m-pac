#[doc = "Register `DEVALTK` reader"]
pub type R = crate::R<DevaltkSpec>;
#[doc = "Register `DEVALTK` writer"]
pub type W = crate::W<DevaltkSpec>;
#[doc = "Field `I2C7_1_SL` reader - SMBus/I2C Module 7, Bus 1 Select"]
pub type I2c7_1SlR = crate::BitReader;
#[doc = "Field `I2C7_1_SL` writer - SMBus/I2C Module 7, Bus 1 Select"]
pub type I2c7_1SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - SMBus/I2C Module 7, Bus 1 Select"]
    #[inline(always)]
    pub fn i2c7_1_sl(&self) -> I2c7_1SlR {
        I2c7_1SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTK")
            .field("i2c7_1_sl", &self.i2c7_1_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - SMBus/I2C Module 7, Bus 1 Select"]
    #[inline(always)]
    pub fn i2c7_1_sl(&mut self) -> I2c7_1SlW<DevaltkSpec> {
        I2c7_1SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function K Register (DEVALTK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltkSpec;
impl crate::RegisterSpec for DevaltkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltk::R`](R) reader structure"]
impl crate::Readable for DevaltkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltk::W`](W) writer structure"]
impl crate::Writable for DevaltkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTK to value 0"]
impl crate::Resettable for DevaltkSpec {
    const RESET_VALUE: u8 = 0;
}
