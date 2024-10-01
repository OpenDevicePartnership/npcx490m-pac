#[doc = "Register `UPSRn` reader"]
pub type R = crate::R<UpsrnSpec>;
#[doc = "Register `UPSRn` writer"]
pub type W = crate::W<UpsrnSpec>;
#[doc = "Field `UDIV10_8` reader - Baud Rate Divisor"]
pub type Udiv10_8R = crate::FieldReader;
#[doc = "Field `UDIV10_8` writer - Baud Rate Divisor"]
pub type Udiv10_8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UPSC` reader - Prescaler Select"]
pub type UpscR = crate::FieldReader;
#[doc = "Field `UPSC` writer - Prescaler Select"]
pub type UpscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Baud Rate Divisor"]
    #[inline(always)]
    pub fn udiv10_8(&self) -> Udiv10_8R {
        Udiv10_8R::new(self.bits & 7)
    }
    #[doc = "Bits 3:7 - Prescaler Select"]
    #[inline(always)]
    pub fn upsc(&self) -> UpscR {
        UpscR::new((self.bits >> 3) & 0x1f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPSRn")
            .field("udiv10_8", &self.udiv10_8())
            .field("upsc", &self.upsc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Baud Rate Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn udiv10_8(&mut self) -> Udiv10_8W<UpsrnSpec> {
        Udiv10_8W::new(self, 0)
    }
    #[doc = "Bits 3:7 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn upsc(&mut self) -> UpscW<UpsrnSpec> {
        UpscW::new(self, 3)
    }
}
#[doc = "Baud Rate Prescaler Register (UPSRn)\n\nYou can [`read`](crate::Reg::read) this register and get [`upsrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upsrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpsrnSpec;
impl crate::RegisterSpec for UpsrnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upsrn::R`](R) reader structure"]
impl crate::Readable for UpsrnSpec {}
#[doc = "`write(|w| ..)` method takes [`upsrn::W`](W) writer structure"]
impl crate::Writable for UpsrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPSRn to value 0"]
impl crate::Resettable for UpsrnSpec {
    const RESET_VALUE: u8 = 0;
}
