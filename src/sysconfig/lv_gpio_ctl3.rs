#[doc = "Register `LV_GPIO_CTL3` reader"]
pub type R = crate::R<LvGpioCtl3Spec>;
#[doc = "Register `LV_GPIO_CTL3` writer"]
pub type W = crate::W<LvGpioCtl3Spec>;
#[doc = "Field `GC6_LV` reader - GPIOC6/SMI Low-Voltage Select"]
pub type Gc6LvR = crate::BitReader;
#[doc = "Field `GC6_LV` writer - GPIOC6/SMI Low-Voltage Select"]
pub type Gc6LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G37_LV` reader - GPIO37 Low-Voltage Select"]
pub type G37LvR = crate::BitReader;
#[doc = "Field `G37_LV` writer - GPIO37 Low-Voltage Select"]
pub type G37LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G40_LV` reader - GPIO40/TA1_TACH1 Low-Voltage Select"]
pub type G40LvR = crate::BitReader;
#[doc = "Field `G40_LV` writer - GPIO40/TA1_TACH1 Low-Voltage Select"]
pub type G40LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G82_LV` reader - GPIO82 Low-Voltage Select"]
pub type G82LvR = crate::BitReader;
#[doc = "Field `G82_LV` writer - GPIO82 Low-Voltage Select"]
pub type G82LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G75_LV` reader - GPIO75 Low-Voltage Select"]
pub type G75LvR = crate::BitReader;
#[doc = "Field `G75_LV` writer - GPIO75 Low-Voltage Select"]
pub type G75LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G80_LV` reader - GPIO80 Low-Voltage Select"]
pub type G80LvR = crate::BitReader;
#[doc = "Field `G80_LV` writer - GPIO80 Low-Voltage Select"]
pub type G80LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5_LV` reader - GPIOC5 Low-Voltage Select"]
pub type Gc5LvR = crate::BitReader;
#[doc = "Field `GC5_LV` writer - GPIOC5 Low-Voltage Select"]
pub type Gc5LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOC6/SMI Low-Voltage Select"]
    #[inline(always)]
    pub fn gc6_lv(&self) -> Gc6LvR {
        Gc6LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO37 Low-Voltage Select"]
    #[inline(always)]
    pub fn g37_lv(&self) -> G37LvR {
        G37LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO40/TA1_TACH1 Low-Voltage Select"]
    #[inline(always)]
    pub fn g40_lv(&self) -> G40LvR {
        G40LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO82 Low-Voltage Select"]
    #[inline(always)]
    pub fn g82_lv(&self) -> G82LvR {
        G82LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO75 Low-Voltage Select"]
    #[inline(always)]
    pub fn g75_lv(&self) -> G75LvR {
        G75LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO80 Low-Voltage Select"]
    #[inline(always)]
    pub fn g80_lv(&self) -> G80LvR {
        G80LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOC5 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc5_lv(&self) -> Gc5LvR {
        Gc5LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL3")
            .field("gc6_lv", &self.gc6_lv())
            .field("g37_lv", &self.g37_lv())
            .field("g40_lv", &self.g40_lv())
            .field("g82_lv", &self.g82_lv())
            .field("g75_lv", &self.g75_lv())
            .field("g80_lv", &self.g80_lv())
            .field("gc5_lv", &self.gc5_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIOC6/SMI Low-Voltage Select"]
    #[inline(always)]
    pub fn gc6_lv(&mut self) -> Gc6LvW<LvGpioCtl3Spec> {
        Gc6LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO37 Low-Voltage Select"]
    #[inline(always)]
    pub fn g37_lv(&mut self) -> G37LvW<LvGpioCtl3Spec> {
        G37LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO40/TA1_TACH1 Low-Voltage Select"]
    #[inline(always)]
    pub fn g40_lv(&mut self) -> G40LvW<LvGpioCtl3Spec> {
        G40LvW::new(self, 2)
    }
    #[doc = "Bit 4 - GPIO82 Low-Voltage Select"]
    #[inline(always)]
    pub fn g82_lv(&mut self) -> G82LvW<LvGpioCtl3Spec> {
        G82LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO75 Low-Voltage Select"]
    #[inline(always)]
    pub fn g75_lv(&mut self) -> G75LvW<LvGpioCtl3Spec> {
        G75LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO80 Low-Voltage Select"]
    #[inline(always)]
    pub fn g80_lv(&mut self) -> G80LvW<LvGpioCtl3Spec> {
        G80LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOC5 Low-Voltage Select"]
    #[inline(always)]
    pub fn gc5_lv(&mut self) -> Gc5LvW<LvGpioCtl3Spec> {
        Gc5LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 3 Register (LV_GPIO_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl3Spec;
impl crate::RegisterSpec for LvGpioCtl3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl3::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl3::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL3 to value 0"]
impl crate::Resettable for LvGpioCtl3Spec {
    const RESET_VALUE: u8 = 0;
}
