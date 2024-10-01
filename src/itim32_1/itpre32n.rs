#[doc = "Register `ITPRE32n` reader"]
pub type R = crate::R<Itpre32nSpec>;
#[doc = "Register `ITPRE32n` writer"]
pub type W = crate::W<Itpre32nSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal Timer Prescaler Register (ITPRE32n)\n\nYou can [`read`](crate::Reg::read) this register and get [`itpre32n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itpre32n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itpre32nSpec;
impl crate::RegisterSpec for Itpre32nSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itpre32n::R`](R) reader structure"]
impl crate::Readable for Itpre32nSpec {}
#[doc = "`write(|w| ..)` method takes [`itpre32n::W`](W) writer structure"]
impl crate::Writable for Itpre32nSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITPRE32n to value 0"]
impl crate::Resettable for Itpre32nSpec {
    const RESET_VALUE: u8 = 0;
}
