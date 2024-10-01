#[doc = "Register `HEAD_BASEH` reader"]
pub type R = crate::R<HeadBasehSpec>;
#[doc = "Register `HEAD_BASEH` writer"]
pub type W = crate::W<HeadBasehSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Header Base High Register (HEAD_BASEH)\n\nYou can [`read`](crate::Reg::read) this register and get [`head_baseh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`head_baseh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeadBasehSpec;
impl crate::RegisterSpec for HeadBasehSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`head_baseh::R`](R) reader structure"]
impl crate::Readable for HeadBasehSpec {}
#[doc = "`write(|w| ..)` method takes [`head_baseh::W`](W) writer structure"]
impl crate::Writable for HeadBasehSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HEAD_BASEH to value 0"]
impl crate::Resettable for HeadBasehSpec {
    const RESET_VALUE: u16 = 0;
}
