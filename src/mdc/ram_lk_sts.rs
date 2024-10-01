#[doc = "Register `RAM_LK_STS` reader"]
pub type R = crate::R<RamLkStsSpec>;
#[doc = "Register `RAM_LK_STS` writer"]
pub type W = crate::W<RamLkStsSpec>;
#[doc = "Field `RAM_WLK_VIOL` reader - RAM Write Lock Violation"]
pub type RamWlkViolR = crate::BitReader;
#[doc = "Field `RAM_WLK_VIOL` writer - RAM Write Lock Violation"]
pub type RamWlkViolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_FETCH_VIOL` reader - RAM Fetch Violation"]
pub type RamFetchViolR = crate::BitReader;
#[doc = "Field `RAM_FETCH_VIOL` writer - RAM Fetch Violation"]
pub type RamFetchViolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM Write Lock Violation"]
    #[inline(always)]
    pub fn ram_wlk_viol(&self) -> RamWlkViolR {
        RamWlkViolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Fetch Violation"]
    #[inline(always)]
    pub fn ram_fetch_viol(&self) -> RamFetchViolR {
        RamFetchViolR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM_LK_STS")
            .field("ram_wlk_viol", &self.ram_wlk_viol())
            .field("ram_fetch_viol", &self.ram_fetch_viol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM Write Lock Violation"]
    #[inline(always)]
    #[must_use]
    pub fn ram_wlk_viol(&mut self) -> RamWlkViolW<RamLkStsSpec> {
        RamWlkViolW::new(self, 0)
    }
    #[doc = "Bit 1 - RAM Fetch Violation"]
    #[inline(always)]
    #[must_use]
    pub fn ram_fetch_viol(&mut self) -> RamFetchViolW<RamLkStsSpec> {
        RamFetchViolW::new(self, 1)
    }
}
#[doc = "RAM Lock Status Register (RAM_LK_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_lk_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_lk_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamLkStsSpec;
impl crate::RegisterSpec for RamLkStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_lk_sts::R`](R) reader structure"]
impl crate::Readable for RamLkStsSpec {}
#[doc = "`write(|w| ..)` method takes [`ram_lk_sts::W`](W) writer structure"]
impl crate::Writable for RamLkStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RAM_LK_STS to value 0"]
impl crate::Resettable for RamLkStsSpec {
    const RESET_VALUE: u8 = 0;
}
