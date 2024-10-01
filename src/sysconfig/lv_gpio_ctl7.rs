#[doc = "Register `LV_GPIO_CTL7` reader"]
pub type R = crate::R<LvGpioCtl7Spec>;
#[doc = "Register `LV_GPIO_CTL7` writer"]
pub type W = crate::W<LvGpioCtl7Spec>;
#[doc = "Field `G14_LV` reader - GPIO14 Low-Voltage Select"]
pub type G14LvR = crate::BitReader;
#[doc = "Field `G14_LV` writer - GPIO14 Low-Voltage Select"]
pub type G14LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G15_LV` reader - GPIO15 Low-Voltage Select"]
pub type G15LvR = crate::BitReader;
#[doc = "Field `G15_LV` writer - GPIO15 Low-Voltage Select"]
pub type G15LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G16_LV` reader - GPIO16 Low-Voltage Select"]
pub type G16LvR = crate::BitReader;
#[doc = "Field `G16_LV` writer - GPIO16 Low-Voltage Select"]
pub type G16LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G17_LV` reader - GPIO17 Low-Voltage Select"]
pub type G17LvR = crate::BitReader;
#[doc = "Field `G17_LV` writer - GPIO17 Low-Voltage Select"]
pub type G17LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G20_LV` reader - GPIO20 Low-Voltage Select"]
pub type G20LvR = crate::BitReader;
#[doc = "Field `G20_LV` writer - GPIO20 Low-Voltage Select"]
pub type G20LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G21_LV` reader - GPIO21 Low-Voltage Select"]
pub type G21LvR = crate::BitReader;
#[doc = "Field `G21_LV` writer - GPIO21 Low-Voltage Select"]
pub type G21LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G22_LV` reader - GPIO22 Low-Voltage Select"]
pub type G22LvR = crate::BitReader;
#[doc = "Field `G22_LV` writer - GPIO22 Low-Voltage Select"]
pub type G22LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G23_LV` reader - GPIO23 Low-Voltage Select"]
pub type G23LvR = crate::BitReader;
#[doc = "Field `G23_LV` writer - GPIO23 Low-Voltage Select"]
pub type G23LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO14 Low-Voltage Select"]
    #[inline(always)]
    pub fn g14_lv(&self) -> G14LvR {
        G14LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO15 Low-Voltage Select"]
    #[inline(always)]
    pub fn g15_lv(&self) -> G15LvR {
        G15LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO16 Low-Voltage Select"]
    #[inline(always)]
    pub fn g16_lv(&self) -> G16LvR {
        G16LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO17 Low-Voltage Select"]
    #[inline(always)]
    pub fn g17_lv(&self) -> G17LvR {
        G17LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO20 Low-Voltage Select"]
    #[inline(always)]
    pub fn g20_lv(&self) -> G20LvR {
        G20LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO21 Low-Voltage Select"]
    #[inline(always)]
    pub fn g21_lv(&self) -> G21LvR {
        G21LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO22 Low-Voltage Select"]
    #[inline(always)]
    pub fn g22_lv(&self) -> G22LvR {
        G22LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO23 Low-Voltage Select"]
    #[inline(always)]
    pub fn g23_lv(&self) -> G23LvR {
        G23LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL7")
            .field("g14_lv", &self.g14_lv())
            .field("g15_lv", &self.g15_lv())
            .field("g16_lv", &self.g16_lv())
            .field("g17_lv", &self.g17_lv())
            .field("g20_lv", &self.g20_lv())
            .field("g21_lv", &self.g21_lv())
            .field("g22_lv", &self.g22_lv())
            .field("g23_lv", &self.g23_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIO14 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g14_lv(&mut self) -> G14LvW<LvGpioCtl7Spec> {
        G14LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO15 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g15_lv(&mut self) -> G15LvW<LvGpioCtl7Spec> {
        G15LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO16 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g16_lv(&mut self) -> G16LvW<LvGpioCtl7Spec> {
        G16LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO17 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g17_lv(&mut self) -> G17LvW<LvGpioCtl7Spec> {
        G17LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO20 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g20_lv(&mut self) -> G20LvW<LvGpioCtl7Spec> {
        G20LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO21 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g21_lv(&mut self) -> G21LvW<LvGpioCtl7Spec> {
        G21LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO22 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g22_lv(&mut self) -> G22LvW<LvGpioCtl7Spec> {
        G22LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO23 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g23_lv(&mut self) -> G23LvW<LvGpioCtl7Spec> {
        G23LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 7 Register (LV_GPIO_CTL7)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl7Spec;
impl crate::RegisterSpec for LvGpioCtl7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl7::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl7Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl7::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL7 to value 0"]
impl crate::Resettable for LvGpioCtl7Spec {
    const RESET_VALUE: u8 = 0;
}
