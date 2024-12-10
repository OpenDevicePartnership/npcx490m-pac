#[doc = "Register `LV_GPIO_CTL0` reader"]
pub type R = crate::R<LvGpioCtl0Spec>;
#[doc = "Register `LV_GPIO_CTL0` writer"]
pub type W = crate::W<LvGpioCtl0Spec>;
#[doc = "Field `GB5_LV` reader - GPIOB5/I2C0_SCL0 Low-Voltage Select"]
pub type Gb5LvR = crate::BitReader;
#[doc = "Field `GB5_LV` writer - GPIOB5/I2C0_SCL0 Low-Voltage Select"]
pub type Gb5LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GB4_LV` reader - GPIOB4/I2C0_SDA0 Low-Voltage Select"]
pub type Gb4LvR = crate::BitReader;
#[doc = "Field `GB4_LV` writer - GPIOB4/I2C0_SDA0 Low-Voltage Select"]
pub type Gb4LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GB3_LV` reader - GPIOB3/I2C7_SCL0 Low-Voltage Select"]
pub type Gb3LvR = crate::BitReader;
#[doc = "Field `GB3_LV` writer - GPIOB3/I2C7_SCL0 Low-Voltage Select"]
pub type Gb3LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GB2_LV` reader - GPIOB2/I2C7_SDA0 Low-Voltage Select"]
pub type Gb2LvR = crate::BitReader;
#[doc = "Field `GB2_LV` writer - GPIOB2/I2C7_SDA0 Low-Voltage Select"]
pub type Gb2LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G90_LV` reader - GPIO90/I2C1_SCL0 Low-Voltage Select"]
pub type G90LvR = crate::BitReader;
#[doc = "Field `G90_LV` writer - GPIO90/I2C1_SCL0 Low-Voltage Select"]
pub type G90LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G87_LV` reader - GPIO87/I2C1_SDA0 Low-Voltage Select"]
pub type G87LvR = crate::BitReader;
#[doc = "Field `G87_LV` writer - GPIO87/I2C1_SDA0 Low-Voltage Select"]
pub type G87LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G00_LV` reader - GPIO00 Low-Voltage Select"]
pub type G00LvR = crate::BitReader;
#[doc = "Field `G00_LV` writer - GPIO00 Low-Voltage Select"]
pub type G00LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G33_LV` reader - GPIO33/I2C5_SCL0 Low-Voltage Select"]
pub type G33LvR = crate::BitReader;
#[doc = "Field `G33_LV` writer - GPIO33/I2C5_SCL0 Low-Voltage Select"]
pub type G33LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOB5/I2C0_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb5_lv(&self) -> Gb5LvR {
        Gb5LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB4/I2C0_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb4_lv(&self) -> Gb4LvR {
        Gb4LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOB3/I2C7_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb3_lv(&self) -> Gb3LvR {
        Gb3LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOB2/I2C7_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb2_lv(&self) -> Gb2LvR {
        Gb2LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO90/I2C1_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g90_lv(&self) -> G90LvR {
        G90LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO87/I2C1_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g87_lv(&self) -> G87LvR {
        G87LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO00 Low-Voltage Select"]
    #[inline(always)]
    pub fn g00_lv(&self) -> G00LvR {
        G00LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO33/I2C5_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g33_lv(&self) -> G33LvR {
        G33LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL0")
            .field("gb5_lv", &self.gb5_lv())
            .field("gb4_lv", &self.gb4_lv())
            .field("gb3_lv", &self.gb3_lv())
            .field("gb2_lv", &self.gb2_lv())
            .field("g90_lv", &self.g90_lv())
            .field("g87_lv", &self.g87_lv())
            .field("g00_lv", &self.g00_lv())
            .field("g33_lv", &self.g33_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIOB5/I2C0_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb5_lv(&mut self) -> Gb5LvW<LvGpioCtl0Spec> {
        Gb5LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB4/I2C0_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb4_lv(&mut self) -> Gb4LvW<LvGpioCtl0Spec> {
        Gb4LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOB3/I2C7_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb3_lv(&mut self) -> Gb3LvW<LvGpioCtl0Spec> {
        Gb3LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOB2/I2C7_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gb2_lv(&mut self) -> Gb2LvW<LvGpioCtl0Spec> {
        Gb2LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO90/I2C1_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g90_lv(&mut self) -> G90LvW<LvGpioCtl0Spec> {
        G90LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO87/I2C1_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g87_lv(&mut self) -> G87LvW<LvGpioCtl0Spec> {
        G87LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO00 Low-Voltage Select"]
    #[inline(always)]
    pub fn g00_lv(&mut self) -> G00LvW<LvGpioCtl0Spec> {
        G00LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO33/I2C5_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g33_lv(&mut self) -> G33LvW<LvGpioCtl0Spec> {
        G33LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 0 Register (LV_GPIO_CTL0)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl0Spec;
impl crate::RegisterSpec for LvGpioCtl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl0::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl0::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL0 to value 0"]
impl crate::Resettable for LvGpioCtl0Spec {
    const RESET_VALUE: u8 = 0;
}
