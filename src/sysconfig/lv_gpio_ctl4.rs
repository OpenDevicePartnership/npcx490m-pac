#[doc = "Register `LV_GPIO_CTL4` reader"]
pub type R = crate::R<LvGpioCtl4Spec>;
#[doc = "Register `LV_GPIO_CTL4` writer"]
pub type W = crate::W<LvGpioCtl4Spec>;
#[doc = "Field `G86_LV` reader - GPIO86/CR_SOUT2/I2C4_SDA0 Low-Voltage Select"]
pub type G86LvR = crate::BitReader;
#[doc = "Field `G86_LV` writer - GPIO86/CR_SOUT2/I2C4_SDA0 Low-Voltage Select"]
pub type G86LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC2_LV` reader - GPIOC2/I2C6_SCL0 Low-Voltage Select"]
pub type Gc2LvR = crate::BitReader;
#[doc = "Field `GC2_LV` writer - GPIOC2/I2C6_SCL0 Low-Voltage Select"]
pub type Gc2LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GF3_LV` reader - GPIOF3/I2C4_SCL1 Low-Voltage Select"]
pub type Gf3LvR = crate::BitReader;
#[doc = "Field `GF3_LV` writer - GPIOF3/I2C4_SCL1 Low-Voltage Select"]
pub type Gf3LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GF2_LV` reader - GPIOF2/I2C4_SDA1 Low-Voltage Select"]
pub type Gf2LvR = crate::BitReader;
#[doc = "Field `GF2_LV` writer - GPIOF2/I2C4_SDA1 Low-Voltage Select"]
pub type Gf2LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GF5_LV` reader - GPIOF5/I2C5_SCL1 Low-Voltage Select"]
pub type Gf5LvR = crate::BitReader;
#[doc = "Field `GF5_LV` writer - GPIOF5/I2C5_SCL1 Low-Voltage Select"]
pub type Gf5LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GF4_LV` reader - GPIOF4/I2C5_SDA1 Low-Voltage Select"]
pub type Gf4LvR = crate::BitReader;
#[doc = "Field `GF4_LV` writer - GPIOF4/I2C5_SDA1 Low-Voltage Select"]
pub type Gf4LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE4_LV` reader - GPIOE4/I2C6_SCL1 Low-Voltage Select"]
pub type Ge4LvR = crate::BitReader;
#[doc = "Field `GE4_LV` writer - GPIOE4/I2C6_SCL1 Low-Voltage Select"]
pub type Ge4LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE3_LV` reader - GPIOE3/I2C6_SDA1 Low-Voltage Select"]
pub type Ge3LvR = crate::BitReader;
#[doc = "Field `GE3_LV` writer - GPIOE3/I2C6_SDA1 Low-Voltage Select"]
pub type Ge3LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO86/CR_SOUT2/I2C4_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g86_lv(&self) -> G86LvR {
        G86LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOC2/I2C6_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc2_lv(&self) -> Gc2LvR {
        Gc2LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOF3/I2C4_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf3_lv(&self) -> Gf3LvR {
        Gf3LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOF2/I2C4_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf2_lv(&self) -> Gf2LvR {
        Gf2LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOF5/I2C5_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf5_lv(&self) -> Gf5LvR {
        Gf5LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF4/I2C5_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf4_lv(&self) -> Gf4LvR {
        Gf4LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOE4/I2C6_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn ge4_lv(&self) -> Ge4LvR {
        Ge4LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOE3/I2C6_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn ge3_lv(&self) -> Ge3LvR {
        Ge3LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL4")
            .field("g86_lv", &self.g86_lv())
            .field("gc2_lv", &self.gc2_lv())
            .field("gf3_lv", &self.gf3_lv())
            .field("gf2_lv", &self.gf2_lv())
            .field("gf5_lv", &self.gf5_lv())
            .field("gf4_lv", &self.gf4_lv())
            .field("ge4_lv", &self.ge4_lv())
            .field("ge3_lv", &self.ge3_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIO86/CR_SOUT2/I2C4_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g86_lv(&mut self) -> G86LvW<LvGpioCtl4Spec> {
        G86LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOC2/I2C6_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc2_lv(&mut self) -> Gc2LvW<LvGpioCtl4Spec> {
        Gc2LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOF3/I2C4_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf3_lv(&mut self) -> Gf3LvW<LvGpioCtl4Spec> {
        Gf3LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOF2/I2C4_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf2_lv(&mut self) -> Gf2LvW<LvGpioCtl4Spec> {
        Gf2LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOF5/I2C5_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf5_lv(&mut self) -> Gf5LvW<LvGpioCtl4Spec> {
        Gf5LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF4/I2C5_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf4_lv(&mut self) -> Gf4LvW<LvGpioCtl4Spec> {
        Gf4LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOE4/I2C6_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn ge4_lv(&mut self) -> Ge4LvW<LvGpioCtl4Spec> {
        Ge4LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOE3/I2C6_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn ge3_lv(&mut self) -> Ge3LvW<LvGpioCtl4Spec> {
        Ge3LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 4 Register (LV_GPIO_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl4Spec;
impl crate::RegisterSpec for LvGpioCtl4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl4::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl4::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL4 to value 0"]
impl crate::Resettable for LvGpioCtl4Spec {
    const RESET_VALUE: u8 = 0;
}
