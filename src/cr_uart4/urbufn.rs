#[doc = "Register `URBUFn` reader"]
pub type R = crate::R<UrbufnSpec>;
#[doc = "Register `URBUFn` writer"]
pub type W = crate::W<UrbufnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Receive Data Buffer Register (URBUFn)\n\nYou can [`read`](crate::Reg::read) this register and get [`urbufn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urbufn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrbufnSpec;
impl crate::RegisterSpec for UrbufnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`urbufn::R`](R) reader structure"]
impl crate::Readable for UrbufnSpec {}
#[doc = "`write(|w| ..)` method takes [`urbufn::W`](W) writer structure"]
impl crate::Writable for UrbufnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets URBUFn to value 0"]
impl crate::Resettable for UrbufnSpec {
    const RESET_VALUE: u8 = 0;
}
