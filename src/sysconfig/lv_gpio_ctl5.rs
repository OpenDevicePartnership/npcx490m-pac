#[doc = "Register `LV_GPIO_CTL5` reader"]
pub type R = crate::R<LvGpioCtl5Spec>;
#[doc = "Register `LV_GPIO_CTL5` writer"]
pub type W = crate::W<LvGpioCtl5Spec>;
#[doc = "Field `G72_LV` reader - GPIO72 Low-Voltage Select"]
pub type G72LvR = crate::BitReader;
#[doc = "Field `G72_LV` writer - GPIO72 Low-Voltage Select"]
pub type G72LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G50_LV` reader - GPIO50 Low-Voltage Select"]
pub type G50LvR = crate::BitReader;
#[doc = "Field `G50_LV` writer - GPIO50 Low-Voltage Select"]
pub type G50LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G02_LV` reader - GPIO02 Low-Voltage Select"]
pub type G02LvR = crate::BitReader;
#[doc = "Field `G02_LV` writer - GPIO02 Low-Voltage Select"]
pub type G02LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G01_LV` reader - GPIO01 Low-Voltage Select"]
pub type G01LvR = crate::BitReader;
#[doc = "Field `G01_LV` writer - GPIO01 Low-Voltage Select"]
pub type G01LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE2_LV` reader - GPIOE2/FLM_CSI Low-Voltage Select"]
pub type Ge2LvR = crate::BitReader;
#[doc = "Field `GE2_LV` writer - GPIOE2/FLM_CSI Low-Voltage Select"]
pub type Ge2LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GD6_LV` reader - GPIOD6/CR_SOUT3 Low-Voltage Select"]
pub type Gd6LvR = crate::BitReader;
#[doc = "Field `GD6_LV` writer - GPIOD6/CR_SOUT3 Low-Voltage Select"]
pub type Gd6LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO72 Low-Voltage Select"]
    #[inline(always)]
    pub fn g72_lv(&self) -> G72LvR {
        G72LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO50 Low-Voltage Select"]
    #[inline(always)]
    pub fn g50_lv(&self) -> G50LvR {
        G50LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO02 Low-Voltage Select"]
    #[inline(always)]
    pub fn g02_lv(&self) -> G02LvR {
        G02LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO01 Low-Voltage Select"]
    #[inline(always)]
    pub fn g01_lv(&self) -> G01LvR {
        G01LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOE2/FLM_CSI Low-Voltage Select"]
    #[inline(always)]
    pub fn ge2_lv(&self) -> Ge2LvR {
        Ge2LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOD6/CR_SOUT3 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd6_lv(&self) -> Gd6LvR {
        Gd6LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL5")
            .field("g72_lv", &self.g72_lv())
            .field("g50_lv", &self.g50_lv())
            .field("g02_lv", &self.g02_lv())
            .field("g01_lv", &self.g01_lv())
            .field("ge2_lv", &self.ge2_lv())
            .field("gd6_lv", &self.gd6_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIO72 Low-Voltage Select"]
    #[inline(always)]
    pub fn g72_lv(&mut self) -> G72LvW<LvGpioCtl5Spec> {
        G72LvW::new(self, 0)
    }
    #[doc = "Bit 3 - GPIO50 Low-Voltage Select"]
    #[inline(always)]
    pub fn g50_lv(&mut self) -> G50LvW<LvGpioCtl5Spec> {
        G50LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO02 Low-Voltage Select"]
    #[inline(always)]
    pub fn g02_lv(&mut self) -> G02LvW<LvGpioCtl5Spec> {
        G02LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO01 Low-Voltage Select"]
    #[inline(always)]
    pub fn g01_lv(&mut self) -> G01LvW<LvGpioCtl5Spec> {
        G01LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOE2/FLM_CSI Low-Voltage Select"]
    #[inline(always)]
    pub fn ge2_lv(&mut self) -> Ge2LvW<LvGpioCtl5Spec> {
        Ge2LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOD6/CR_SOUT3 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd6_lv(&mut self) -> Gd6LvW<LvGpioCtl5Spec> {
        Gd6LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 5 Register (LV_GPIO_CTL5)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl5Spec;
impl crate::RegisterSpec for LvGpioCtl5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl5::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl5Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl5::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL5 to value 0"]
impl crate::Resettable for LvGpioCtl5Spec {
    const RESET_VALUE: u8 = 0;
}
