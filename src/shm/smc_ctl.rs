#[doc = "Register `SMC_CTL` reader"]
pub type R = crate::R<SmcCtlSpec>;
#[doc = "Register `SMC_CTL` writer"]
pub type W = crate::W<SmcCtlSpec>;
#[doc = "Field `HERES` reader - Host Error Response"]
pub type HeresR = crate::FieldReader;
#[doc = "Field `HERES` writer - Host Error Response"]
pub type HeresW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HERR_IE` reader - Enable Interrupt by Host Access Errors"]
pub type HerrIeR = crate::BitReader;
#[doc = "Field `HERR_IE` writer - Enable Interrupt by Host Access Errors"]
pub type HerrIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM1_IE` reader - Enable Interrupt by Host Semaphore 1 Written"]
pub type Hsem1IeR = crate::BitReader;
#[doc = "Field `HSEM1_IE` writer - Enable Interrupt by Host Semaphore 1 Written"]
pub type Hsem1IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM2_IE` reader - Enable Interrupt by Host Semaphore 2 Written"]
pub type Hsem2IeR = crate::BitReader;
#[doc = "Field `HSEM2_IE` writer - Enable Interrupt by Host Semaphore 2 Written"]
pub type Hsem2IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHM_ACC_IE` reader - Enable Interrupt by Host Access to Shared Memory"]
pub type ShmAccIeR = crate::BitReader;
#[doc = "Field `SHM_ACC_IE` writer - Enable Interrupt by Host Access to Shared Memory"]
pub type ShmAccIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEM_IMA_IE` reader - Enable Interrupt by Host Indirect Memory Access Written"]
pub type HsemImaIeR = crate::BitReader;
#[doc = "Field `HSEM_IMA_IE` writer - Enable Interrupt by Host Indirect Memory Access Written"]
pub type HsemImaIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTWAIT` reader - Host Access Stall"]
pub type HostwaitR = crate::BitReader;
#[doc = "Field `HOSTWAIT` writer - Host Access Stall"]
pub type HostwaitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Host Error Response"]
    #[inline(always)]
    pub fn heres(&self) -> HeresR {
        HeresR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Enable Interrupt by Host Access Errors"]
    #[inline(always)]
    pub fn herr_ie(&self) -> HerrIeR {
        HerrIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Semaphore 1 Written"]
    #[inline(always)]
    pub fn hsem1_ie(&self) -> Hsem1IeR {
        Hsem1IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Interrupt by Host Semaphore 2 Written"]
    #[inline(always)]
    pub fn hsem2_ie(&self) -> Hsem2IeR {
        Hsem2IeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt by Host Access to Shared Memory"]
    #[inline(always)]
    pub fn shm_acc_ie(&self) -> ShmAccIeR {
        ShmAccIeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Interrupt by Host Indirect Memory Access Written"]
    #[inline(always)]
    pub fn hsem_ima_ie(&self) -> HsemImaIeR {
        HsemImaIeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Host Access Stall"]
    #[inline(always)]
    pub fn hostwait(&self) -> HostwaitR {
        HostwaitR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMC_CTL")
            .field("heres", &self.heres())
            .field("herr_ie", &self.herr_ie())
            .field("hsem1_ie", &self.hsem1_ie())
            .field("hsem2_ie", &self.hsem2_ie())
            .field("shm_acc_ie", &self.shm_acc_ie())
            .field("hsem_ima_ie", &self.hsem_ima_ie())
            .field("hostwait", &self.hostwait())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Host Error Response"]
    #[inline(always)]
    pub fn heres(&mut self) -> HeresW<SmcCtlSpec> {
        HeresW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable Interrupt by Host Access Errors"]
    #[inline(always)]
    pub fn herr_ie(&mut self) -> HerrIeW<SmcCtlSpec> {
        HerrIeW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Semaphore 1 Written"]
    #[inline(always)]
    pub fn hsem1_ie(&mut self) -> Hsem1IeW<SmcCtlSpec> {
        Hsem1IeW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Interrupt by Host Semaphore 2 Written"]
    #[inline(always)]
    pub fn hsem2_ie(&mut self) -> Hsem2IeW<SmcCtlSpec> {
        Hsem2IeW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Interrupt by Host Access to Shared Memory"]
    #[inline(always)]
    pub fn shm_acc_ie(&mut self) -> ShmAccIeW<SmcCtlSpec> {
        ShmAccIeW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Interrupt by Host Indirect Memory Access Written"]
    #[inline(always)]
    pub fn hsem_ima_ie(&mut self) -> HsemImaIeW<SmcCtlSpec> {
        HsemImaIeW::new(self, 6)
    }
    #[doc = "Bit 7 - Host Access Stall"]
    #[inline(always)]
    pub fn hostwait(&mut self) -> HostwaitW<SmcCtlSpec> {
        HostwaitW::new(self, 7)
    }
}
#[doc = "Shared Memory Core Control Register (SMC_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcCtlSpec;
impl crate::RegisterSpec for SmcCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smc_ctl::R`](R) reader structure"]
impl crate::Readable for SmcCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`smc_ctl::W`](W) writer structure"]
impl crate::Writable for SmcCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMC_CTL to value 0"]
impl crate::Resettable for SmcCtlSpec {
    const RESET_VALUE: u8 = 0;
}
