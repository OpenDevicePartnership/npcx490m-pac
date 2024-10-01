#[doc = "Register `LV_GPIO_CTLP` reader"]
pub type R = crate::R<LvGpioCtlpSpec>;
#[doc = "Register `LV_GPIO_CTLP` writer"]
pub type W = crate::W<LvGpioCtlpSpec>;
#[doc = "Field `INTRUD1_LV` reader - INTRUDER1 Low-Voltage Select"]
pub type Intrud1LvR = crate::BitReader;
#[doc = "Field `INTRUD1_LV` writer - INTRUDER1 Low-Voltage Select"]
pub type Intrud1LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD2_LV` reader - INTRUDER2 Low-Voltage Select"]
pub type Intrud2LvR = crate::BitReader;
#[doc = "Field `INTRUD2_LV` writer - INTRUDER2 Low-Voltage Select"]
pub type Intrud2LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - INTRUDER1 Low-Voltage Select"]
    #[inline(always)]
    pub fn intrud1_lv(&self) -> Intrud1LvR {
        Intrud1LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INTRUDER2 Low-Voltage Select"]
    #[inline(always)]
    pub fn intrud2_lv(&self) -> Intrud2LvR {
        Intrud2LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTLP")
            .field("intrud1_lv", &self.intrud1_lv())
            .field("intrud2_lv", &self.intrud2_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - INTRUDER1 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn intrud1_lv(&mut self) -> Intrud1LvW<LvGpioCtlpSpec> {
        Intrud1LvW::new(self, 6)
    }
    #[doc = "Bit 7 - INTRUDER2 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn intrud2_lv(&mut self) -> Intrud2LvW<LvGpioCtlpSpec> {
        Intrud2LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control P Register (LV_GPIO_CTLP)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctlp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctlp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtlpSpec;
impl crate::RegisterSpec for LvGpioCtlpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctlp::R`](R) reader structure"]
impl crate::Readable for LvGpioCtlpSpec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctlp::W`](W) writer structure"]
impl crate::Writable for LvGpioCtlpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTLP to value 0"]
impl crate::Resettable for LvGpioCtlpSpec {
    const RESET_VALUE: u8 = 0;
}
