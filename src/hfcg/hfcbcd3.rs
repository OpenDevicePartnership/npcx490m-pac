#[doc = "Register `HFCBCD3` reader"]
pub type R = crate::R<Hfcbcd3Spec>;
#[doc = "Register `HFCBCD3` writer"]
pub type W = crate::W<Hfcbcd3Spec>;
#[doc = "Field `MCLKD1_SL` reader - MCLK Division for I3CI1 Select"]
pub type Mclkd1SlR = crate::BitReader;
#[doc = "Field `MCLKD1_SL` writer - MCLK Division for I3CI1 Select"]
pub type Mclkd1SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCLKD2_SL` reader - MCLK Division for I3CI2 Select"]
pub type Mclkd2SlR = crate::BitReader;
#[doc = "Field `MCLKD2_SL` writer - MCLK Division for I3CI2 Select"]
pub type Mclkd2SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCLKD3_SL` reader - MCLK Division for I3CI3 Select"]
pub type Mclkd3SlR = crate::BitReader;
#[doc = "Field `MCLKD3_SL` writer - MCLK Division for I3CI3 Select"]
pub type Mclkd3SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCLK Division for I3CI1 Select"]
    #[inline(always)]
    pub fn mclkd1_sl(&self) -> Mclkd1SlR {
        Mclkd1SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK Division for I3CI2 Select"]
    #[inline(always)]
    pub fn mclkd2_sl(&self) -> Mclkd2SlR {
        Mclkd2SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK Division for I3CI3 Select"]
    #[inline(always)]
    pub fn mclkd3_sl(&self) -> Mclkd3SlR {
        Mclkd3SlR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCBCD3")
            .field("mclkd1_sl", &self.mclkd1_sl())
            .field("mclkd2_sl", &self.mclkd2_sl())
            .field("mclkd3_sl", &self.mclkd3_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MCLK Division for I3CI1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn mclkd1_sl(&mut self) -> Mclkd1SlW<Hfcbcd3Spec> {
        Mclkd1SlW::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK Division for I3CI2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn mclkd2_sl(&mut self) -> Mclkd2SlW<Hfcbcd3Spec> {
        Mclkd2SlW::new(self, 1)
    }
    #[doc = "Bit 2 - MCLK Division for I3CI3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn mclkd3_sl(&mut self) -> Mclkd3SlW<Hfcbcd3Spec> {
        Mclkd3SlW::new(self, 2)
    }
}
#[doc = "HFCG Bus Clock Dividers 3 Register (HFCBCD3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfcbcd3Spec;
impl crate::RegisterSpec for Hfcbcd3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcbcd3::R`](R) reader structure"]
impl crate::Readable for Hfcbcd3Spec {}
#[doc = "`write(|w| ..)` method takes [`hfcbcd3::W`](W) writer structure"]
impl crate::Writable for Hfcbcd3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCBCD3 to value 0"]
impl crate::Resettable for Hfcbcd3Spec {
    const RESET_VALUE: u8 = 0;
}
