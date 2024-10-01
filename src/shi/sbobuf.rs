#[doc = "Register `SBOBUF` reader"]
pub type R = crate::R<SbobufSpec>;
#[doc = "Register `SBOBUF` writer"]
pub type W = crate::W<SbobufSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Single Byte Output Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`sbobuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbobuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbobufSpec;
impl crate::RegisterSpec for SbobufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sbobuf::R`](R) reader structure"]
impl crate::Readable for SbobufSpec {}
#[doc = "`write(|w| ..)` method takes [`sbobuf::W`](W) writer structure"]
impl crate::Writable for SbobufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SBOBUF to value 0"]
impl crate::Resettable for SbobufSpec {
    const RESET_VALUE: u8 = 0;
}
