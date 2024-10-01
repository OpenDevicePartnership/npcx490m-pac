#[doc = "Register `ADCCS2` reader"]
pub type R = crate::R<Adccs2Spec>;
#[doc = "Register `ADCCS2` writer"]
pub type W = crate::W<Adccs2Spec>;
#[doc = "Field `CC25_16` reader - Convert Channel n"]
pub type Cc25_16R = crate::FieldReader<u16>;
#[doc = "Field `CC25_16` writer - Convert Channel n"]
pub type Cc25_16W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CC29` reader - Convert Channel 29"]
pub type Cc29R = crate::BitReader;
#[doc = "Field `CC29` writer - Convert Channel 29"]
pub type Cc29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC30` reader - Convert Channel 30"]
pub type Cc30R = crate::BitReader;
#[doc = "Field `CC30` writer - Convert Channel 30"]
pub type Cc30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC31` reader - Convert Channel 31"]
pub type Cc31R = crate::BitReader;
#[doc = "Field `CC31` writer - Convert Channel 31"]
pub type Cc31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Convert Channel n"]
    #[inline(always)]
    pub fn cc25_16(&self) -> Cc25_16R {
        Cc25_16R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 13 - Convert Channel 29"]
    #[inline(always)]
    pub fn cc29(&self) -> Cc29R {
        Cc29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Convert Channel 30"]
    #[inline(always)]
    pub fn cc30(&self) -> Cc30R {
        Cc30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Convert Channel 31"]
    #[inline(always)]
    pub fn cc31(&self) -> Cc31R {
        Cc31R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCS2")
            .field("cc25_16", &self.cc25_16())
            .field("cc29", &self.cc29())
            .field("cc30", &self.cc30())
            .field("cc31", &self.cc31())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Convert Channel n"]
    #[inline(always)]
    #[must_use]
    pub fn cc25_16(&mut self) -> Cc25_16W<Adccs2Spec> {
        Cc25_16W::new(self, 0)
    }
    #[doc = "Bit 13 - Convert Channel 29"]
    #[inline(always)]
    #[must_use]
    pub fn cc29(&mut self) -> Cc29W<Adccs2Spec> {
        Cc29W::new(self, 13)
    }
    #[doc = "Bit 14 - Convert Channel 30"]
    #[inline(always)]
    #[must_use]
    pub fn cc30(&mut self) -> Cc30W<Adccs2Spec> {
        Cc30W::new(self, 14)
    }
    #[doc = "Bit 15 - Convert Channel 31"]
    #[inline(always)]
    #[must_use]
    pub fn cc31(&mut self) -> Cc31W<Adccs2Spec> {
        Cc31W::new(self, 15)
    }
}
#[doc = "ADC Scan Channels Select 2 Register (ADCCS2)\n\nYou can [`read`](crate::Reg::read) this register and get [`adccs2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccs2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adccs2Spec;
impl crate::RegisterSpec for Adccs2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adccs2::R`](R) reader structure"]
impl crate::Readable for Adccs2Spec {}
#[doc = "`write(|w| ..)` method takes [`adccs2::W`](W) writer structure"]
impl crate::Writable for Adccs2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADCCS2 to value 0"]
impl crate::Resettable for Adccs2Spec {
    const RESET_VALUE: u16 = 0;
}
