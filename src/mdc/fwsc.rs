#[doc = "Register `FWSC` reader"]
pub type R = crate::R<FwscSpec>;
#[doc = "Register `FWSC` writer"]
pub type W = crate::W<FwscSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Firmware Scratch Register (FWSC)\n\nYou can [`read`](crate::Reg::read) this register and get [`fwsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwscSpec;
impl crate::RegisterSpec for FwscSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fwsc::R`](R) reader structure"]
impl crate::Readable for FwscSpec {}
#[doc = "`write(|w| ..)` method takes [`fwsc::W`](W) writer structure"]
impl crate::Writable for FwscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FWSC to value 0"]
impl crate::Resettable for FwscSpec {
    const RESET_VALUE: u8 = 0;
}
