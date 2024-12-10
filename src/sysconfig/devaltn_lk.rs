#[doc = "Register `DEVALTN_LK` reader"]
pub type R = crate::R<DevaltnLkSpec>;
#[doc = "Register `DEVALTN_LK` writer"]
pub type W = crate::W<DevaltnLkSpec>;
#[doc = "Field `I3C1_SL_LK` reader - I3C1 Signals Select Lock"]
pub type I3c1SlLkR = crate::BitReader;
#[doc = "Field `I3C1_SL_LK` writer - I3C1 Signals Select Lock"]
pub type I3c1SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2_SL_LK` reader - I3C2 Signals Select Lock"]
pub type I3c2SlLkR = crate::BitReader;
#[doc = "Field `I3C2_SL_LK` writer - I3C2 Signals Select Lock"]
pub type I3c2SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3_SL_LK` reader - I3C3 Signals Select Lock"]
pub type I3c3SlLkR = crate::BitReader;
#[doc = "Field `I3C3_SL_LK` writer - I3C3 Signals Select Lock"]
pub type I3c3SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I3C1 Signals Select Lock"]
    #[inline(always)]
    pub fn i3c1_sl_lk(&self) -> I3c1SlLkR {
        I3c1SlLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I3C2 Signals Select Lock"]
    #[inline(always)]
    pub fn i3c2_sl_lk(&self) -> I3c2SlLkR {
        I3c2SlLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I3C3 Signals Select Lock"]
    #[inline(always)]
    pub fn i3c3_sl_lk(&self) -> I3c3SlLkR {
        I3c3SlLkR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTN_LK")
            .field("i3c1_sl_lk", &self.i3c1_sl_lk())
            .field("i3c2_sl_lk", &self.i3c2_sl_lk())
            .field("i3c3_sl_lk", &self.i3c3_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I3C1 Signals Select Lock"]
    #[inline(always)]
    pub fn i3c1_sl_lk(&mut self) -> I3c1SlLkW<DevaltnLkSpec> {
        I3c1SlLkW::new(self, 0)
    }
    #[doc = "Bit 1 - I3C2 Signals Select Lock"]
    #[inline(always)]
    pub fn i3c2_sl_lk(&mut self) -> I3c2SlLkW<DevaltnLkSpec> {
        I3c2SlLkW::new(self, 1)
    }
    #[doc = "Bit 2 - I3C3 Signals Select Lock"]
    #[inline(always)]
    pub fn i3c3_sl_lk(&mut self) -> I3c3SlLkW<DevaltnLkSpec> {
        I3c3SlLkW::new(self, 2)
    }
}
#[doc = "Device Alternate Function N Lock Register (DEVALTN_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltn_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltn_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltnLkSpec;
impl crate::RegisterSpec for DevaltnLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltn_lk::R`](R) reader structure"]
impl crate::Readable for DevaltnLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltn_lk::W`](W) writer structure"]
impl crate::Writable for DevaltnLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTN_LK to value 0"]
impl crate::Resettable for DevaltnLkSpec {
    const RESET_VALUE: u8 = 0;
}
