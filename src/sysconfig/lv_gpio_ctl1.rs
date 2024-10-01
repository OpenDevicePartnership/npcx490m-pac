#[doc = "Register `LV_GPIO_CTL1` reader"]
pub type R = crate::R<LvGpioCtl1Spec>;
#[doc = "Register `LV_GPIO_CTL1` writer"]
pub type W = crate::W<LvGpioCtl1Spec>;
#[doc = "Field `G92_LV` reader - GPIO92/I2C2_SCL0 Low-Voltage Select"]
pub type G92LvR = crate::BitReader;
#[doc = "Field `G92_LV` writer - GPIO92/I2C2_SCL0 Low-Voltage Select"]
pub type G92LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G91_LV` reader - GPIO91/I2C2_SDA0 Low-Voltage Select"]
pub type G91LvR = crate::BitReader;
#[doc = "Field `G91_LV` writer - GPIO91/I2C2_SDA0 Low-Voltage Select"]
pub type G91LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GD1_LV` reader - GPIOD1/I2C3_SCL0 Low-Voltage Select"]
pub type Gd1LvR = crate::BitReader;
#[doc = "Field `GD1_LV` writer - GPIOD1/I2C3_SCL0 Low-Voltage Select"]
pub type Gd1LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GD0_LV` reader - GPIOD0/I2C3_SDA0 Low-Voltage Select"]
pub type Gd0LvR = crate::BitReader;
#[doc = "Field `GD0_LV` writer - GPIOD0/I2C3_SDA0 Low-Voltage Select"]
pub type Gd0LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G36_LV` reader - GPIO36/I2C5_SDA0 Low-Voltage Select"]
pub type G36LvR = crate::BitReader;
#[doc = "Field `G36_LV` writer - GPIO36/I2C5_SDA0 Low-Voltage Select"]
pub type G36LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G64_LV` reader - GPIO64/CR_SIN1 Low-Voltage Select"]
pub type G64LvR = crate::BitReader;
#[doc = "Field `G64_LV` writer - GPIO64/CR_SIN1 Low-Voltage Select"]
pub type G64LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G65_LV` reader - GPIO65/CR_SOUT1 Low-Voltage Select"]
pub type G65LvR = crate::BitReader;
#[doc = "Field `G65_LV` writer - GPIO65/CR_SOUT1 Low-Voltage Select"]
pub type G65LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G66_LV` reader - GPIO66 Low-Voltage Select"]
pub type G66LvR = crate::BitReader;
#[doc = "Field `G66_LV` writer - GPIO66 Low-Voltage Select"]
pub type G66LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO92/I2C2_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g92_lv(&self) -> G92LvR {
        G92LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO91/I2C2_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g91_lv(&self) -> G91LvR {
        G91LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOD1/I2C3_SCL0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd1_lv(&self) -> Gd1LvR {
        Gd1LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD0/I2C3_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd0_lv(&self) -> Gd0LvR {
        Gd0LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO36/I2C5_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn g36_lv(&self) -> G36LvR {
        G36LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO64/CR_SIN1 Low-Voltage Select"]
    #[inline(always)]
    pub fn g64_lv(&self) -> G64LvR {
        G64LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO65/CR_SOUT1 Low-Voltage Select"]
    #[inline(always)]
    pub fn g65_lv(&self) -> G65LvR {
        G65LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO66 Low-Voltage Select"]
    #[inline(always)]
    pub fn g66_lv(&self) -> G66LvR {
        G66LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL1")
            .field("g92_lv", &self.g92_lv())
            .field("g91_lv", &self.g91_lv())
            .field("gd1_lv", &self.gd1_lv())
            .field("gd0_lv", &self.gd0_lv())
            .field("g36_lv", &self.g36_lv())
            .field("g64_lv", &self.g64_lv())
            .field("g65_lv", &self.g65_lv())
            .field("g66_lv", &self.g66_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIO92/I2C2_SCL0 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g92_lv(&mut self) -> G92LvW<LvGpioCtl1Spec> {
        G92LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO91/I2C2_SDA0 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g91_lv(&mut self) -> G91LvW<LvGpioCtl1Spec> {
        G91LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOD1/I2C3_SCL0 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn gd1_lv(&mut self) -> Gd1LvW<LvGpioCtl1Spec> {
        Gd1LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD0/I2C3_SDA0 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn gd0_lv(&mut self) -> Gd0LvW<LvGpioCtl1Spec> {
        Gd0LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO36/I2C5_SDA0 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g36_lv(&mut self) -> G36LvW<LvGpioCtl1Spec> {
        G36LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO64/CR_SIN1 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g64_lv(&mut self) -> G64LvW<LvGpioCtl1Spec> {
        G64LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO65/CR_SOUT1 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g65_lv(&mut self) -> G65LvW<LvGpioCtl1Spec> {
        G65LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO66 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g66_lv(&mut self) -> G66LvW<LvGpioCtl1Spec> {
        G66LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 1 Register (LV_GPIO_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl1Spec;
impl crate::RegisterSpec for LvGpioCtl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl1::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl1::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL1 to value 0"]
impl crate::Resettable for LvGpioCtl1Spec {
    const RESET_VALUE: u8 = 0;
}
