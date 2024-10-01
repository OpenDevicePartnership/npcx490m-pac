#[doc = "Register `TnCFG` reader"]
pub type R = crate::R<TnCfgSpec>;
#[doc = "Register `TnCFG` writer"]
pub type W = crate::W<TnCfgSpec>;
#[doc = "Field `TADBEN` reader - TAn Debounce Enable"]
pub type TadbenR = crate::BitReader;
#[doc = "Field `TADBEN` writer - TAn Debounce Enable"]
pub type TadbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBDBEN` reader - TBn Debounce Enable"]
pub type TbdbenR = crate::BitReader;
#[doc = "Field `TBDBEN` writer - TBn Debounce Enable"]
pub type TbdbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - TAn Debounce Enable"]
    #[inline(always)]
    pub fn tadben(&self) -> TadbenR {
        TadbenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TBn Debounce Enable"]
    #[inline(always)]
    pub fn tbdben(&self) -> TbdbenR {
        TbdbenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnCFG")
            .field("tadben", &self.tadben())
            .field("tbdben", &self.tbdben())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - TAn Debounce Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tadben(&mut self) -> TadbenW<TnCfgSpec> {
        TadbenW::new(self, 6)
    }
    #[doc = "Bit 7 - TBn Debounce Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbdben(&mut self) -> TbdbenW<TnCfgSpec> {
        TbdbenW::new(self, 7)
    }
}
#[doc = "Timer Configuration Register (TnCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCfgSpec;
impl crate::RegisterSpec for TnCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_cfg::R`](R) reader structure"]
impl crate::Readable for TnCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_cfg::W`](W) writer structure"]
impl crate::Writable for TnCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnCFG to value 0"]
impl crate::Resettable for TnCfgSpec {
    const RESET_VALUE: u8 = 0;
}
