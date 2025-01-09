#[doc = "Register `DIVCOR2` reader"]
pub type R = crate::R<Divcor2Spec>;
#[doc = "Register `DIVCOR2` writer"]
pub type W = crate::W<Divcor2Spec>;
#[doc = "Field `DIVCOR2` reader - Divisor Correction Value 2"]
pub type Divcor2R = crate::FieldReader<u16>;
#[doc = "Field `DIVCOR2` writer - Divisor Correction Value 2"]
pub type Divcor2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Divisor Correction Value 2"]
    #[inline(always)]
    pub fn divcor2(&self) -> Divcor2R {
        Divcor2R::new(self.bits & 0x01ff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVCOR2")
            .field("divcor2", &self.divcor2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Divisor Correction Value 2"]
    #[inline(always)]
    pub fn divcor2(&mut self) -> Divcor2W<Divcor2Spec> {
        Divcor2W::new(self, 0)
    }
}
#[doc = "Divisor Correction Value 2 Register (DIVCOR2)\n\nYou can [`read`](crate::Reg::read) this register and get [`divcor2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcor2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Divcor2Spec;
impl crate::RegisterSpec for Divcor2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`divcor2::R`](R) reader structure"]
impl crate::Readable for Divcor2Spec {}
#[doc = "`write(|w| ..)` method takes [`divcor2::W`](W) writer structure"]
impl crate::Writable for Divcor2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DIVCOR2 to value 0x0129"]
impl crate::Resettable for Divcor2Spec {
    const RESET_VALUE: u16 = 0x0129;
}
