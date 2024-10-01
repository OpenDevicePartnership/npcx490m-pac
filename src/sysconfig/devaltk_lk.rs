#[doc = "Register `DEVALTK_LK` reader"]
pub type R = crate::R<DevaltkLkSpec>;
#[doc = "Register `DEVALTK_LK` writer"]
pub type W = crate::W<DevaltkLkSpec>;
#[doc = "Field `I2C7_1_SL_LK` reader - SMBus/I2C Module 7, Bus 1 Select Lock"]
pub type I2c7_1SlLkR = crate::BitReader;
#[doc = "Field `I2C7_1_SL_LK` writer - SMBus/I2C Module 7, Bus 1 Select Lock"]
pub type I2c7_1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - SMBus/I2C Module 7, Bus 1 Select Lock"]
    #[inline(always)]
    pub fn i2c7_1_sl_lk(&self) -> I2c7_1SlLkR {
        I2c7_1SlLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTK_LK")
            .field("i2c7_1_sl_lk", &self.i2c7_1_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - SMBus/I2C Module 7, Bus 1 Select Lock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c7_1_sl_lk(&mut self) -> I2c7_1SlLkW<DevaltkLkSpec> {
        I2c7_1SlLkW::new(self, 7)
    }
}
#[doc = "Device Alternate Function K Lock Register (DEVALTK_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltk_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltk_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltkLkSpec;
impl crate::RegisterSpec for DevaltkLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltk_lk::R`](R) reader structure"]
impl crate::Readable for DevaltkLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltk_lk::W`](W) writer structure"]
impl crate::Writable for DevaltkLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTK_LK to value 0"]
impl crate::Resettable for DevaltkLkSpec {
    const RESET_VALUE: u8 = 0;
}
