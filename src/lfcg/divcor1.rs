#[doc = "Register `DIVCOR1` reader"]
pub type R = crate::R<Divcor1Spec>;
#[doc = "Register `DIVCOR1` writer"]
pub type W = crate::W<Divcor1Spec>;
#[doc = "Field `DIVCOR1` reader - Divisor Correction Value 1"]
pub type Divcor1R = crate::FieldReader<u16>;
#[doc = "Field `DIVCOR1` writer - Divisor Correction Value 1"]
pub type Divcor1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Divisor Correction Value 1"]
    #[inline(always)]
    pub fn divcor1(&self) -> Divcor1R {
        Divcor1R::new(self.bits & 0x01ff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVCOR1")
            .field("divcor1", &self.divcor1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Divisor Correction Value 1"]
    #[inline(always)]
    pub fn divcor1(&mut self) -> Divcor1W<Divcor1Spec> {
        Divcor1W::new(self, 0)
    }
}
#[doc = "Divisor Correction Value 1 Register (DIVCOR1)\n\nYou can [`read`](crate::Reg::read) this register and get [`divcor1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcor1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Divcor1Spec;
impl crate::RegisterSpec for Divcor1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`divcor1::R`](R) reader structure"]
impl crate::Readable for Divcor1Spec {}
#[doc = "`write(|w| ..)` method takes [`divcor1::W`](W) writer structure"]
impl crate::Writable for Divcor1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DIVCOR1 to value 0"]
impl crate::Resettable for Divcor1Spec {
    const RESET_VALUE: u16 = 0;
}
