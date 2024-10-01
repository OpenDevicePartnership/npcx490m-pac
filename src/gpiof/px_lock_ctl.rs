#[doc = "Register `PxLOCK_CTL` reader"]
pub type R = crate::R<PxLockCtlSpec>;
#[doc = "Register `PxLOCK_CTL` writer"]
pub type W = crate::W<PxLockCtlSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port GPIOx Lock Control Register (PxLOCK_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_lock_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_lock_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxLockCtlSpec;
impl crate::RegisterSpec for PxLockCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_lock_ctl::R`](R) reader structure"]
impl crate::Readable for PxLockCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`px_lock_ctl::W`](W) writer structure"]
impl crate::Writable for PxLockCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxLOCK_CTL to value 0"]
impl crate::Resettable for PxLockCtlSpec {
    const RESET_VALUE: u8 = 0;
}
