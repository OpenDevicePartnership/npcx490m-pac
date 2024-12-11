#[doc = "Register `COFS3` reader"]
pub type R = crate::R<Cofs3Spec>;
#[doc = "Register `COFS3` writer"]
pub type W = crate::W<Cofs3Spec>;
#[doc = "Field `COFS3_DAT` reader - Core Offset in Window 3, Data"]
pub type Cofs3DatR = crate::FieldReader<u16>;
#[doc = "Field `COFS3_DAT` writer - Core Offset in Window 3, Data"]
pub type Cofs3DatW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Core Offset in Window 3, Data"]
    #[inline(always)]
    pub fn cofs3_dat(&self) -> Cofs3DatR {
        Cofs3DatR::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COFS3")
            .field("cofs3_dat", &self.cofs3_dat())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Core Offset in Window 3, Data"]
    #[inline(always)]
    pub fn cofs3_dat(&mut self) -> Cofs3DatW<Cofs3Spec> {
        Cofs3DatW::new(self, 0)
    }
}
#[doc = "Core_Offset in Window 3 Address (COFS3)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cofs3Spec;
impl crate::RegisterSpec for Cofs3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cofs3::R`](R) reader structure"]
impl crate::Readable for Cofs3Spec {}
#[doc = "`write(|w| ..)` method takes [`cofs3::W`](W) writer structure"]
impl crate::Writable for Cofs3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COFS3 to value 0"]
impl crate::Resettable for Cofs3Spec {
    const RESET_VALUE: u16 = 0;
}
