#[doc = "Register `LV_GPIO_CTL6` reader"]
pub type R = crate::R<LvGpioCtl6Spec>;
#[doc = "Register `LV_GPIO_CTL6` writer"]
pub type W = crate::W<LvGpioCtl6Spec>;
#[doc = "Field `G03_LV` reader - GPIO03 Low-Voltage Select"]
pub type G03LvR = crate::BitReader;
#[doc = "Field `G03_LV` writer - GPIO03 Low-Voltage Select"]
pub type G03LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G05_LV` reader - GPIO05 Low-Voltage Select"]
pub type G05LvR = crate::BitReader;
#[doc = "Field `G05_LV` writer - GPIO05 Low-Voltage Select"]
pub type G05LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G04_LV` reader - GPIO04 Low-Voltage Select"]
pub type G04LvR = crate::BitReader;
#[doc = "Field `G04_LV` writer - GPIO04 Low-Voltage Select"]
pub type G04LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G06_LV` reader - GPIO06 Low-Voltage Select"]
pub type G06LvR = crate::BitReader;
#[doc = "Field `G06_LV` writer - GPIO06 Low-Voltage Select"]
pub type G06LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G07_LV` reader - GPIO07 Low-Voltage Select"]
pub type G07LvR = crate::BitReader;
#[doc = "Field `G07_LV` writer - GPIO07 Low-Voltage Select"]
pub type G07LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G10_LV` reader - GPIO10/CR_SIN1 Low-Voltage Select"]
pub type G10LvR = crate::BitReader;
#[doc = "Field `G10_LV` writer - GPIO10/CR_SIN1 Low-Voltage Select"]
pub type G10LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G11_LV` reader - GPIO11/CR_SOUT1 Low-Voltage Select"]
pub type G11LvR = crate::BitReader;
#[doc = "Field `G11_LV` writer - GPIO11/CR_SOUT1 Low-Voltage Select"]
pub type G11LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G13_LV` reader - GPIO13 Low-Voltage Select"]
pub type G13LvR = crate::BitReader;
#[doc = "Field `G13_LV` writer - GPIO13 Low-Voltage Select"]
pub type G13LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO03 Low-Voltage Select"]
    #[inline(always)]
    pub fn g03_lv(&self) -> G03LvR {
        G03LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO05 Low-Voltage Select"]
    #[inline(always)]
    pub fn g05_lv(&self) -> G05LvR {
        G05LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO04 Low-Voltage Select"]
    #[inline(always)]
    pub fn g04_lv(&self) -> G04LvR {
        G04LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO06 Low-Voltage Select"]
    #[inline(always)]
    pub fn g06_lv(&self) -> G06LvR {
        G06LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO07 Low-Voltage Select"]
    #[inline(always)]
    pub fn g07_lv(&self) -> G07LvR {
        G07LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO10/CR_SIN1 Low-Voltage Select"]
    #[inline(always)]
    pub fn g10_lv(&self) -> G10LvR {
        G10LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO11/CR_SOUT1 Low-Voltage Select"]
    #[inline(always)]
    pub fn g11_lv(&self) -> G11LvR {
        G11LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO13 Low-Voltage Select"]
    #[inline(always)]
    pub fn g13_lv(&self) -> G13LvR {
        G13LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL6")
            .field("g03_lv", &self.g03_lv())
            .field("g05_lv", &self.g05_lv())
            .field("g04_lv", &self.g04_lv())
            .field("g06_lv", &self.g06_lv())
            .field("g07_lv", &self.g07_lv())
            .field("g10_lv", &self.g10_lv())
            .field("g11_lv", &self.g11_lv())
            .field("g13_lv", &self.g13_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIO03 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g03_lv(&mut self) -> G03LvW<LvGpioCtl6Spec> {
        G03LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO05 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g05_lv(&mut self) -> G05LvW<LvGpioCtl6Spec> {
        G05LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO04 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g04_lv(&mut self) -> G04LvW<LvGpioCtl6Spec> {
        G04LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO06 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g06_lv(&mut self) -> G06LvW<LvGpioCtl6Spec> {
        G06LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO07 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g07_lv(&mut self) -> G07LvW<LvGpioCtl6Spec> {
        G07LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO10/CR_SIN1 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g10_lv(&mut self) -> G10LvW<LvGpioCtl6Spec> {
        G10LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO11/CR_SOUT1 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g11_lv(&mut self) -> G11LvW<LvGpioCtl6Spec> {
        G11LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO13 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g13_lv(&mut self) -> G13LvW<LvGpioCtl6Spec> {
        G13LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 6 Register (LV_GPIO_CTL6)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl6Spec;
impl crate::RegisterSpec for LvGpioCtl6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl6::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl6Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl6::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL6 to value 0"]
impl crate::Resettable for LvGpioCtl6Spec {
    const RESET_VALUE: u8 = 0;
}
