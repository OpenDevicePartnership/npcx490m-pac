#[doc = "Register `URBUFn` reader"]
pub type R = crate::R<UrbufnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Receive Data Buffer Register (URBUFn)\n\nYou can [`read`](crate::Reg::read) this register and get [`urbufn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrbufnSpec;
impl crate::RegisterSpec for UrbufnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`urbufn::R`](R) reader structure"]
impl crate::Readable for UrbufnSpec {}
#[doc = "`reset()` method sets URBUFn to value 0"]
impl crate::Resettable for UrbufnSpec {
    const RESET_VALUE: u8 = 0;
}
