#[doc = "Register `RAM_PD1` reader"]
pub type R = crate::R<RamPd1Spec>;
#[doc = "Register `RAM_PD1` writer"]
pub type W = crate::W<RamPd1Spec>;
#[doc = "Field `RAM_PD_BKC0` reader - RAM Power-Down Code Block 0"]
pub type RamPdBkc0R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC0` writer - RAM Power-Down Code Block 0"]
pub type RamPdBkc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKC1` reader - RAM Power-Down Code Block 1"]
pub type RamPdBkc1R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC1` writer - RAM Power-Down Code Block 1"]
pub type RamPdBkc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKC2` reader - RAM Power-Down Code Block 2"]
pub type RamPdBkc2R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC2` writer - RAM Power-Down Code Block 2"]
pub type RamPdBkc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKC3` reader - RAM Power-Down Code Block 3"]
pub type RamPdBkc3R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC3` writer - RAM Power-Down Code Block 3"]
pub type RamPdBkc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKC4` reader - RAM Power-Down Code Block 4"]
pub type RamPdBkc4R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC4` writer - RAM Power-Down Code Block 4"]
pub type RamPdBkc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKC5` reader - RAM Power-Down Code Block 5"]
pub type RamPdBkc5R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC5` writer - RAM Power-Down Code Block 5"]
pub type RamPdBkc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKC6` reader - RAM Power-Down Code Block 6"]
pub type RamPdBkc6R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC6` writer - RAM Power-Down Code Block 6"]
pub type RamPdBkc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKC7` reader - RAM Power-Down Code Block 7"]
pub type RamPdBkc7R = crate::BitReader;
#[doc = "Field `RAM_PD_BKC7` writer - RAM Power-Down Code Block 7"]
pub type RamPdBkc7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Power-Down Code Block 0"]
    #[inline(always)]
    pub fn ram_pd_bkc0(&self) -> RamPdBkc0R {
        RamPdBkc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Power-Down Code Block 1"]
    #[inline(always)]
    pub fn ram_pd_bkc1(&self) -> RamPdBkc1R {
        RamPdBkc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Power-Down Code Block 2"]
    #[inline(always)]
    pub fn ram_pd_bkc2(&self) -> RamPdBkc2R {
        RamPdBkc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Power-Down Code Block 3"]
    #[inline(always)]
    pub fn ram_pd_bkc3(&self) -> RamPdBkc3R {
        RamPdBkc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM Power-Down Code Block 4"]
    #[inline(always)]
    pub fn ram_pd_bkc4(&self) -> RamPdBkc4R {
        RamPdBkc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM Power-Down Code Block 5"]
    #[inline(always)]
    pub fn ram_pd_bkc5(&self) -> RamPdBkc5R {
        RamPdBkc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAM Power-Down Code Block 6"]
    #[inline(always)]
    pub fn ram_pd_bkc6(&self) -> RamPdBkc6R {
        RamPdBkc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Power-Down Code Block 7"]
    #[inline(always)]
    pub fn ram_pd_bkc7(&self) -> RamPdBkc7R {
        RamPdBkc7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_PD1")
            .field("ram_pd_bkc0", &self.ram_pd_bkc0())
            .field("ram_pd_bkc1", &self.ram_pd_bkc1())
            .field("ram_pd_bkc2", &self.ram_pd_bkc2())
            .field("ram_pd_bkc3", &self.ram_pd_bkc3())
            .field("ram_pd_bkc4", &self.ram_pd_bkc4())
            .field("ram_pd_bkc5", &self.ram_pd_bkc5())
            .field("ram_pd_bkc6", &self.ram_pd_bkc6())
            .field("ram_pd_bkc7", &self.ram_pd_bkc7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Power-Down Code Block 0"]
    #[inline(always)]
    pub fn ram_pd_bkc0(&mut self) -> RamPdBkc0W<RamPd1Spec> {
        RamPdBkc0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Power-Down Code Block 1"]
    #[inline(always)]
    pub fn ram_pd_bkc1(&mut self) -> RamPdBkc1W<RamPd1Spec> {
        RamPdBkc1W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Power-Down Code Block 2"]
    #[inline(always)]
    pub fn ram_pd_bkc2(&mut self) -> RamPdBkc2W<RamPd1Spec> {
        RamPdBkc2W::new(self, 2)
    }
    #[doc = "Bit 3 - RAM Power-Down Code Block 3"]
    #[inline(always)]
    pub fn ram_pd_bkc3(&mut self) -> RamPdBkc3W<RamPd1Spec> {
        RamPdBkc3W::new(self, 3)
    }
    #[doc = "Bit 4 - RAM Power-Down Code Block 4"]
    #[inline(always)]
    pub fn ram_pd_bkc4(&mut self) -> RamPdBkc4W<RamPd1Spec> {
        RamPdBkc4W::new(self, 4)
    }
    #[doc = "Bit 5 - RAM Power-Down Code Block 5"]
    #[inline(always)]
    pub fn ram_pd_bkc5(&mut self) -> RamPdBkc5W<RamPd1Spec> {
        RamPdBkc5W::new(self, 5)
    }
    #[doc = "Bit 6 - RAM Power-Down Code Block 6"]
    #[inline(always)]
    pub fn ram_pd_bkc6(&mut self) -> RamPdBkc6W<RamPd1Spec> {
        RamPdBkc6W::new(self, 6)
    }
    #[doc = "Bit 7 - RAM Power-Down Code Block 7"]
    #[inline(always)]
    pub fn ram_pd_bkc7(&mut self) -> RamPdBkc7W<RamPd1Spec> {
        RamPdBkc7W::new(self, 7)
    }
}
#[doc = "RAM Power-Down Control 1 Register (RAM_PD1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamPd1Spec;
impl crate::RegisterSpec for RamPd1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_pd1::R`](R) reader structure"]
impl crate::Readable for RamPd1Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_pd1::W`](W) writer structure"]
impl crate::Writable for RamPd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_PD1 to value 0"]
impl crate::Resettable for RamPd1Spec {
    const RESET_VALUE: u8 = 0;
}
