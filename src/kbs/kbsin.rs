#[doc = "Register `KBSIN` reader"]
pub type R = crate::R<KbsinSpec>;
#[doc = "Register `KBSIN` writer"]
pub type W = crate::W<KbsinSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan In Register (KBSIN)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsinSpec;
impl crate::RegisterSpec for KbsinSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbsin::R`](R) reader structure"]
impl crate::Readable for KbsinSpec {}
#[doc = "`write(|w| ..)` method takes [`kbsin::W`](W) writer structure"]
impl crate::Writable for KbsinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBSIN to value 0"]
impl crate::Resettable for KbsinSpec {
    const RESET_VALUE: u8 = 0;
}
