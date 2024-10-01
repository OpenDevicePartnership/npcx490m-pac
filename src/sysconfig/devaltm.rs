#[doc = "Register `DEVALTM` reader"]
pub type R = crate::R<DevaltmSpec>;
#[doc = "Register `DEVALTM` writer"]
pub type W = crate::W<DevaltmSpec>;
#[doc = "Field `AD21_SL` reader - AD21 Select"]
pub type Ad21SlR = crate::BitReader;
#[doc = "Field `AD21_SL` writer - AD21 Select"]
pub type Ad21SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD22_SL` reader - AD22 Select"]
pub type Ad22SlR = crate::BitReader;
#[doc = "Field `AD22_SL` writer - AD22 Select"]
pub type Ad22SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD23_SL` reader - AD23 Select"]
pub type Ad23SlR = crate::BitReader;
#[doc = "Field `AD23_SL` writer - AD23 Select"]
pub type Ad23SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD24_SL` reader - AD24 Select"]
pub type Ad24SlR = crate::BitReader;
#[doc = "Field `AD24_SL` writer - AD24 Select"]
pub type Ad24SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD25_SL` reader - AD25 Select"]
pub type Ad25SlR = crate::BitReader;
#[doc = "Field `AD25_SL` writer - AD25 Select"]
pub type Ad25SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AD21 Select"]
    #[inline(always)]
    pub fn ad21_sl(&self) -> Ad21SlR {
        Ad21SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD22 Select"]
    #[inline(always)]
    pub fn ad22_sl(&self) -> Ad22SlR {
        Ad22SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD23 Select"]
    #[inline(always)]
    pub fn ad23_sl(&self) -> Ad23SlR {
        Ad23SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD24 Select"]
    #[inline(always)]
    pub fn ad24_sl(&self) -> Ad24SlR {
        Ad24SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD25 Select"]
    #[inline(always)]
    pub fn ad25_sl(&self) -> Ad25SlR {
        Ad25SlR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTM")
            .field("ad21_sl", &self.ad21_sl())
            .field("ad22_sl", &self.ad22_sl())
            .field("ad23_sl", &self.ad23_sl())
            .field("ad24_sl", &self.ad24_sl())
            .field("ad25_sl", &self.ad25_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AD21 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad21_sl(&mut self) -> Ad21SlW<DevaltmSpec> {
        Ad21SlW::new(self, 0)
    }
    #[doc = "Bit 1 - AD22 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad22_sl(&mut self) -> Ad22SlW<DevaltmSpec> {
        Ad22SlW::new(self, 1)
    }
    #[doc = "Bit 2 - AD23 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad23_sl(&mut self) -> Ad23SlW<DevaltmSpec> {
        Ad23SlW::new(self, 2)
    }
    #[doc = "Bit 3 - AD24 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad24_sl(&mut self) -> Ad24SlW<DevaltmSpec> {
        Ad24SlW::new(self, 3)
    }
    #[doc = "Bit 4 - AD25 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ad25_sl(&mut self) -> Ad25SlW<DevaltmSpec> {
        Ad25SlW::new(self, 4)
    }
}
#[doc = "Device Alternate Function M Register (DEVALTM)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltmSpec;
impl crate::RegisterSpec for DevaltmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltm::R`](R) reader structure"]
impl crate::Readable for DevaltmSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltm::W`](W) writer structure"]
impl crate::Writable for DevaltmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTM to value 0"]
impl crate::Resettable for DevaltmSpec {
    const RESET_VALUE: u8 = 0;
}
