#[doc = "Register `LV_GPIO_CTL8` reader"]
pub type R = crate::R<LvGpioCtl8Spec>;
#[doc = "Register `LV_GPIO_CTL8` writer"]
pub type W = crate::W<LvGpioCtl8Spec>;
#[doc = "Field `GE0_LV` reader - GPIOE0 Low-Voltage Select"]
pub type Ge0LvR = crate::BitReader;
#[doc = "Field `GE0_LV` writer - GPIOE0 Low-Voltage Select"]
pub type Ge0LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G41_LV` reader - GPIO41 Low-Voltage Select"]
pub type G41LvR = crate::BitReader;
#[doc = "Field `G41_LV` writer - GPIO41 Low-Voltage Select"]
pub type G41LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GF0_LV` reader - GPIOF0 Low-Voltage Select"]
pub type Gf0LvR = crate::BitReader;
#[doc = "Field `GF0_LV` writer - GPIOF0 Low-Voltage Select"]
pub type Gf0LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G42_LV` reader - GPIO42 Low-Voltage Select"]
pub type G42LvR = crate::BitReader;
#[doc = "Field `G42_LV` writer - GPIO42 Low-Voltage Select"]
pub type G42LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G43_LV` reader - GPIO43 Low-Voltage Select"]
pub type G43LvR = crate::BitReader;
#[doc = "Field `G43_LV` writer - GPIO43 Low-Voltage Select"]
pub type G43LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G44_LV` reader - GPIO44 Low-Voltage Select"]
pub type G44LvR = crate::BitReader;
#[doc = "Field `G44_LV` writer - GPIO44 Low-Voltage Select"]
pub type G44LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G45_LV` reader - GPIO45 Low-Voltage Select"]
pub type G45LvR = crate::BitReader;
#[doc = "Field `G45_LV` writer - GPIO45 Low-Voltage Select"]
pub type G45LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE1_LV` reader - GPIOE1 Low-Voltage Select"]
pub type Ge1LvR = crate::BitReader;
#[doc = "Field `GE1_LV` writer - GPIOE1 Low-Voltage Select"]
pub type Ge1LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOE0 Low-Voltage Select"]
    #[inline(always)]
    pub fn ge0_lv(&self) -> Ge0LvR {
        Ge0LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO41 Low-Voltage Select"]
    #[inline(always)]
    pub fn g41_lv(&self) -> G41LvR {
        G41LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOF0 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf0_lv(&self) -> Gf0LvR {
        Gf0LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO42 Low-Voltage Select"]
    #[inline(always)]
    pub fn g42_lv(&self) -> G42LvR {
        G42LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO43 Low-Voltage Select"]
    #[inline(always)]
    pub fn g43_lv(&self) -> G43LvR {
        G43LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO44 Low-Voltage Select"]
    #[inline(always)]
    pub fn g44_lv(&self) -> G44LvR {
        G44LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO45 Low-Voltage Select"]
    #[inline(always)]
    pub fn g45_lv(&self) -> G45LvR {
        G45LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOE1 Low-Voltage Select"]
    #[inline(always)]
    pub fn ge1_lv(&self) -> Ge1LvR {
        Ge1LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL8")
            .field("ge0_lv", &self.ge0_lv())
            .field("g41_lv", &self.g41_lv())
            .field("gf0_lv", &self.gf0_lv())
            .field("g42_lv", &self.g42_lv())
            .field("g43_lv", &self.g43_lv())
            .field("g44_lv", &self.g44_lv())
            .field("g45_lv", &self.g45_lv())
            .field("ge1_lv", &self.ge1_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIOE0 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn ge0_lv(&mut self) -> Ge0LvW<LvGpioCtl8Spec> {
        Ge0LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO41 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g41_lv(&mut self) -> G41LvW<LvGpioCtl8Spec> {
        G41LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOF0 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn gf0_lv(&mut self) -> Gf0LvW<LvGpioCtl8Spec> {
        Gf0LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO42 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g42_lv(&mut self) -> G42LvW<LvGpioCtl8Spec> {
        G42LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO43 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g43_lv(&mut self) -> G43LvW<LvGpioCtl8Spec> {
        G43LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO44 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g44_lv(&mut self) -> G44LvW<LvGpioCtl8Spec> {
        G44LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO45 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g45_lv(&mut self) -> G45LvW<LvGpioCtl8Spec> {
        G45LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOE1 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn ge1_lv(&mut self) -> Ge1LvW<LvGpioCtl8Spec> {
        Ge1LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 8 Register (LV_GPIO_CTL8)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl8Spec;
impl crate::RegisterSpec for LvGpioCtl8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl8::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl8Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl8::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL8 to value 0"]
impl crate::Resettable for LvGpioCtl8Spec {
    const RESET_VALUE: u8 = 0;
}
