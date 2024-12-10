#[doc = "Register `DISIDL_CTL1` reader"]
pub type R = crate::R<DisidlCtl1Spec>;
#[doc = "Register `DISIDL_CTL1` writer"]
pub type W = crate::W<DisidlCtl1Spec>;
#[doc = "Field `FIU0_DID` reader - FIU0 Disable in Idle"]
pub type Fiu0DidR = crate::BitReader;
#[doc = "Field `FIU0_DID` writer - FIU0 Disable in Idle"]
pub type Fiu0DidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU1_DID` reader - FIU1 Disable in Idle"]
pub type Fiu1DidR = crate::BitReader;
#[doc = "Field `FIU1_DID` writer - FIU1 Disable in Idle"]
pub type Fiu1DidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHM_ACC_DID` reader - SHM Access Disable in Idle"]
pub type ShmAccDidR = crate::BitReader;
#[doc = "Field `SHM_ACC_DID` writer - SHM Access Disable in Idle"]
pub type ShmAccDidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - FIU0 Disable in Idle"]
    #[inline(always)]
    pub fn fiu0_did(&self) -> Fiu0DidR {
        Fiu0DidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIU1 Disable in Idle"]
    #[inline(always)]
    pub fn fiu1_did(&self) -> Fiu1DidR {
        Fiu1DidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SHM Access Disable in Idle"]
    #[inline(always)]
    pub fn shm_acc_did(&self) -> ShmAccDidR {
        ShmAccDidR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DISIDL_CTL1")
            .field("fiu0_did", &self.fiu0_did())
            .field("fiu1_did", &self.fiu1_did())
            .field("shm_acc_did", &self.shm_acc_did())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - FIU0 Disable in Idle"]
    #[inline(always)]
    pub fn fiu0_did(&mut self) -> Fiu0DidW<DisidlCtl1Spec> {
        Fiu0DidW::new(self, 4)
    }
    #[doc = "Bit 5 - FIU1 Disable in Idle"]
    #[inline(always)]
    pub fn fiu1_did(&mut self) -> Fiu1DidW<DisidlCtl1Spec> {
        Fiu1DidW::new(self, 5)
    }
    #[doc = "Bit 7 - SHM Access Disable in Idle"]
    #[inline(always)]
    pub fn shm_acc_did(&mut self) -> ShmAccDidW<DisidlCtl1Spec> {
        ShmAccDidW::new(self, 7)
    }
}
#[doc = "Disable in Idle Control 1 Register (DISIDL_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`disidl_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disidl_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisidlCtl1Spec;
impl crate::RegisterSpec for DisidlCtl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`disidl_ctl1::R`](R) reader structure"]
impl crate::Readable for DisidlCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`disidl_ctl1::W`](W) writer structure"]
impl crate::Writable for DisidlCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DISIDL_CTL1 to value 0"]
impl crate::Resettable for DisidlCtl1Spec {
    const RESET_VALUE: u8 = 0;
}
