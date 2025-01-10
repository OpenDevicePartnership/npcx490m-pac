#[doc = "Register `LV_GPIO_CTLP` reader"]
pub type R = crate::R<LvGpioCtlpSpec>;
#[doc = "Register `LV_GPIO_CTLP` writer"]
pub type W = crate::W<LvGpioCtlpSpec>;
#[doc = "Field `PSL_IN1_LV` reader - PSL_IN1 Low-Voltage Select"]
pub type PslIn1LvR = crate::BitReader;
#[doc = "Field `PSL_IN1_LV` writer - PSL_IN1 Low-Voltage Select"]
pub type PslIn1LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN2_LV` reader - PSL_IN2 Low-Voltage Select"]
pub type PslIn2LvR = crate::BitReader;
#[doc = "Field `PSL_IN2_LV` writer - PSL_IN2 Low-Voltage Select"]
pub type PslIn2LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN3_LV` reader - PSL_IN3 Low-Voltage Select"]
pub type PslIn3LvR = crate::BitReader;
#[doc = "Field `PSL_IN3_LV` writer - PSL_IN3 Low-Voltage Select"]
pub type PslIn3LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_IN4_LV` reader - PSL_IN4 Low-Voltage Select"]
pub type PslIn4LvR = crate::BitReader;
#[doc = "Field `PSL_IN4_LV` writer - PSL_IN4 Low-Voltage Select"]
pub type PslIn4LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD1_LV` reader - INTRUDER1 Low-Voltage Select"]
pub type Intrud1LvR = crate::BitReader;
#[doc = "Field `INTRUD1_LV` writer - INTRUDER1 Low-Voltage Select"]
pub type Intrud1LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD2_LV` reader - INTRUDER2 Low-Voltage Select"]
pub type Intrud2LvR = crate::BitReader;
#[doc = "Field `INTRUD2_LV` writer - INTRUDER2 Low-Voltage Select"]
pub type Intrud2LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PSL_IN1 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in1_lv(&self) -> PslIn1LvR {
        PslIn1LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSL_IN2 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in2_lv(&self) -> PslIn2LvR {
        PslIn2LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSL_IN3 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in3_lv(&self) -> PslIn3LvR {
        PslIn3LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PSL_IN4 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in4_lv(&self) -> PslIn4LvR {
        PslIn4LvR::new(((self.bits >> 3) & 1) != 0)
    }
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
            .field("psl_in1_lv", &self.psl_in1_lv())
            .field("psl_in2_lv", &self.psl_in2_lv())
            .field("psl_in3_lv", &self.psl_in3_lv())
            .field("psl_in4_lv", &self.psl_in4_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PSL_IN1 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in1_lv(&mut self) -> PslIn1LvW<LvGpioCtlpSpec> {
        PslIn1LvW::new(self, 0)
    }
    #[doc = "Bit 1 - PSL_IN2 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in2_lv(&mut self) -> PslIn2LvW<LvGpioCtlpSpec> {
        PslIn2LvW::new(self, 1)
    }
    #[doc = "Bit 2 - PSL_IN3 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in3_lv(&mut self) -> PslIn3LvW<LvGpioCtlpSpec> {
        PslIn3LvW::new(self, 2)
    }
    #[doc = "Bit 3 - PSL_IN4 Low-Voltage Select"]
    #[inline(always)]
    pub fn psl_in4_lv(&mut self) -> PslIn4LvW<LvGpioCtlpSpec> {
        PslIn4LvW::new(self, 3)
    }
    #[doc = "Bit 6 - INTRUDER1 Low-Voltage Select"]
    #[inline(always)]
    pub fn intrud1_lv(&mut self) -> Intrud1LvW<LvGpioCtlpSpec> {
        Intrud1LvW::new(self, 6)
    }
    #[doc = "Bit 7 - INTRUDER2 Low-Voltage Select"]
    #[inline(always)]
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
