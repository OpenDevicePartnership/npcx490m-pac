#[doc = "Register `LV_GPIO_CTL9` reader"]
pub type R = crate::R<LvGpioCtl9Spec>;
#[doc = "Register `LV_GPIO_CTL9` writer"]
pub type W = crate::W<LvGpioCtl9Spec>;
#[doc = "Field `GF1_LV` reader - GPIOF1 Low-Voltage Select"]
pub type Gf1LvR = crate::BitReader;
#[doc = "Field `GF1_LV` writer - GPIOF1 Low-Voltage Select"]
pub type Gf1LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G61_LV` reader - GPIO61 Low-Voltage Select"]
pub type G61LvR = crate::BitReader;
#[doc = "Field `G61_LV` writer - GPIO61 Low-Voltage Select"]
pub type G61LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G62_LV` reader - GPIO62 Low-Voltage Select"]
pub type G62LvR = crate::BitReader;
#[doc = "Field `G62_LV` writer - GPIO62 Low-Voltage Select"]
pub type G62LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G63_LV` reader - GPIO63 Low-Voltage Select"]
pub type G63LvR = crate::BitReader;
#[doc = "Field `G63_LV` writer - GPIO63 Low-Voltage Select"]
pub type G63LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G67_LV` reader - GPIO67 Low-Voltage Select"]
pub type G67LvR = crate::BitReader;
#[doc = "Field `G67_LV` writer - GPIO67 Low-Voltage Select"]
pub type G67LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G70_LV` reader - GPIO70 Low-Voltage Select"]
pub type G70LvR = crate::BitReader;
#[doc = "Field `G70_LV` writer - GPIO70 Low-Voltage Select"]
pub type G70LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G76_LV` reader - GPIO76/EC_SCI Low-Voltage Select"]
pub type G76LvR = crate::BitReader;
#[doc = "Field `G76_LV` writer - GPIO76/EC_SCI Low-Voltage Select"]
pub type G76LvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G83_LV` reader - GPIO83 Low-Voltage Select"]
pub type G83LvR = crate::BitReader;
#[doc = "Field `G83_LV` writer - GPIO83 Low-Voltage Select"]
pub type G83LvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOF1 Low-Voltage Select"]
    #[inline(always)]
    pub fn gf1_lv(&self) -> Gf1LvR {
        Gf1LvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO61 Low-Voltage Select"]
    #[inline(always)]
    pub fn g61_lv(&self) -> G61LvR {
        G61LvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO62 Low-Voltage Select"]
    #[inline(always)]
    pub fn g62_lv(&self) -> G62LvR {
        G62LvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO63 Low-Voltage Select"]
    #[inline(always)]
    pub fn g63_lv(&self) -> G63LvR {
        G63LvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO67 Low-Voltage Select"]
    #[inline(always)]
    pub fn g67_lv(&self) -> G67LvR {
        G67LvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO70 Low-Voltage Select"]
    #[inline(always)]
    pub fn g70_lv(&self) -> G70LvR {
        G70LvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO76/EC_SCI Low-Voltage Select"]
    #[inline(always)]
    pub fn g76_lv(&self) -> G76LvR {
        G76LvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO83 Low-Voltage Select"]
    #[inline(always)]
    pub fn g83_lv(&self) -> G83LvR {
        G83LvR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LV_GPIO_CTL9")
            .field("gf1_lv", &self.gf1_lv())
            .field("g61_lv", &self.g61_lv())
            .field("g62_lv", &self.g62_lv())
            .field("g63_lv", &self.g63_lv())
            .field("g67_lv", &self.g67_lv())
            .field("g70_lv", &self.g70_lv())
            .field("g76_lv", &self.g76_lv())
            .field("g83_lv", &self.g83_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GPIOF1 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn gf1_lv(&mut self) -> Gf1LvW<LvGpioCtl9Spec> {
        Gf1LvW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO61 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g61_lv(&mut self) -> G61LvW<LvGpioCtl9Spec> {
        G61LvW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO62 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g62_lv(&mut self) -> G62LvW<LvGpioCtl9Spec> {
        G62LvW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO63 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g63_lv(&mut self) -> G63LvW<LvGpioCtl9Spec> {
        G63LvW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO67 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g67_lv(&mut self) -> G67LvW<LvGpioCtl9Spec> {
        G67LvW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO70 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g70_lv(&mut self) -> G70LvW<LvGpioCtl9Spec> {
        G70LvW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO76/EC_SCI Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g76_lv(&mut self) -> G76LvW<LvGpioCtl9Spec> {
        G76LvW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO83 Low-Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn g83_lv(&mut self) -> G83LvW<LvGpioCtl9Spec> {
        G83LvW::new(self, 7)
    }
}
#[doc = "Low-Voltage GPIO Pins Control 9 Register (LV_GPIO_CTL9)\n\nYou can [`read`](crate::Reg::read) this register and get [`lv_gpio_ctl9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lv_gpio_ctl9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvGpioCtl9Spec;
impl crate::RegisterSpec for LvGpioCtl9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lv_gpio_ctl9::R`](R) reader structure"]
impl crate::Readable for LvGpioCtl9Spec {}
#[doc = "`write(|w| ..)` method takes [`lv_gpio_ctl9::W`](W) writer structure"]
impl crate::Writable for LvGpioCtl9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LV_GPIO_CTL9 to value 0"]
impl crate::Resettable for LvGpioCtl9Spec {
    const RESET_VALUE: u8 = 0;
}
