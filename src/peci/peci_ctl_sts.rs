#[doc = "Register `PECI_CTL_STS` reader"]
pub type R = crate::R<PeciCtlStsSpec>;
#[doc = "Register `PECI_CTL_STS` writer"]
pub type W = crate::W<PeciCtlStsSpec>;
#[doc = "Field `START_BUSY` reader - Start/Busy"]
pub type StartBusyR = crate::BitReader;
#[doc = "Field `START_BUSY` writer - Start/Busy"]
pub type StartBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - Done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ERR` reader - CRC Error"]
pub type CrcErrR = crate::BitReader;
#[doc = "Field `CRC_ERR` writer - CRC Error"]
pub type CrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRT_ERR` reader - Abort Error"]
pub type AbrtErrR = crate::BitReader;
#[doc = "Field `ABRT_ERR` writer - Abort Error"]
pub type AbrtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWFCS_EN` reader - Assured Write FCS Enable"]
pub type AwfcsEnR = crate::BitReader;
#[doc = "Field `AWFCS_EN` writer - Assured Write FCS Enable"]
pub type AwfcsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_EN` reader - DONE Enable"]
pub type DoneEnR = crate::BitReader;
#[doc = "Field `DONE_EN` writer - DONE Enable"]
pub type DoneEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start/Busy"]
    #[inline(always)]
    pub fn start_busy(&self) -> StartBusyR {
        StartBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - CRC Error"]
    #[inline(always)]
    pub fn crc_err(&self) -> CrcErrR {
        CrcErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Abort Error"]
    #[inline(always)]
    pub fn abrt_err(&self) -> AbrtErrR {
        AbrtErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Assured Write FCS Enable"]
    #[inline(always)]
    pub fn awfcs_en(&self) -> AwfcsEnR {
        AwfcsEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DONE Enable"]
    #[inline(always)]
    pub fn done_en(&self) -> DoneEnR {
        DoneEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PECI_CTL_STS")
            .field("start_busy", &self.start_busy())
            .field("done", &self.done())
            .field("crc_err", &self.crc_err())
            .field("abrt_err", &self.abrt_err())
            .field("awfcs_en", &self.awfcs_en())
            .field("done_en", &self.done_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start/Busy"]
    #[inline(always)]
    #[must_use]
    pub fn start_busy(&mut self) -> StartBusyW<PeciCtlStsSpec> {
        StartBusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<PeciCtlStsSpec> {
        DoneW::new(self, 1)
    }
    #[doc = "Bit 3 - CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn crc_err(&mut self) -> CrcErrW<PeciCtlStsSpec> {
        CrcErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Abort Error"]
    #[inline(always)]
    #[must_use]
    pub fn abrt_err(&mut self) -> AbrtErrW<PeciCtlStsSpec> {
        AbrtErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Assured Write FCS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn awfcs_en(&mut self) -> AwfcsEnW<PeciCtlStsSpec> {
        AwfcsEnW::new(self, 5)
    }
    #[doc = "Bit 6 - DONE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn done_en(&mut self) -> DoneEnW<PeciCtlStsSpec> {
        DoneEnW::new(self, 6)
    }
}
#[doc = "PECI Control Status Register (PECI_CTL_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`peci_ctl_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peci_ctl_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeciCtlStsSpec;
impl crate::RegisterSpec for PeciCtlStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`peci_ctl_sts::R`](R) reader structure"]
impl crate::Readable for PeciCtlStsSpec {}
#[doc = "`write(|w| ..)` method takes [`peci_ctl_sts::W`](W) writer structure"]
impl crate::Writable for PeciCtlStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PECI_CTL_STS to value 0"]
impl crate::Resettable for PeciCtlStsSpec {
    const RESET_VALUE: u8 = 0;
}
