#[doc = "Register `THEN` reader"]
pub type R = crate::R<ThenSpec>;
#[doc = "Register `THEN` writer"]
pub type W = crate::W<ThenSpec>;
#[doc = "Field `THEN6_1` reader - Threshold Enable"]
pub type Then6_1R = crate::FieldReader;
#[doc = "Field `THEN6_1` writer - Threshold Enable"]
pub type Then6_1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Threshold Enable"]
    #[inline(always)]
    pub fn then6_1(&self) -> Then6_1R {
        Then6_1R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THEN")
            .field("then6_1", &self.then6_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn then6_1(&mut self) -> Then6_1W<ThenSpec> {
        Then6_1W::new(self, 0)
    }
}
#[doc = "Threshold Enable Register (THEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`then::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`then::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThenSpec;
impl crate::RegisterSpec for ThenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`then::R`](R) reader structure"]
impl crate::Readable for ThenSpec {}
#[doc = "`write(|w| ..)` method takes [`then::W`](W) writer structure"]
impl crate::Writable for ThenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets THEN to value 0"]
impl crate::Resettable for ThenSpec {
    const RESET_VALUE: u16 = 0;
}
