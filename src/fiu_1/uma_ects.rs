#[doc = "Register `UMA_ECTS` reader"]
pub type R = crate::R<UmaEctsSpec>;
#[doc = "Register `UMA_ECTS` writer"]
pub type W = crate::W<UmaEctsSpec>;
#[doc = "Field `SW_CS` reader - Software-Controlled Chip-Select"]
pub type SwCsR = crate::BitReader;
#[doc = "Field `SW_CS` writer - Software-Controlled Chip-Select"]
pub type SwCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_CS` reader - Secondary Chip-Select"]
pub type SecCsR = crate::BitReader;
#[doc = "Field `SEC_CS` writer - Secondary Chip-Select"]
pub type SecCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UMA_LOCK` reader - UMA Operation Lock"]
pub type UmaLockR = crate::BitReader;
#[doc = "Field `UMA_LOCK` writer - UMA Operation Lock"]
pub type UmaLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UMA_ADDR_SIZE` reader - Address Field Size Select"]
pub type UmaAddrSizeR = crate::FieldReader;
#[doc = "Field `UMA_ADDR_SIZE` writer - Address Field Size Select"]
pub type UmaAddrSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Software-Controlled Chip-Select"]
    #[inline(always)]
    pub fn sw_cs(&self) -> SwCsR {
        SwCsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Secondary Chip-Select"]
    #[inline(always)]
    pub fn sec_cs(&self) -> SecCsR {
        SecCsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UMA Operation Lock"]
    #[inline(always)]
    pub fn uma_lock(&self) -> UmaLockR {
        UmaLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Address Field Size Select"]
    #[inline(always)]
    pub fn uma_addr_size(&self) -> UmaAddrSizeR {
        UmaAddrSizeR::new((self.bits >> 4) & 7)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UMA_ECTS")
            .field("sw_cs", &self.sw_cs())
            .field("sec_cs", &self.sec_cs())
            .field("uma_lock", &self.uma_lock())
            .field("uma_addr_size", &self.uma_addr_size())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software-Controlled Chip-Select"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cs(&mut self) -> SwCsW<UmaEctsSpec> {
        SwCsW::new(self, 0)
    }
    #[doc = "Bit 2 - Secondary Chip-Select"]
    #[inline(always)]
    #[must_use]
    pub fn sec_cs(&mut self) -> SecCsW<UmaEctsSpec> {
        SecCsW::new(self, 2)
    }
    #[doc = "Bit 3 - UMA Operation Lock"]
    #[inline(always)]
    #[must_use]
    pub fn uma_lock(&mut self) -> UmaLockW<UmaEctsSpec> {
        UmaLockW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Address Field Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn uma_addr_size(&mut self) -> UmaAddrSizeW<UmaEctsSpec> {
        UmaAddrSizeW::new(self, 4)
    }
}
#[doc = "UMA Extended Control and Status Register (UMA_ECTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`uma_ects::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uma_ects::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmaEctsSpec;
impl crate::RegisterSpec for UmaEctsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uma_ects::R`](R) reader structure"]
impl crate::Readable for UmaEctsSpec {}
#[doc = "`write(|w| ..)` method takes [`uma_ects::W`](W) writer structure"]
impl crate::Writable for UmaEctsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UMA_ECTS to value 0"]
impl crate::Resettable for UmaEctsSpec {
    const RESET_VALUE: u8 = 0;
}
