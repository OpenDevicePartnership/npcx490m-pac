#[doc = "Register `LV_GPIO_CTLB` reader"]
pub type R = crate::R<LvGpioCtlbSpec>;
#[doc = "Register `LV_GPIO_CTLB` writer"]
pub type W = crate::W<LvGpioCtlbSpec>;
#[doc = "Field `GD4_LV` reader - GPIOD4/CR_SIN3 Low-Voltage Select"]
pub type Gd4LvR = crate::BitReader;
#[doc = "Field `GD4_LV` writer - GPIOD4/CR_SIN3 Low-Voltage Select"]
pub type Gd4LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GD5_LV` reader - GPIOD5 Low-Voltage Select"]
pub type Gd5LvR = crate::BitReader;
#[doc = "Field `GD5_LV` writer - GPIOD5 Low-Voltage Select"]
pub type Gd5LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE5_LV` reader - GPIOE5 Low-Voltage Select"]
pub type Ge5LvR = crate::BitReader;
#[doc = "Field `GE5_LV` writer - GPIOE5 Low-Voltage Select"]
pub type Ge5LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G60_LV` reader - GPIO60 Low-Voltage Select"]
pub type G60LvR = crate::BitReader;
#[doc = "Field `G60_LV` writer - GPIO60 Low-Voltage Select"]
pub type G60LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOD4/CR_SIN3 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd4_lv(&self) -> Gd4LvR {
        Gd4LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOD5 Low-Voltage Select"]
    #[inline(always)]
    pub fn gd5_lv(&self) -> Gd5LvR {
        Gd5LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOE5 Low-Voltage Select"]
    #[inline(always)]
    pub fn ge5_lv(&self) -> Ge5LvR {
        Ge5LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO60 Low-Voltage Select"]
    #[inline(always)]
    pub fn g60_lv(&self) -> G60LvR {
        G60LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTLB")
            .field("gd4_lv", &self.gd4_lv())
            .field("gd5_lv", &self.gd5_lv())
            .field("ge5_lv", &self.ge5_lv())
            .field("g60_lv", &self.g60_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIOD4/CR_SIN3 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn gd4_lv(&mut self) -> Gd4LvW<LvGpioCtlbSpec> {
        Gd4LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOD5 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn gd5_lv(&mut self) -> Gd5LvW<LvGpioCtlbSpec> {
        Gd5LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOE5 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn ge5_lv(&mut self) -> Ge5LvW<LvGpioCtlbSpec> {
        Ge5LvW::new(self, 2)
    }
    #[doc = "Bit 7 - GPIO60 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g60_lv(&mut self) -> G60LvW<LvGpioCtlbSpec> {
        G60LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control B Register (LV_GPIO_CTLB)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtlbSpec;
impl crate::RegisterSpec for LvGpioCtlbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctlb::R`](R) reader structure"]
impl crate::Readable for LvGpioCtlbSpec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctlb::W`](W) writer structure"]
impl crate::Writable for LvGpioCtlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTLB to value 0"]
impl crate::Resettable for LvGpioCtlbSpec {
    const RESET_VALUE: u8 = 0;
}
