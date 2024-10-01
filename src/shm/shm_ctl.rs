#[doc = "Register `SHM_CTL` reader"]
pub type R = crate::R<ShmCtlSpec>;
#[doc = "Register `SHM_CTL` writer"]
pub type W = crate::W<ShmCtlSpec>;
#[doc = "Field `STALL_HOST` reader - Stall Host Access to Shared Memory"]
pub type StallHostR = crate::BitReader;
#[doc = "Field `STALL_HOST` writer - Stall Host Access to Shared Memory"]
pub type StallHostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Stall Host Access to Shared Memory"]
    #[inline(always)]
    pub fn stall_host(&self) -> StallHostR {
        StallHostR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHM_CTL")
            .field("stall_host", &self.stall_host())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - Stall Host Access to Shared Memory"]
    #[inline(always)]
    #[must_use]
    pub fn stall_host(&mut self) -> StallHostW<ShmCtlSpec> {
        StallHostW::new(self, 6)
    }
}
#[doc = "Shared Memory Control Register (SHM_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`shm_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shm_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShmCtlSpec;
impl crate::RegisterSpec for ShmCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shm_ctl::R`](R) reader structure"]
impl crate::Readable for ShmCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`shm_ctl::W`](W) writer structure"]
impl crate::Writable for ShmCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHM_CTL to value 0"]
impl crate::Resettable for ShmCtlSpec {
    const RESET_VALUE: u8 = 0;
}
