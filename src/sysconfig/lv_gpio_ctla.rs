#[doc = "Register `LV_GPIO_CTLA` reader"]
pub type R = crate::R<LvGpioCtlaSpec>;
#[doc = "Register `LV_GPIO_CTLA` writer"]
pub type W = crate::W<LvGpioCtlaSpec>;
#[doc = "Field `GB1_LV` reader - GPIOB1/CR_SIN4 Low-Voltage Select"]
pub type Gb1LvR = crate::BitReader;
#[doc = "Field `GB1_LV` writer - GPIOB1/CR_SIN4 Low-Voltage Select"]
pub type Gb1LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GB6_LV` reader - GPIOB6 Low-Voltage Select"]
pub type Gb6LvR = crate::BitReader;
#[doc = "Field `GB6_LV` writer - GPIOB6 Low-Voltage Select"]
pub type Gb6LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GB7_LV` reader - GPIOB7/I2C7_SDA1 Low-Voltage Select"]
pub type Gb7LvR = crate::BitReader;
#[doc = "Field `GB7_LV` writer - GPIOB7/I2C7_SDA1 Low-Voltage Select"]
pub type Gb7LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC0_LV` reader - GPIOC0/I2C7_SCL1 Low-Voltage Select"]
pub type Gc0LvR = crate::BitReader;
#[doc = "Field `GC0_LV` writer - GPIOC0/I2C7_SCL1 Low-Voltage Select"]
pub type Gc0LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC3_LV` reader - GPIOC3 Low-Voltage Select"]
pub type Gc3LvR = crate::BitReader;
#[doc = "Field `GC3_LV` writer - GPIOC3 Low-Voltage Select"]
pub type Gc3LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC4_LV` reader - GPIOC4 Low-Voltage Select"]
pub type Gc4LvR = crate::BitReader;
#[doc = "Field `GC4_LV` writer - GPIOC4 Low-Voltage Select"]
pub type Gc4LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GD2_LV` reader - GPIOD2 Low-Voltage Select"]
pub type Gd2LvR = crate::BitReader;
#[doc = "Field `GD2_LV` writer - GPIOD2 Low-Voltage Select"]
pub type Gd2LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GD3_LV` reader - GPIOD3/TB1_TACH2PWM_IN Low-Voltage Select"]
pub type Gd3LvR = crate::BitReader;
#[doc = "Field `GD3_LV` writer - GPIOD3/TB1_TACH2PWM_IN Low-Voltage Select"]
pub type Gd3LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOB1/CR_SIN4 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb1_lv(&self) -> Gb1LvR {
        Gb1LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB6 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb6_lv(&self) -> Gb6LvR {
        Gb6LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOB7/I2C7_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb7_lv(&self) -> Gb7LvR {
        Gb7LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOC0/I2C7_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc0_lv(&self) -> Gc0LvR {
        Gc0LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOC3 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc3_lv(&self) -> Gc3LvR {
        Gc3LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOC4 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc4_lv(&self) -> Gc4LvR {
        Gc4LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOD2 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd2_lv(&self) -> Gd2LvR {
        Gd2LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOD3/TB1_TACH2PWM_IN Low-Voltage Select"]
    #[inline(always)]
    pub fn gd3_lv(&self) -> Gd3LvR {
        Gd3LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTLA")
            .field("gb1_lv", &self.gb1_lv())
            .field("gb6_lv", &self.gb6_lv())
            .field("gb7_lv", &self.gb7_lv())
            .field("gc0_lv", &self.gc0_lv())
            .field("gc3_lv", &self.gc3_lv())
            .field("gc4_lv", &self.gc4_lv())
            .field("gd2_lv", &self.gd2_lv())
            .field("gd3_lv", &self.gd3_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIOB1/CR_SIN4 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb1_lv(&mut self) -> Gb1LvW<LvGpioCtlaSpec> {
        Gb1LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB6 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb6_lv(&mut self) -> Gb6LvW<LvGpioCtlaSpec> {
        Gb6LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOB7/I2C7_SDA1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb7_lv(&mut self) -> Gb7LvW<LvGpioCtlaSpec> {
        Gb7LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOC0/I2C7_SCL1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc0_lv(&mut self) -> Gc0LvW<LvGpioCtlaSpec> {
        Gc0LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOC3 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc3_lv(&mut self) -> Gc3LvW<LvGpioCtlaSpec> {
        Gc3LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOC4 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc4_lv(&mut self) -> Gc4LvW<LvGpioCtlaSpec> {
        Gc4LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOD2 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd2_lv(&mut self) -> Gd2LvW<LvGpioCtlaSpec> {
        Gd2LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOD3/TB1_TACH2PWM_IN Low-Voltage Select"]
    #[inline(always)]
    pub fn gd3_lv(&mut self) -> Gd3LvW<LvGpioCtlaSpec> {
        Gd3LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control A Register (LV_GPIO_CTLA)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtlaSpec;
impl crate::RegisterSpec for LvGpioCtlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctla::R`](R) reader structure"]
impl crate::Readable for LvGpioCtlaSpec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctla::W`](W) writer structure"]
impl crate::Writable for LvGpioCtlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTLA to value 0"]
impl crate::Resettable for LvGpioCtlaSpec {
    const RESET_VALUE: u8 = 0;
}
