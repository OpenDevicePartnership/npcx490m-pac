#[doc = "Register `HEAD_BASEL` reader"]
pub type R = crate::R<HeadBaselSpec>;
#[doc = "Register `HEAD_BASEL` writer"]
pub type W = crate::W<HeadBaselSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Header Base Low Register (HEAD_BASEL)\n\nYou can [`read`](crate::Reg::read) this register and get [`head_basel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`head_basel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeadBaselSpec;
impl crate::RegisterSpec for HeadBaselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`head_basel::R`](R) reader structure"]
impl crate::Readable for HeadBaselSpec {}
#[doc = "`write(|w| ..)` method takes [`head_basel::W`](W) writer structure"]
impl crate::Writable for HeadBaselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HEAD_BASEL to value 0"]
impl crate::Resettable for HeadBaselSpec {
    const RESET_VALUE: u16 = 0;
}
