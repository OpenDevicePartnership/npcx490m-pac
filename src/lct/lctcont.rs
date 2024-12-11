#[doc = "Register `LCTCONT` reader"]
pub type R = crate::R<LctcontSpec>;
#[doc = "Register `LCTCONT` writer"]
pub type W = crate::W<LctcontSpec>;
#[doc = "Field `LCTEN` reader - LCT Enable"]
pub type LctenR = crate::BitReader;
#[doc = "Field `LCTEN` writer - LCT Enable"]
pub type LctenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCTEVEN` reader - LCT Event Enable"]
pub type LctevenR = crate::BitReader;
#[doc = "Field `LCTEVEN` writer - LCT Event Enable"]
pub type LctevenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCT_PSL_EN` reader - LCT PSL Event Enable"]
pub type LctPslEnR = crate::BitReader;
#[doc = "Field `LCT_PSL_EN` writer - LCT PSL Event Enable"]
pub type LctPslEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCT_CLK_EN` reader - LCT Clock Enable"]
pub type LctClkEnR = crate::BitReader;
#[doc = "Field `LCT_CLK_EN` writer - LCT Clock Enable"]
pub type LctClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCT_VSBY_PWR` reader - LCT, VSBY Power Select"]
pub type LctVsbyPwrR = crate::BitReader;
#[doc = "Field `LCT_VSBY_PWR` writer - LCT, VSBY Power Select"]
pub type LctVsbyPwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCT Enable"]
    #[inline(always)]
    pub fn lcten(&self) -> LctenR {
        LctenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCT Event Enable"]
    #[inline(always)]
    pub fn lcteven(&self) -> LctevenR {
        LctevenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCT PSL Event Enable"]
    #[inline(always)]
    pub fn lct_psl_en(&self) -> LctPslEnR {
        LctPslEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - LCT Clock Enable"]
    #[inline(always)]
    pub fn lct_clk_en(&self) -> LctClkEnR {
        LctClkEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCT, VSBY Power Select"]
    #[inline(always)]
    pub fn lct_vsby_pwr(&self) -> LctVsbyPwrR {
        LctVsbyPwrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCTCONT")
            .field("lcten", &self.lcten())
            .field("lcteven", &self.lcteven())
            .field("lct_psl_en", &self.lct_psl_en())
            .field("lct_clk_en", &self.lct_clk_en())
            .field("lct_vsby_pwr", &self.lct_vsby_pwr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - LCT Enable"]
    #[inline(always)]
    pub fn lcten(&mut self) -> LctenW<LctcontSpec> {
        LctenW::new(self, 0)
    }
    #[doc = "Bit 1 - LCT Event Enable"]
    #[inline(always)]
    pub fn lcteven(&mut self) -> LctevenW<LctcontSpec> {
        LctevenW::new(self, 1)
    }
    #[doc = "Bit 2 - LCT PSL Event Enable"]
    #[inline(always)]
    pub fn lct_psl_en(&mut self) -> LctPslEnW<LctcontSpec> {
        LctPslEnW::new(self, 2)
    }
    #[doc = "Bit 6 - LCT Clock Enable"]
    #[inline(always)]
    pub fn lct_clk_en(&mut self) -> LctClkEnW<LctcontSpec> {
        LctClkEnW::new(self, 6)
    }
    #[doc = "Bit 7 - LCT, VSBY Power Select"]
    #[inline(always)]
    pub fn lct_vsby_pwr(&mut self) -> LctVsbyPwrW<LctcontSpec> {
        LctVsbyPwrW::new(self, 7)
    }
}
#[doc = "LCT Control Register (LCTCONT)\n\nYou can [`read`](crate::Reg::read) this register and get [`lctcont::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lctcont::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LctcontSpec;
impl crate::RegisterSpec for LctcontSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lctcont::R`](R) reader structure"]
impl crate::Readable for LctcontSpec {}
#[doc = "`write(|w| ..)` method takes [`lctcont::W`](W) writer structure"]
impl crate::Writable for LctcontSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCTCONT to value 0"]
impl crate::Resettable for LctcontSpec {
    const RESET_VALUE: u8 = 0;
}
