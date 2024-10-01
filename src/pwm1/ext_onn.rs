#[doc = "Register `EXT_ONn` reader"]
pub type R = crate::R<ExtOnnSpec>;
#[doc = "Register `EXT_ONn` writer"]
pub type W = crate::W<ExtOnnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Extend ON Time Register (EXT_ONn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_onn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_onn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtOnnSpec;
impl crate::RegisterSpec for ExtOnnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ext_onn::R`](R) reader structure"]
impl crate::Readable for ExtOnnSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_onn::W`](W) writer structure"]
impl crate::Writable for ExtOnnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EXT_ONn to value 0"]
impl crate::Resettable for ExtOnnSpec {
    const RESET_VALUE: u8 = 0;
}
