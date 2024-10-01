#[doc = "Register `DEVALTN` reader"]
pub type R = crate::R<DevaltnSpec>;
#[doc = "Register `DEVALTN` writer"]
pub type W = crate::W<DevaltnSpec>;
#[doc = "Field `I3C1_SL` reader - I3C1 Signals Select"]
pub type I3c1SlR = crate::BitReader;
#[doc = "Field `I3C1_SL` writer - I3C1 Signals Select"]
pub type I3c1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2_SL` reader - I3C2 Signals Select"]
pub type I3c2SlR = crate::BitReader;
#[doc = "Field `I3C2_SL` writer - I3C2 Signals Select"]
pub type I3c2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3_SL` reader - I3C3 Signals Select"]
pub type I3c3SlR = crate::BitReader;
#[doc = "Field `I3C3_SL` writer - I3C3 Signals Select"]
pub type I3c3SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I3C1 Signals Select"]
    #[inline(always)]
    pub fn i3c1_sl(&self) -> I3c1SlR {
        I3c1SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I3C2 Signals Select"]
    #[inline(always)]
    pub fn i3c2_sl(&self) -> I3c2SlR {
        I3c2SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I3C3 Signals Select"]
    #[inline(always)]
    pub fn i3c3_sl(&self) -> I3c3SlR {
        I3c3SlR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTN")
            .field("i3c1_sl", &self.i3c1_sl())
            .field("i3c2_sl", &self.i3c2_sl())
            .field("i3c3_sl", &self.i3c3_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I3C1 Signals Select"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1_sl(&mut self) -> I3c1SlW<DevaltnSpec> {
        I3c1SlW::new(self, 0)
    }
    #[doc = "Bit 1 - I3C2 Signals Select"]
    #[inline(always)]
    #[must_use]
    pub fn i3c2_sl(&mut self) -> I3c2SlW<DevaltnSpec> {
        I3c2SlW::new(self, 1)
    }
    #[doc = "Bit 2 - I3C3 Signals Select"]
    #[inline(always)]
    #[must_use]
    pub fn i3c3_sl(&mut self) -> I3c3SlW<DevaltnSpec> {
        I3c3SlW::new(self, 2)
    }
}
#[doc = "Device Alternate Function N Register (DEVALTN)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltnSpec;
impl crate::RegisterSpec for DevaltnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltn::R`](R) reader structure"]
impl crate::Readable for DevaltnSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltn::W`](W) writer structure"]
impl crate::Writable for DevaltnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTN to value 0"]
impl crate::Resettable for DevaltnSpec {
    const RESET_VALUE: u8 = 0;
}
