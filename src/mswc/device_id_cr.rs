#[doc = "Register `DEVICE_ID_CR` reader"]
pub type R = crate::R<DeviceIdCrSpec>;
#[doc = "Register `DEVICE_ID_CR` writer"]
pub type W = crate::W<DeviceIdCrSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DEVICE_ID Core Access Register (DEVICE_ID_CR)\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`device_id_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceIdCrSpec;
impl crate::RegisterSpec for DeviceIdCrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`device_id_cr::R`](R) reader structure"]
impl crate::Readable for DeviceIdCrSpec {}
#[doc = "`write(|w| ..)` method takes [`device_id_cr::W`](W) writer structure"]
impl crate::Writable for DeviceIdCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVICE_ID_CR to value 0"]
impl crate::Resettable for DeviceIdCrSpec {
    const RESET_VALUE: u8 = 0;
}
