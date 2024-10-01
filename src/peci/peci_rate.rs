#[doc = "Register `PECI_RATE` reader"]
pub type R = crate::R<PeciRateSpec>;
#[doc = "Register `PECI_RATE` writer"]
pub type W = crate::W<PeciRateSpec>;
#[doc = "Field `MAX_BIT_RATE` reader - Maximum Bit Rate"]
pub type MaxBitRateR = crate::FieldReader;
#[doc = "Field `MAX_BIT_RATE` writer - Maximum Bit Rate"]
pub type MaxBitRateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EHSP_EN` reader - Enhanced High-Speed Enable"]
pub type EhspEnR = crate::BitReader;
#[doc = "Field `EHSP_EN` writer - Enhanced High-Speed Enable"]
pub type EhspEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLOAD_EN` reader - PECI High Load Enable"]
pub type HloadEnR = crate::BitReader;
#[doc = "Field `HLOAD_EN` writer - PECI High Load Enable"]
pub type HloadEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Maximum Bit Rate"]
    #[inline(always)]
    pub fn max_bit_rate(&self) -> MaxBitRateR {
        MaxBitRateR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Enhanced High-Speed Enable"]
    #[inline(always)]
    pub fn ehsp_en(&self) -> EhspEnR {
        EhspEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PECI High Load Enable"]
    #[inline(always)]
    pub fn hload_en(&self) -> HloadEnR {
        HloadEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECI_RATE")
            .field("max_bit_rate", &self.max_bit_rate())
            .field("ehsp_en", &self.ehsp_en())
            .field("hload_en", &self.hload_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Maximum Bit Rate"]
    #[inline(always)]
    #[must_use]
    pub fn max_bit_rate(&mut self) -> MaxBitRateW<PeciRateSpec> {
        MaxBitRateW::new(self, 0)
    }
    #[doc = "Bit 6 - Enhanced High-Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ehsp_en(&mut self) -> EhspEnW<PeciRateSpec> {
        EhspEnW::new(self, 6)
    }
    #[doc = "Bit 7 - PECI High Load Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hload_en(&mut self) -> HloadEnW<PeciRateSpec> {
        HloadEnW::new(self, 7)
    }
}
#[doc = "PECI Transfer Rate Register (PECI_RATE)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_rate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_rate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciRateSpec;
impl crate::RegisterSpec for PeciRateSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_rate::R`](R) reader structure"]
impl crate::Readable for PeciRateSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_rate::W`](W) writer structure"]
impl crate::Writable for PeciRateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_RATE to value 0"]
impl crate::Resettable for PeciRateSpec {
    const RESET_VALUE: u8 = 0;
}
