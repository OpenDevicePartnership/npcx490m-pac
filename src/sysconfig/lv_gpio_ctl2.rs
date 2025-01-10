#[doc = "Register `LV_GPIO_CTL2` reader"]
pub type R = crate::R<LvGpioCtl2Spec>;
#[doc = "Register `LV_GPIO_CTL2` writer"]
pub type W = crate::W<LvGpioCtl2Spec>;
#[doc = "Field `G74_LV` reader - GPIO74/DCY_IN Low-Voltage Select"]
pub type G74LvR = crate::BitReader;
#[doc = "Field `G74_LV` writer - GPIO74/DCY_IN Low-Voltage Select"]
pub type G74LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G73_LV` reader - GPIO73/TA2 Low-Voltage Select"]
pub type G73LvR = crate::BitReader;
#[doc = "Field `G73_LV` writer - GPIO73/TA2 Low-Voltage Select"]
pub type G73LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC1_LV` reader - GPIOC1/I2C6_SDA0 Low-Voltage Select"]
pub type Gc1LvR = crate::BitReader;
#[doc = "Field `GC1_LV` writer - GPIOC1/I2C6_SDA0 Low-Voltage Select"]
pub type Gc1LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC7_LV` reader - GPIOC7 Low-Voltage Select"]
pub type Gc7LvR = crate::BitReader;
#[doc = "Field `GC7_LV` writer - GPIOC7 Low-Voltage Select"]
pub type Gc7LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE7_LV` reader - GPIOE7/32KCLKIN Low-Voltage Select"]
pub type Ge7LvR = crate::BitReader;
#[doc = "Field `GE7_LV` writer - GPIOE7/32KCLKIN Low-Voltage Select"]
pub type Ge7LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G34_LV` reader - GPIO34 Low-Voltage Select"]
pub type G34LvR = crate::BitReader;
#[doc = "Field `G34_LV` writer - GPIO34 Low-Voltage Select"]
pub type G34LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO74/DCY_IN Low-Voltage Select"]
    #[inline(always)]
    pub fn g74_lv(&self) -> G74LvR {
        G74LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO73/TA2 Low-Voltage Select"]
    #[inline(always)]
    pub fn g73_lv(&self) -> G73LvR {
        G73LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOC1/I2C6_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc1_lv(&self) -> Gc1LvR {
        Gc1LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOC7 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc7_lv(&self) -> Gc7LvR {
        Gc7LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOE7/32KCLKIN Low-Voltage Select"]
    #[inline(always)]
    pub fn ge7_lv(&self) -> Ge7LvR {
        Ge7LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO34 Low-Voltage Select"]
    #[inline(always)]
    pub fn g34_lv(&self) -> G34LvR {
        G34LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL2")
            .field("g74_lv", &self.g74_lv())
            .field("g73_lv", &self.g73_lv())
            .field("gc1_lv", &self.gc1_lv())
            .field("gc7_lv", &self.gc7_lv())
            .field("ge7_lv", &self.ge7_lv())
            .field("g34_lv", &self.g34_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIO74/DCY_IN Low-Voltage Select"]
    #[inline(always)]
    pub fn g74_lv(&mut self) -> G74LvW<LvGpioCtl2Spec> {
        G74LvW::new(self, 0)
    }
    #[doc = "Bit 3 - GPIO73/TA2 Low-Voltage Select"]
    #[inline(always)]
    pub fn g73_lv(&mut self) -> G73LvW<LvGpioCtl2Spec> {
        G73LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOC1/I2C6_SDA0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc1_lv(&mut self) -> Gc1LvW<LvGpioCtl2Spec> {
        Gc1LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOC7 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc7_lv(&mut self) -> Gc7LvW<LvGpioCtl2Spec> {
        Gc7LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOE7/32KCLKIN Low-Voltage Select"]
    #[inline(always)]
    pub fn ge7_lv(&mut self) -> Ge7LvW<LvGpioCtl2Spec> {
        Ge7LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO34 Low-Voltage Select"]
    #[inline(always)]
    pub fn g34_lv(&mut self) -> G34LvW<LvGpioCtl2Spec> {
        G34LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 2 Register (LV_GPIO_CTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl2Spec;
impl crate::RegisterSpec for LvGpioCtl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl2::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl2::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL2 to value 0"]
impl crate::Resettable for LvGpioCtl2Spec {
    const RESET_VALUE: u8 = 0;
}
