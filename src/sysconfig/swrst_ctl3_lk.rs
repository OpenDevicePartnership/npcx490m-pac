#[doc = "Register `SWRST_CTL3_LK` reader"]
pub type R = crate::R<SwrstCtl3LkSpec>;
#[doc = "Register `SWRST_CTL3_LK` writer"]
pub type W = crate::W<SwrstCtl3LkSpec>;
#[doc = "Field `SIOCFG_RST_LK` reader - SuperI/O Configuration Reset Lock"]
pub type SiocfgRstLkR = crate::BitReader;
#[doc = "Field `SIOCFG_RST_LK` writer - SuperI/O Configuration Reset Lock"]
pub type SiocfgRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERPORT_RST_LK` reader - Serial Port Reset Lock"]
pub type SerportRstLkR = crate::BitReader;
#[doc = "Field `SERPORT_RST_LK` writer - Serial Port Reset Lock"]
pub type SerportRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C1_RST_LK` reader - I3C1 Reset Lock"]
pub type I3c1RstLkR = crate::BitReader;
#[doc = "Field `I3C1_RST_LK` writer - I3C1 Reset Lock"]
pub type I3c1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2_RST_LK` reader - I3C2 Reset Lock"]
pub type I3c2RstLkR = crate::BitReader;
#[doc = "Field `I3C2_RST_LK` writer - I3C2 Reset Lock"]
pub type I3c2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3_RST_LK` reader - I3C3 Reset Lock"]
pub type I3c3RstLkR = crate::BitReader;
#[doc = "Field `I3C3_RST_LK` writer - I3C3 Reset Lock"]
pub type I3c3RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSWC_RST_LK` reader - MSWC Reset Lock"]
pub type MswcRstLkR = crate::BitReader;
#[doc = "Field `MSWC_RST_LK` writer - MSWC Reset Lock"]
pub type MswcRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHM_RST_LK` reader - SHM Reset Lock"]
pub type ShmRstLkR = crate::BitReader;
#[doc = "Field `SHM_RST_LK` writer - SHM Reset Lock"]
pub type ShmRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH1_RST_LK` reader - PM Channel 1 Reset Lock"]
pub type Pmch1RstLkR = crate::BitReader;
#[doc = "Field `PMCH1_RST_LK` writer - PM Channel 1 Reset Lock"]
pub type Pmch1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH2_RST_LK` reader - PM Channel 2 Reset Lock"]
pub type Pmch2RstLkR = crate::BitReader;
#[doc = "Field `PMCH2_RST_LK` writer - PM Channel 2 Reset Lock"]
pub type Pmch2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH3_RST_LK` reader - PM Channel 3 Reset Lock"]
pub type Pmch3RstLkR = crate::BitReader;
#[doc = "Field `PMCH3_RST_LK` writer - PM Channel 3 Reset Lock"]
pub type Pmch3RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH4_RST_LK` reader - PM Channel 4 Reset Lock"]
pub type Pmch4RstLkR = crate::BitReader;
#[doc = "Field `PMCH4_RST_LK` writer - PM Channel 4 Reset Lock"]
pub type Pmch4RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBC_RST_LK` reader - KBC Reset Lock"]
pub type KbcRstLkR = crate::BitReader;
#[doc = "Field `KBC_RST_LK` writer - KBC Reset Lock"]
pub type KbcRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2HOST_RST_LK` reader - Core-to-Host Reset Lock"]
pub type C2hostRstLkR = crate::BitReader;
#[doc = "Field `C2HOST_RST_LK` writer - Core-to-Host Reset Lock"]
pub type C2hostRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT3_RST_LK` reader - CR_UART 3 Reset Lock"]
pub type Crurt3RstLkR = crate::BitReader;
#[doc = "Field `CRURT3_RST_LK` writer - CR_UART 3 Reset Lock"]
pub type Crurt3RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT4_RST_LK` reader - CR_UART 4 Reset Lock"]
pub type Crurt4RstLkR = crate::BitReader;
#[doc = "Field `CRURT4_RST_LK` writer - CR_UART 4 Reset Lock"]
pub type Crurt4RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFCG_RST_LK` reader - LFCG Reset Lock"]
pub type LfcgRstLkR = crate::BitReader;
#[doc = "Field `LFCG_RST_LK` writer - LFCG Reset Lock"]
pub type LfcgRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_RST_LK` reader - Development Reset Lock"]
pub type DevRstLkR = crate::BitReader;
#[doc = "Field `DEV_RST_LK` writer - Development Reset Lock"]
pub type DevRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_RST_LK` reader - SYSCFG Reset Lock"]
pub type SyscfgRstLkR = crate::BitReader;
#[doc = "Field `SYSCFG_RST_LK` writer - SYSCFG Reset Lock"]
pub type SyscfgRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBY_RST_LK` reader - Standby Reset Lock"]
pub type SbyRstLkR = crate::BitReader;
#[doc = "Field `SBY_RST_LK` writer - Standby Reset Lock"]
pub type SbyRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBRM_RST_LK` reader - BBRM Reset Lock"]
pub type BbrmRstLkR = crate::BitReader;
#[doc = "Field `BBRM_RST_LK` writer - BBRM Reset Lock"]
pub type BbrmRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA2b_RST_LK` reader - SHAb Reset Lock"]
pub type Sha2bRstLkR = crate::BitReader;
#[doc = "Field `SHA2b_RST_LK` writer - SHAb Reset Lock"]
pub type Sha2bRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTPI_RST_LK` reader - OTPI Reset Lock"]
pub type OtpiRstLkR = crate::BitReader;
#[doc = "Field `OTPI_RST_LK` writer - OTPI Reset Lock"]
pub type OtpiRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_RST_LK` reader - RNG Reset Lock"]
pub type RngRstLkR = crate::BitReader;
#[doc = "Field `RNG_RST_LK` writer - RNG Reset Lock"]
pub type RngRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA2a_RST_LK` reader - SHAa Reset Lock"]
pub type Sha2aRstLkR = crate::BitReader;
#[doc = "Field `SHA2a_RST_LK` writer - SHAa Reset Lock"]
pub type Sha2aRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_RST_LK` reader - PKA Reset Lock"]
pub type PkaRstLkR = crate::BitReader;
#[doc = "Field `PKA_RST_LK` writer - PKA Reset Lock"]
pub type PkaRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_RST_LK` reader - AES Reset Lock"]
pub type AesRstLkR = crate::BitReader;
#[doc = "Field `AES_RST_LK` writer - AES Reset Lock"]
pub type AesRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SuperI/O Configuration Reset Lock"]
    #[inline(always)]
    pub fn siocfg_rst_lk(&self) -> SiocfgRstLkR {
        SiocfgRstLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Port Reset Lock"]
    #[inline(always)]
    pub fn serport_rst_lk(&self) -> SerportRstLkR {
        SerportRstLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - I3C1 Reset Lock"]
    #[inline(always)]
    pub fn i3c1_rst_lk(&self) -> I3c1RstLkR {
        I3c1RstLkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I3C2 Reset Lock"]
    #[inline(always)]
    pub fn i3c2_rst_lk(&self) -> I3c2RstLkR {
        I3c2RstLkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I3C3 Reset Lock"]
    #[inline(always)]
    pub fn i3c3_rst_lk(&self) -> I3c3RstLkR {
        I3c3RstLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MSWC Reset Lock"]
    #[inline(always)]
    pub fn mswc_rst_lk(&self) -> MswcRstLkR {
        MswcRstLkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SHM Reset Lock"]
    #[inline(always)]
    pub fn shm_rst_lk(&self) -> ShmRstLkR {
        ShmRstLkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PM Channel 1 Reset Lock"]
    #[inline(always)]
    pub fn pmch1_rst_lk(&self) -> Pmch1RstLkR {
        Pmch1RstLkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PM Channel 2 Reset Lock"]
    #[inline(always)]
    pub fn pmch2_rst_lk(&self) -> Pmch2RstLkR {
        Pmch2RstLkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PM Channel 3 Reset Lock"]
    #[inline(always)]
    pub fn pmch3_rst_lk(&self) -> Pmch3RstLkR {
        Pmch3RstLkR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PM Channel 4 Reset Lock"]
    #[inline(always)]
    pub fn pmch4_rst_lk(&self) -> Pmch4RstLkR {
        Pmch4RstLkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - KBC Reset Lock"]
    #[inline(always)]
    pub fn kbc_rst_lk(&self) -> KbcRstLkR {
        KbcRstLkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Core-to-Host Reset Lock"]
    #[inline(always)]
    pub fn c2host_rst_lk(&self) -> C2hostRstLkR {
        C2hostRstLkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - CR_UART 3 Reset Lock"]
    #[inline(always)]
    pub fn crurt3_rst_lk(&self) -> Crurt3RstLkR {
        Crurt3RstLkR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CR_UART 4 Reset Lock"]
    #[inline(always)]
    pub fn crurt4_rst_lk(&self) -> Crurt4RstLkR {
        Crurt4RstLkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LFCG Reset Lock"]
    #[inline(always)]
    pub fn lfcg_rst_lk(&self) -> LfcgRstLkR {
        LfcgRstLkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Development Reset Lock"]
    #[inline(always)]
    pub fn dev_rst_lk(&self) -> DevRstLkR {
        DevRstLkR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SYSCFG Reset Lock"]
    #[inline(always)]
    pub fn syscfg_rst_lk(&self) -> SyscfgRstLkR {
        SyscfgRstLkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Standby Reset Lock"]
    #[inline(always)]
    pub fn sby_rst_lk(&self) -> SbyRstLkR {
        SbyRstLkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BBRM Reset Lock"]
    #[inline(always)]
    pub fn bbrm_rst_lk(&self) -> BbrmRstLkR {
        BbrmRstLkR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SHAb Reset Lock"]
    #[inline(always)]
    pub fn sha2b_rst_lk(&self) -> Sha2bRstLkR {
        Sha2bRstLkR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OTPI Reset Lock"]
    #[inline(always)]
    pub fn otpi_rst_lk(&self) -> OtpiRstLkR {
        OtpiRstLkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RNG Reset Lock"]
    #[inline(always)]
    pub fn rng_rst_lk(&self) -> RngRstLkR {
        RngRstLkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SHAa Reset Lock"]
    #[inline(always)]
    pub fn sha2a_rst_lk(&self) -> Sha2aRstLkR {
        Sha2aRstLkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PKA Reset Lock"]
    #[inline(always)]
    pub fn pka_rst_lk(&self) -> PkaRstLkR {
        PkaRstLkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AES Reset Lock"]
    #[inline(always)]
    pub fn aes_rst_lk(&self) -> AesRstLkR {
        AesRstLkR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRST_CTL3_LK")
            .field("siocfg_rst_lk", &self.siocfg_rst_lk())
            .field("serport_rst_lk", &self.serport_rst_lk())
            .field("i3c1_rst_lk", &self.i3c1_rst_lk())
            .field("i3c2_rst_lk", &self.i3c2_rst_lk())
            .field("i3c3_rst_lk", &self.i3c3_rst_lk())
            .field("mswc_rst_lk", &self.mswc_rst_lk())
            .field("shm_rst_lk", &self.shm_rst_lk())
            .field("pmch1_rst_lk", &self.pmch1_rst_lk())
            .field("pmch2_rst_lk", &self.pmch2_rst_lk())
            .field("pmch3_rst_lk", &self.pmch3_rst_lk())
            .field("pmch4_rst_lk", &self.pmch4_rst_lk())
            .field("kbc_rst_lk", &self.kbc_rst_lk())
            .field("c2host_rst_lk", &self.c2host_rst_lk())
            .field("crurt3_rst_lk", &self.crurt3_rst_lk())
            .field("crurt4_rst_lk", &self.crurt4_rst_lk())
            .field("lfcg_rst_lk", &self.lfcg_rst_lk())
            .field("dev_rst_lk", &self.dev_rst_lk())
            .field("syscfg_rst_lk", &self.syscfg_rst_lk())
            .field("bbrm_rst_lk", &self.bbrm_rst_lk())
            .field("sha2b_rst_lk", &self.sha2b_rst_lk())
            .field("otpi_rst_lk", &self.otpi_rst_lk())
            .field("rng_rst_lk", &self.rng_rst_lk())
            .field("sha2a_rst_lk", &self.sha2a_rst_lk())
            .field("pka_rst_lk", &self.pka_rst_lk())
            .field("aes_rst_lk", &self.aes_rst_lk())
            .field("sby_rst_lk", &self.sby_rst_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SuperI/O Configuration Reset Lock"]
    #[inline(always)]
    pub fn siocfg_rst_lk(&mut self) -> SiocfgRstLkW<SwrstCtl3LkSpec> {
        SiocfgRstLkW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial Port Reset Lock"]
    #[inline(always)]
    pub fn serport_rst_lk(&mut self) -> SerportRstLkW<SwrstCtl3LkSpec> {
        SerportRstLkW::new(self, 1)
    }
    #[doc = "Bit 4 - I3C1 Reset Lock"]
    #[inline(always)]
    pub fn i3c1_rst_lk(&mut self) -> I3c1RstLkW<SwrstCtl3LkSpec> {
        I3c1RstLkW::new(self, 4)
    }
    #[doc = "Bit 5 - I3C2 Reset Lock"]
    #[inline(always)]
    pub fn i3c2_rst_lk(&mut self) -> I3c2RstLkW<SwrstCtl3LkSpec> {
        I3c2RstLkW::new(self, 5)
    }
    #[doc = "Bit 6 - I3C3 Reset Lock"]
    #[inline(always)]
    pub fn i3c3_rst_lk(&mut self) -> I3c3RstLkW<SwrstCtl3LkSpec> {
        I3c3RstLkW::new(self, 6)
    }
    #[doc = "Bit 8 - MSWC Reset Lock"]
    #[inline(always)]
    pub fn mswc_rst_lk(&mut self) -> MswcRstLkW<SwrstCtl3LkSpec> {
        MswcRstLkW::new(self, 8)
    }
    #[doc = "Bit 9 - SHM Reset Lock"]
    #[inline(always)]
    pub fn shm_rst_lk(&mut self) -> ShmRstLkW<SwrstCtl3LkSpec> {
        ShmRstLkW::new(self, 9)
    }
    #[doc = "Bit 10 - PM Channel 1 Reset Lock"]
    #[inline(always)]
    pub fn pmch1_rst_lk(&mut self) -> Pmch1RstLkW<SwrstCtl3LkSpec> {
        Pmch1RstLkW::new(self, 10)
    }
    #[doc = "Bit 11 - PM Channel 2 Reset Lock"]
    #[inline(always)]
    pub fn pmch2_rst_lk(&mut self) -> Pmch2RstLkW<SwrstCtl3LkSpec> {
        Pmch2RstLkW::new(self, 11)
    }
    #[doc = "Bit 12 - PM Channel 3 Reset Lock"]
    #[inline(always)]
    pub fn pmch3_rst_lk(&mut self) -> Pmch3RstLkW<SwrstCtl3LkSpec> {
        Pmch3RstLkW::new(self, 12)
    }
    #[doc = "Bit 13 - PM Channel 4 Reset Lock"]
    #[inline(always)]
    pub fn pmch4_rst_lk(&mut self) -> Pmch4RstLkW<SwrstCtl3LkSpec> {
        Pmch4RstLkW::new(self, 13)
    }
    #[doc = "Bit 15 - KBC Reset Lock"]
    #[inline(always)]
    pub fn kbc_rst_lk(&mut self) -> KbcRstLkW<SwrstCtl3LkSpec> {
        KbcRstLkW::new(self, 15)
    }
    #[doc = "Bit 16 - Core-to-Host Reset Lock"]
    #[inline(always)]
    pub fn c2host_rst_lk(&mut self) -> C2hostRstLkW<SwrstCtl3LkSpec> {
        C2hostRstLkW::new(self, 16)
    }
    #[doc = "Bit 18 - CR_UART 3 Reset Lock"]
    #[inline(always)]
    pub fn crurt3_rst_lk(&mut self) -> Crurt3RstLkW<SwrstCtl3LkSpec> {
        Crurt3RstLkW::new(self, 18)
    }
    #[doc = "Bit 19 - CR_UART 4 Reset Lock"]
    #[inline(always)]
    pub fn crurt4_rst_lk(&mut self) -> Crurt4RstLkW<SwrstCtl3LkSpec> {
        Crurt4RstLkW::new(self, 19)
    }
    #[doc = "Bit 20 - LFCG Reset Lock"]
    #[inline(always)]
    pub fn lfcg_rst_lk(&mut self) -> LfcgRstLkW<SwrstCtl3LkSpec> {
        LfcgRstLkW::new(self, 20)
    }
    #[doc = "Bit 22 - Development Reset Lock"]
    #[inline(always)]
    pub fn dev_rst_lk(&mut self) -> DevRstLkW<SwrstCtl3LkSpec> {
        DevRstLkW::new(self, 22)
    }
    #[doc = "Bit 23 - SYSCFG Reset Lock"]
    #[inline(always)]
    pub fn syscfg_rst_lk(&mut self) -> SyscfgRstLkW<SwrstCtl3LkSpec> {
        SyscfgRstLkW::new(self, 23)
    }
    #[doc = "Bit 24 - Standby Reset Lock"]
    #[inline(always)]
    pub fn sby_rst_lk(&mut self) -> SbyRstLkW<SwrstCtl3LkSpec> {
        SbyRstLkW::new(self, 24)
    }
    #[doc = "Bit 25 - BBRM Reset Lock"]
    #[inline(always)]
    pub fn bbrm_rst_lk(&mut self) -> BbrmRstLkW<SwrstCtl3LkSpec> {
        BbrmRstLkW::new(self, 25)
    }
    #[doc = "Bit 26 - SHAb Reset Lock"]
    #[inline(always)]
    pub fn sha2b_rst_lk(&mut self) -> Sha2bRstLkW<SwrstCtl3LkSpec> {
        Sha2bRstLkW::new(self, 26)
    }
    #[doc = "Bit 27 - OTPI Reset Lock"]
    #[inline(always)]
    pub fn otpi_rst_lk(&mut self) -> OtpiRstLkW<SwrstCtl3LkSpec> {
        OtpiRstLkW::new(self, 27)
    }
    #[doc = "Bit 28 - RNG Reset Lock"]
    #[inline(always)]
    pub fn rng_rst_lk(&mut self) -> RngRstLkW<SwrstCtl3LkSpec> {
        RngRstLkW::new(self, 28)
    }
    #[doc = "Bit 29 - SHAa Reset Lock"]
    #[inline(always)]
    pub fn sha2a_rst_lk(&mut self) -> Sha2aRstLkW<SwrstCtl3LkSpec> {
        Sha2aRstLkW::new(self, 29)
    }
    #[doc = "Bit 30 - PKA Reset Lock"]
    #[inline(always)]
    pub fn pka_rst_lk(&mut self) -> PkaRstLkW<SwrstCtl3LkSpec> {
        PkaRstLkW::new(self, 30)
    }
    #[doc = "Bit 31 - AES Reset Lock"]
    #[inline(always)]
    pub fn aes_rst_lk(&mut self) -> AesRstLkW<SwrstCtl3LkSpec> {
        AesRstLkW::new(self, 31)
    }
}
#[doc = "Software Reset Control 3 Lock Register (SWRST_CTL3_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl3_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl3_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCtl3LkSpec;
impl crate::RegisterSpec for SwrstCtl3LkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_ctl3_lk::R`](R) reader structure"]
impl crate::Readable for SwrstCtl3LkSpec {}
#[doc = "`write(|w| ..)` method takes [`swrst_ctl3_lk::W`](W) writer structure"]
impl crate::Writable for SwrstCtl3LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRST_CTL3_LK to value 0"]
impl crate::Resettable for SwrstCtl3LkSpec {
    const RESET_VALUE: u32 = 0;
}
