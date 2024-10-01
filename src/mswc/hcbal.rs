#[doc = "Register `HCBAL` reader"]
pub type R = crate::R<HcbalSpec>;
#[doc = "Register `HCBAL` writer"]
pub type W = crate::W<HcbalSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Configuration Base Address Low Register (HCBAL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcbalSpec;
impl crate::RegisterSpec for HcbalSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hcbal::R`](R) reader structure"]
impl crate::Readable for HcbalSpec {}
#[doc = "`write(|w| ..)` method takes [`hcbal::W`](W) writer structure"]
impl crate::Writable for HcbalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HCBAL to value 0"]
impl crate::Resettable for HcbalSpec {
    const RESET_VALUE: u8 = 0;
}
