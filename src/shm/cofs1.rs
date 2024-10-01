#[doc = "Register `COFS1` reader"]
pub type R = crate::R<Cofs1Spec>;
#[doc = "Register `COFS1` writer"]
pub type W = crate::W<Cofs1Spec>;
#[doc = "Field `COFS1_DAT` reader - Core Offset in Window 1, Data"]
pub type Cofs1DatR = crate::FieldReader<u16>;
#[doc = "Field `COFS1_DAT` writer - Core Offset in Window 1, Data"]
pub type Cofs1DatW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Core Offset in Window 1, Data"]
    #[inline(always)]
    pub fn cofs1_dat(&self) -> Cofs1DatR {
        Cofs1DatR::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COFS1")
            .field("cofs1_dat", &self.cofs1_dat())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Core Offset in Window 1, Data"]
    #[inline(always)]
    #[must_use]
    pub fn cofs1_dat(&mut self) -> Cofs1DatW<Cofs1Spec> {
        Cofs1DatW::new(self, 0)
    }
}
#[doc = "Core_Offset in Window 1 Address Register (COFS1)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cofs1Spec;
impl crate::RegisterSpec for Cofs1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cofs1::R`](R) reader structure"]
impl crate::Readable for Cofs1Spec {}
#[doc = "`write(|w| ..)` method takes [`cofs1::W`](W) writer structure"]
impl crate::Writable for Cofs1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COFS1 to value 0"]
impl crate::Resettable for Cofs1Spec {
    const RESET_VALUE: u16 = 0;
}
