#[doc = "Register `KBSIN` reader"]
pub type R = crate::R<KbsinSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Keyboard Scan In Register (KBSIN)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsinSpec;
impl crate::RegisterSpec for KbsinSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbsin::R`](R) reader structure"]
impl crate::Readable for KbsinSpec {}
#[doc = "`reset()` method sets KBSIN to value 0"]
impl crate::Resettable for KbsinSpec {
    const RESET_VALUE: u8 = 0;
}
