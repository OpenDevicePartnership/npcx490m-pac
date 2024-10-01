#[doc = "Register `PUPD_EN0` reader"]
pub type R = crate::R<PupdEn0Spec>;
#[doc = "Register `PUPD_EN0` writer"]
pub type W = crate::W<PupdEn0Spec>;
#[doc = "Field `I2C0_0_PUE` reader - SMBus/I2C Module 0, Bus 0 Pull-Up Enable"]
pub type I2c0_0PueR = crate::BitReader;
#[doc = "Field `I2C0_0_PUE` writer - SMBus/I2C Module 0, Bus 0 Pull-Up Enable"]
pub type I2c0_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C7_0_PUE` reader - SMBus/I2C Module 7, Bus 0 Pull-Up Enable"]
pub type I2c7_0PueR = crate::BitReader;
#[doc = "Field `I2C7_0_PUE` writer - SMBus/I2C Module 7, Bus 0 Pull-Up Enable"]
pub type I2c7_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_0_PUE` reader - SMBus/I2C Module 1, Bus 0 Pull-Up Enable"]
pub type I2c1_0PueR = crate::BitReader;
#[doc = "Field `I2C1_0_PUE` writer - SMBus/I2C Module 1, Bus 0 Pull-Up Enable"]
pub type I2c1_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6_0_PUE` reader - SMBus/I2C Module 6, Bus 0 Pull-Up Enable"]
pub type I2c6_0PueR = crate::BitReader;
#[doc = "Field `I2C6_0_PUE` writer - SMBus/I2C Module 6, Bus 0 Pull-Up Enable"]
pub type I2c6_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_0_PUE` reader - SMBus/I2C Module 2, Bus 0 Pull-Up Enable"]
pub type I2c2_0PueR = crate::BitReader;
#[doc = "Field `I2C2_0_PUE` writer - SMBus/I2C Module 2, Bus 0 Pull-Up Enable"]
pub type I2c2_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C5_0_PUE` reader - SMBus/I2C Module 5, Bus 0 Pull-Up Enable"]
pub type I2c5_0PueR = crate::BitReader;
#[doc = "Field `I2C5_0_PUE` writer - SMBus/I2C Module 5, Bus 0 Pull-Up Enable"]
pub type I2c5_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_0_PUE` reader - SMBus/I2C Module 3, Bus 0 Pull-Up Enable"]
pub type I2c3_0PueR = crate::BitReader;
#[doc = "Field `I2C3_0_PUE` writer - SMBus/I2C Module 3, Bus 0 Pull-Up Enable"]
pub type I2c3_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_0_PUE` reader - SMBus/I2C Module 4, Bus 0 Pull-Up Enable"]
pub type I2c4_0PueR = crate::BitReader;
#[doc = "Field `I2C4_0_PUE` writer - SMBus/I2C Module 4, Bus 0 Pull-Up Enable"]
pub type I2c4_0PueW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMBus/I2C Module 0, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c0_0_pue(&self) -> I2c0_0PueR {
        I2c0_0PueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 7, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c7_0_pue(&self) -> I2c7_0PueR {
        I2c7_0PueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 1, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c1_0_pue(&self) -> I2c1_0PueR {
        I2c1_0PueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 6, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c6_0_pue(&self) -> I2c6_0PueR {
        I2c6_0PueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 2, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c2_0_pue(&self) -> I2c2_0PueR {
        I2c2_0PueR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c5_0_pue(&self) -> I2c5_0PueR {
        I2c5_0PueR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 3, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c3_0_pue(&self) -> I2c3_0PueR {
        I2c3_0PueR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    pub fn i2c4_0_pue(&self) -> I2c4_0PueR {
        I2c4_0PueR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPD_EN0")
            .field("i2c0_0_pue", &self.i2c0_0_pue())
            .field("i2c7_0_pue", &self.i2c7_0_pue())
            .field("i2c1_0_pue", &self.i2c1_0_pue())
            .field("i2c6_0_pue", &self.i2c6_0_pue())
            .field("i2c2_0_pue", &self.i2c2_0_pue())
            .field("i2c5_0_pue", &self.i2c5_0_pue())
            .field("i2c3_0_pue", &self.i2c3_0_pue())
            .field("i2c4_0_pue", &self.i2c4_0_pue())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMBus/I2C Module 0, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_0_pue(&mut self) -> I2c0_0PueW<PupdEn0Spec> {
        I2c0_0PueW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus/I2C Module 7, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c7_0_pue(&mut self) -> I2c7_0PueW<PupdEn0Spec> {
        I2c7_0PueW::new(self, 1)
    }
    #[doc = "Bit 2 - SMBus/I2C Module 1, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_0_pue(&mut self) -> I2c1_0PueW<PupdEn0Spec> {
        I2c1_0PueW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus/I2C Module 6, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6_0_pue(&mut self) -> I2c6_0PueW<PupdEn0Spec> {
        I2c6_0PueW::new(self, 3)
    }
    #[doc = "Bit 4 - SMBus/I2C Module 2, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_0_pue(&mut self) -> I2c2_0PueW<PupdEn0Spec> {
        I2c2_0PueW::new(self, 4)
    }
    #[doc = "Bit 5 - SMBus/I2C Module 5, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5_0_pue(&mut self) -> I2c5_0PueW<PupdEn0Spec> {
        I2c5_0PueW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus/I2C Module 3, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_0_pue(&mut self) -> I2c3_0PueW<PupdEn0Spec> {
        I2c3_0PueW::new(self, 6)
    }
    #[doc = "Bit 7 - SMBus/I2C Module 4, Bus 0 Pull-Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_0_pue(&mut self) -> I2c4_0PueW<PupdEn0Spec> {
        I2c4_0PueW::new(self, 7)
    }
}
#[doc = "Pull-Up/Pull-Down Enable 0 Register (PUPD_EN0)\n\nYou can [`read`](crate::Reg::read) this register and get [`pupd_en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupd_en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PupdEn0Spec;
impl crate::RegisterSpec for PupdEn0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pupd_en0::R`](R) reader structure"]
impl crate::Readable for PupdEn0Spec {}
#[doc = "`write(|w| ..)` method takes [`pupd_en0::W`](W) writer structure"]
impl crate::Writable for PupdEn0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PUPD_EN0 to value 0"]
impl crate::Resettable for PupdEn0Spec {
    const RESET_VALUE: u8 = 0;
}
