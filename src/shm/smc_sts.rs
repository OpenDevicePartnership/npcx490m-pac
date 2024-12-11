#[doc = "Register `SMC_STS` reader"]
pub type R = crate::R<SmcStsSpec>;
#[doc = "Register `SMC_STS` writer"]
pub type W = crate::W<SmcStsSpec>;
#[doc = "Field `HRERR` reader - Host Read Access Error"]
pub type HrerrR = crate::BitReader;
#[doc = "Field `HRERR` writer - Host Read Access Error"]
pub type HrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWERR` reader - Host Write Access Error"]
pub type HwerrR = crate::BitReader;
#[doc = "Field `HWERR` writer - Host Write Access Error"]
pub type HwerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM_IMAW` reader - Host Semaphore Indirect Memory Access Written"]
pub type HsemImawR = crate::BitReader;
#[doc = "Field `HSEM_IMAW` writer - Host Semaphore Indirect Memory Access Written"]
pub type HsemImawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM1W` reader - Host Semaphore 1 Written"]
pub type Hsem1wR = crate::BitReader;
#[doc = "Field `HSEM1W` writer - Host Semaphore 1 Written"]
pub type Hsem1wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM2W` reader - Host Semaphore 2 Written"]
pub type Hsem2wR = crate::BitReader;
#[doc = "Field `HSEM2W` writer - Host Semaphore 2 Written"]
pub type Hsem2wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHM_ACC` reader - Host Access to Shared Memory"]
pub type ShmAccR = crate::BitReader;
#[doc = "Field `SHM_ACC` writer - Host Access to Shared Memory"]
pub type ShmAccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Read Access Error"]
    #[inline(always)]
    pub fn hrerr(&self) -> HrerrR {
        HrerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Write Access Error"]
    #[inline(always)]
    pub fn hwerr(&self) -> HwerrR {
        HwerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Host Semaphore Indirect Memory Access Written"]
    #[inline(always)]
    pub fn hsem_imaw(&self) -> HsemImawR {
        HsemImawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host Semaphore 1 Written"]
    #[inline(always)]
    pub fn hsem1w(&self) -> Hsem1wR {
        Hsem1wR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Semaphore 2 Written"]
    #[inline(always)]
    pub fn hsem2w(&self) -> Hsem2wR {
        Hsem2wR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host Access to Shared Memory"]
    #[inline(always)]
    pub fn shm_acc(&self) -> ShmAccR {
        ShmAccR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMC_STS")
            .field("hrerr", &self.hrerr())
            .field("hwerr", &self.hwerr())
            .field("hsem_imaw", &self.hsem_imaw())
            .field("hsem1w", &self.hsem1w())
            .field("hsem2w", &self.hsem2w())
            .field("shm_acc", &self.shm_acc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Host Read Access Error"]
    #[inline(always)]
    pub fn hrerr(&mut self) -> HrerrW<SmcStsSpec> {
        HrerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Write Access Error"]
    #[inline(always)]
    pub fn hwerr(&mut self) -> HwerrW<SmcStsSpec> {
        HwerrW::new(self, 1)
    }
    #[doc = "Bit 3 - Host Semaphore Indirect Memory Access Written"]
    #[inline(always)]
    pub fn hsem_imaw(&mut self) -> HsemImawW<SmcStsSpec> {
        HsemImawW::new(self, 3)
    }
    #[doc = "Bit 4 - Host Semaphore 1 Written"]
    #[inline(always)]
    pub fn hsem1w(&mut self) -> Hsem1wW<SmcStsSpec> {
        Hsem1wW::new(self, 4)
    }
    #[doc = "Bit 5 - Host Semaphore 2 Written"]
    #[inline(always)]
    pub fn hsem2w(&mut self) -> Hsem2wW<SmcStsSpec> {
        Hsem2wW::new(self, 5)
    }
    #[doc = "Bit 6 - Host Access to Shared Memory"]
    #[inline(always)]
    pub fn shm_acc(&mut self) -> ShmAccW<SmcStsSpec> {
        ShmAccW::new(self, 6)
    }
}
#[doc = "Shared Memory Core Status Register (SMC_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcStsSpec;
impl crate::RegisterSpec for SmcStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smc_sts::R`](R) reader structure"]
impl crate::Readable for SmcStsSpec {}
#[doc = "`write(|w| ..)` method takes [`smc_sts::W`](W) writer structure"]
impl crate::Writable for SmcStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMC_STS to value 0"]
impl crate::Resettable for SmcStsSpec {
    const RESET_VALUE: u8 = 0;
}
