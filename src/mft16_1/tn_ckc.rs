#[doc = "Register `TnCKC` reader"]
pub type R = crate::R<TnCkcSpec>;
#[doc = "Register `TnCKC` writer"]
pub type W = crate::W<TnCkcSpec>;
#[doc = "Field `C1CSEL` reader - Counter 1 Clock Select"]
pub type C1cselR = crate::FieldReader;
#[doc = "Field `C1CSEL` writer - Counter 1 Clock Select"]
pub type C1cselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C2CSEL` reader - Counter 2 Clock Select"]
pub type C2cselR = crate::FieldReader;
#[doc = "Field `C2CSEL` writer - Counter 2 Clock Select"]
pub type C2cselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLS_ACC_CLK` reader - Pulse Accumulate Clock Select"]
pub type PlsAccClkR = crate::BitReader;
#[doc = "Field `PLS_ACC_CLK` writer - Pulse Accumulate Clock Select"]
pub type PlsAccClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOW_PWR` reader - Low-Power Mode Select"]
pub type LowPwrR = crate::BitReader;
#[doc = "Field `LOW_PWR` writer - Low-Power Mode Select"]
pub type LowPwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Counter 1 Clock Select"]
    #[inline(always)]
    pub fn c1csel(&self) -> C1cselR {
        C1cselR::new(self.bits & 7)
    }
    #[doc = "Bits 3:5 - Counter 2 Clock Select"]
    #[inline(always)]
    pub fn c2csel(&self) -> C2cselR {
        C2cselR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - Pulse Accumulate Clock Select"]
    #[inline(always)]
    pub fn pls_acc_clk(&self) -> PlsAccClkR {
        PlsAccClkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-Power Mode Select"]
    #[inline(always)]
    pub fn low_pwr(&self) -> LowPwrR {
        LowPwrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnCKC")
            .field("c1csel", &self.c1csel())
            .field("c2csel", &self.c2csel())
            .field("pls_acc_clk", &self.pls_acc_clk())
            .field("low_pwr", &self.low_pwr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Counter 1 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn c1csel(&mut self) -> C1cselW<TnCkcSpec> {
        C1cselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Counter 2 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn c2csel(&mut self) -> C2cselW<TnCkcSpec> {
        C2cselW::new(self, 3)
    }
    #[doc = "Bit 6 - Pulse Accumulate Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pls_acc_clk(&mut self) -> PlsAccClkW<TnCkcSpec> {
        PlsAccClkW::new(self, 6)
    }
    #[doc = "Bit 7 - Low-Power Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn low_pwr(&mut self) -> LowPwrW<TnCkcSpec> {
        LowPwrW::new(self, 7)
    }
}
#[doc = "Clock Unit Control Register (TnCKC)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_ckc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_ckc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCkcSpec;
impl crate::RegisterSpec for TnCkcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_ckc::R`](R) reader structure"]
impl crate::Readable for TnCkcSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_ckc::W`](W) writer structure"]
impl crate::Writable for TnCkcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnCKC to value 0"]
impl crate::Resettable for TnCkcSpec {
    const RESET_VALUE: u8 = 0;
}
