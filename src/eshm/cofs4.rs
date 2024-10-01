#[doc = "Register `COFS4` reader"]
pub type R = crate::R<Cofs4Spec>;
#[doc = "Register `COFS4` writer"]
pub type W = crate::W<Cofs4Spec>;
#[doc = "Field `COFS4_DAT` reader - Core_Offset in Window 4, Data"]
pub type Cofs4DatR = crate::FieldReader<u16>;
#[doc = "Field `COFS4_DAT` writer - Core_Offset in Window 4, Data"]
pub type Cofs4DatW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Core_Offset in Window 4, Data"]
    #[inline(always)]
    pub fn cofs4_dat(&self) -> Cofs4DatR {
        Cofs4DatR::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COFS4")
            .field("cofs4_dat", &self.cofs4_dat())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Core_Offset in Window 4, Data"]
    #[inline(always)]
    #[must_use]
    pub fn cofs4_dat(&mut self) -> Cofs4DatW<Cofs4Spec> {
        Cofs4DatW::new(self, 0)
    }
}
#[doc = "Core_Offset in Window 4 Address (COFS4)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cofs4Spec;
impl crate::RegisterSpec for Cofs4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cofs4::R`](R) reader structure"]
impl crate::Readable for Cofs4Spec {}
#[doc = "`write(|w| ..)` method takes [`cofs4::W`](W) writer structure"]
impl crate::Writable for Cofs4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COFS4 to value 0"]
impl crate::Resettable for Cofs4Spec {
    const RESET_VALUE: u16 = 0;
}
