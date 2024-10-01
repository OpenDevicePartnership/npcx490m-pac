#[doc = "Register `RAM_PD3` reader"]
pub type R = crate::R<RamPd3Spec>;
#[doc = "Register `RAM_PD3` writer"]
pub type W = crate::W<RamPd3Spec>;
#[doc = "Field `RAM_PD_BKD0` reader - RAM Power-Down Data Block 0"]
pub type RamPdBkd0R = crate::BitReader;
#[doc = "Field `RAM_PD_BKD0` writer - RAM Power-Down Data Block 0"]
pub type RamPdBkd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKD1` reader - RAM Power-Down Data Block 1"]
pub type RamPdBkd1R = crate::BitReader;
#[doc = "Field `RAM_PD_BKD1` writer - RAM Power-Down Data Block 1"]
pub type RamPdBkd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PD_BKD2` reader - RAM Power-Down Data Block 2"]
pub type RamPdBkd2R = crate::BitReader;
#[doc = "Field `RAM_PD_BKD2` writer - RAM Power-Down Data Block 2"]
pub type RamPdBkd2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Power-Down Data Block 0"]
    #[inline(always)]
    pub fn ram_pd_bkd0(&self) -> RamPdBkd0R {
        RamPdBkd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Power-Down Data Block 1"]
    #[inline(always)]
    pub fn ram_pd_bkd1(&self) -> RamPdBkd1R {
        RamPdBkd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Power-Down Data Block 2"]
    #[inline(always)]
    pub fn ram_pd_bkd2(&self) -> RamPdBkd2R {
        RamPdBkd2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_PD3")
            .field("ram_pd_bkd0", &self.ram_pd_bkd0())
            .field("ram_pd_bkd1", &self.ram_pd_bkd1())
            .field("ram_pd_bkd2", &self.ram_pd_bkd2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Power-Down Data Block 0"]
    #[inline(always)]
    #[must_use]
    pub fn ram_pd_bkd0(&mut self) -> RamPdBkd0W<RamPd3Spec> {
        RamPdBkd0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Power-Down Data Block 1"]
    #[inline(always)]
    #[must_use]
    pub fn ram_pd_bkd1(&mut self) -> RamPdBkd1W<RamPd3Spec> {
        RamPdBkd1W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Power-Down Data Block 2"]
    #[inline(always)]
    #[must_use]
    pub fn ram_pd_bkd2(&mut self) -> RamPdBkd2W<RamPd3Spec> {
        RamPdBkd2W::new(self, 2)
    }
}
#[doc = "RAM Power-Down Control 3 Register (RAM_PD3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamPd3Spec;
impl crate::RegisterSpec for RamPd3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_pd3::R`](R) reader structure"]
impl crate::Readable for RamPd3Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_pd3::W`](W) writer structure"]
impl crate::Writable for RamPd3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_PD3 to value 0"]
impl crate::Resettable for RamPd3Spec {
    const RESET_VALUE: u8 = 0;
}
