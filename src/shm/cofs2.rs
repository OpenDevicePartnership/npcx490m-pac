#[doc = "Register `COFS2` reader"]
pub type R = crate::R<Cofs2Spec>;
#[doc = "Register `COFS2` writer"]
pub type W = crate::W<Cofs2Spec>;
#[doc = "Field `COFS2_DAT` reader - Core_Offset in Window 2, Data"]
pub type Cofs2DatR = crate::FieldReader<u16>;
#[doc = "Field `COFS2_DAT` writer - Core_Offset in Window 2, Data"]
pub type Cofs2DatW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Core_Offset in Window 2, Data"]
    #[inline(always)]
    pub fn cofs2_dat(&self) -> Cofs2DatR {
        Cofs2DatR::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COFS2")
            .field("cofs2_dat", &self.cofs2_dat())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Core_Offset in Window 2, Data"]
    #[inline(always)]
    pub fn cofs2_dat(&mut self) -> Cofs2DatW<Cofs2Spec> {
        Cofs2DatW::new(self, 0)
    }
}
#[doc = "Core_Offset in Window 2 Address Register (COFS2)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cofs2Spec;
impl crate::RegisterSpec for Cofs2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cofs2::R`](R) reader structure"]
impl crate::Readable for Cofs2Spec {}
#[doc = "`write(|w| ..)` method takes [`cofs2::W`](W) writer structure"]
impl crate::Writable for Cofs2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COFS2 to value 0"]
impl crate::Resettable for Cofs2Spec {
    const RESET_VALUE: u16 = 0;
}
