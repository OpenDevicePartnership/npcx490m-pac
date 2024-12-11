#[doc = "Register `RAM_LK_CTL` reader"]
pub type R = crate::R<RamLkCtlSpec>;
#[doc = "Register `RAM_LK_CTL` writer"]
pub type W = crate::W<RamLkCtlSpec>;
#[doc = "Field `WLK_BF_EN` reader - RAM Write Lock BusFault Trap Enable"]
pub type WlkBfEnR = crate::BitReader;
#[doc = "Field `WLK_BF_EN` writer - RAM Write Lock BusFault Trap Enable"]
pub type WlkBfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_BF_EN` reader - RAM Fetch BusFault Trap Enable"]
pub type FetchBfEnR = crate::BitReader;
#[doc = "Field `FETCH_BF_EN` writer - RAM Fetch BusFault Trap Enable"]
pub type FetchBfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF_EN_LK` reader - BusFault Trap Enable Lock"]
pub type BfEnLkR = crate::BitReader;
#[doc = "Field `BF_EN_LK` writer - BusFault Trap Enable Lock"]
pub type BfEnLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Write Lock BusFault Trap Enable"]
    #[inline(always)]
    pub fn wlk_bf_en(&self) -> WlkBfEnR {
        WlkBfEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Fetch BusFault Trap Enable"]
    #[inline(always)]
    pub fn fetch_bf_en(&self) -> FetchBfEnR {
        FetchBfEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - BusFault Trap Enable Lock"]
    #[inline(always)]
    pub fn bf_en_lk(&self) -> BfEnLkR {
        BfEnLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_LK_CTL")
            .field("wlk_bf_en", &self.wlk_bf_en())
            .field("fetch_bf_en", &self.fetch_bf_en())
            .field("bf_en_lk", &self.bf_en_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Write Lock BusFault Trap Enable"]
    #[inline(always)]
    pub fn wlk_bf_en(&mut self) -> WlkBfEnW<RamLkCtlSpec> {
        WlkBfEnW::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Fetch BusFault Trap Enable"]
    #[inline(always)]
    pub fn fetch_bf_en(&mut self) -> FetchBfEnW<RamLkCtlSpec> {
        FetchBfEnW::new(self, 1)
    }
    #[doc = "Bit 7 - BusFault Trap Enable Lock"]
    #[inline(always)]
    pub fn bf_en_lk(&mut self) -> BfEnLkW<RamLkCtlSpec> {
        BfEnLkW::new(self, 7)
    }
}
#[doc = "RAM Lock Control Register (RAM_LK_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_lk_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_lk_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamLkCtlSpec;
impl crate::RegisterSpec for RamLkCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_lk_ctl::R`](R) reader structure"]
impl crate::Readable for RamLkCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ram_lk_ctl::W`](W) writer structure"]
impl crate::Writable for RamLkCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_LK_CTL to value 0"]
impl crate::Resettable for RamLkCtlSpec {
    const RESET_VALUE: u8 = 0;
}
