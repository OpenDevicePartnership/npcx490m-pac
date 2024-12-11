#[doc = "Register `SWRST_CTL3` reader"]
pub type R = crate::R<SwrstCtl3Spec>;
#[doc = "Register `SWRST_CTL3` writer"]
pub type W = crate::W<SwrstCtl3Spec>;
#[doc = "Field `SIOCFG_RST` reader - SuperI/O Configuration Reset"]
pub type SiocfgRstR = crate::BitReader;
#[doc = "Field `SIOCFG_RST` writer - SuperI/O Configuration Reset"]
pub type SiocfgRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERPORT_RST` reader - Serial Port Reset"]
pub type SerportRstR = crate::BitReader;
#[doc = "Field `SERPORT_RST` writer - Serial Port Reset"]
pub type SerportRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C1_RST` reader - I3CI1 Reset"]
pub type I3c1RstR = crate::BitReader;
#[doc = "Field `I3C1_RST` writer - I3CI1 Reset"]
pub type I3c1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2_RST` reader - I3CI2 Reset"]
pub type I3c2RstR = crate::BitReader;
#[doc = "Field `I3C2_RST` writer - I3CI2 Reset"]
pub type I3c2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3_RST` reader - I3CI3 Reset"]
pub type I3c3RstR = crate::BitReader;
#[doc = "Field `I3C3_RST` writer - I3CI3 Reset"]
pub type I3c3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSWC_RST` reader - MSWC Reset"]
pub type MswcRstR = crate::BitReader;
#[doc = "Field `MSWC_RST` writer - MSWC Reset"]
pub type MswcRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHM_RST` reader - SHM Reset"]
pub type ShmRstR = crate::BitReader;
#[doc = "Field `SHM_RST` writer - SHM Reset"]
pub type ShmRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH1_RST` reader - PM Channel 1 Reset"]
pub type Pmch1RstR = crate::BitReader;
#[doc = "Field `PMCH1_RST` writer - PM Channel 1 Reset"]
pub type Pmch1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH2_RST` reader - PM Channel 2 Reset"]
pub type Pmch2RstR = crate::BitReader;
#[doc = "Field `PMCH2_RST` writer - PM Channel 2 Reset"]
pub type Pmch2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH3_RST` reader - PM Channel 3 Reset"]
pub type Pmch3RstR = crate::BitReader;
#[doc = "Field `PMCH3_RST` writer - PM Channel 3 Reset"]
pub type Pmch3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMCH4_RST` reader - PM Channel 4 Reset"]
pub type Pmch4RstR = crate::BitReader;
#[doc = "Field `PMCH4_RST` writer - PM Channel 4 Reset"]
pub type Pmch4RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBC_RST` reader - KBC Reset"]
pub type KbcRstR = crate::BitReader;
#[doc = "Field `KBC_RST` writer - KBC Reset"]
pub type KbcRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2HOST_RST` reader - Core-to-Host Reset"]
pub type C2hostRstR = crate::BitReader;
#[doc = "Field `C2HOST_RST` writer - Core-to-Host Reset"]
pub type C2hostRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT3_RST` reader - CR_UART 3 Reset"]
pub type Crurt3RstR = crate::BitReader;
#[doc = "Field `CRURT3_RST` writer - CR_UART 3 Reset"]
pub type Crurt3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRURT4_RST` reader - CR_UART 4 Reset"]
pub type Crurt4RstR = crate::BitReader;
#[doc = "Field `CRURT4_RST` writer - CR_UART 4 Reset"]
pub type Crurt4RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFCG_RST` reader - LFCG Reset"]
pub type LfcgRstR = crate::BitReader;
#[doc = "Field `LFCG_RST` writer - LFCG Reset"]
pub type LfcgRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_RST` reader - Development Reset"]
pub type DevRstR = crate::BitReader;
#[doc = "Field `DEV_RST` writer - Development Reset"]
pub type DevRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG_RST` reader - SYSCFG Reset"]
pub type SyscfgRstR = crate::BitReader;
#[doc = "Field `SYSCFG_RST` writer - SYSCFG Reset"]
pub type SyscfgRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBRM_RST` reader - BBRM Reset"]
pub type BbrmRstR = crate::BitReader;
#[doc = "Field `BBRM_RST` writer - BBRM Reset"]
pub type BbrmRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHAB_RST` reader - SHAb Reset"]
pub type ShabRstR = crate::BitReader;
#[doc = "Field `SHAB_RST` writer - SHAb Reset"]
pub type ShabRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTPI_RST` reader - OTPI Reset"]
pub type OtpiRstR = crate::BitReader;
#[doc = "Field `OTPI_RST` writer - OTPI Reset"]
pub type OtpiRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_RST` reader - RNG Reset"]
pub type RngRstR = crate::BitReader;
#[doc = "Field `RNG_RST` writer - RNG Reset"]
pub type RngRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHAA_RST` reader - SHAa Reset"]
pub type ShaaRstR = crate::BitReader;
#[doc = "Field `SHAA_RST` writer - SHAa Reset"]
pub type ShaaRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_RST` reader - PKA Reset"]
pub type PkaRstR = crate::BitReader;
#[doc = "Field `PKA_RST` writer - PKA Reset"]
pub type PkaRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_RST` reader - AES Reset"]
pub type AesRstR = crate::BitReader;
#[doc = "Field `AES_RST` writer - AES Reset"]
pub type AesRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SuperI/O Configuration Reset"]
    #[inline(always)]
    pub fn siocfg_rst(&self) -> SiocfgRstR {
        SiocfgRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Port Reset"]
    #[inline(always)]
    pub fn serport_rst(&self) -> SerportRstR {
        SerportRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - I3CI1 Reset"]
    #[inline(always)]
    pub fn i3c1_rst(&self) -> I3c1RstR {
        I3c1RstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I3CI2 Reset"]
    #[inline(always)]
    pub fn i3c2_rst(&self) -> I3c2RstR {
        I3c2RstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I3CI3 Reset"]
    #[inline(always)]
    pub fn i3c3_rst(&self) -> I3c3RstR {
        I3c3RstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MSWC Reset"]
    #[inline(always)]
    pub fn mswc_rst(&self) -> MswcRstR {
        MswcRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SHM Reset"]
    #[inline(always)]
    pub fn shm_rst(&self) -> ShmRstR {
        ShmRstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PM Channel 1 Reset"]
    #[inline(always)]
    pub fn pmch1_rst(&self) -> Pmch1RstR {
        Pmch1RstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PM Channel 2 Reset"]
    #[inline(always)]
    pub fn pmch2_rst(&self) -> Pmch2RstR {
        Pmch2RstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PM Channel 3 Reset"]
    #[inline(always)]
    pub fn pmch3_rst(&self) -> Pmch3RstR {
        Pmch3RstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PM Channel 4 Reset"]
    #[inline(always)]
    pub fn pmch4_rst(&self) -> Pmch4RstR {
        Pmch4RstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - KBC Reset"]
    #[inline(always)]
    pub fn kbc_rst(&self) -> KbcRstR {
        KbcRstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Core-to-Host Reset"]
    #[inline(always)]
    pub fn c2host_rst(&self) -> C2hostRstR {
        C2hostRstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - CR_UART 3 Reset"]
    #[inline(always)]
    pub fn crurt3_rst(&self) -> Crurt3RstR {
        Crurt3RstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CR_UART 4 Reset"]
    #[inline(always)]
    pub fn crurt4_rst(&self) -> Crurt4RstR {
        Crurt4RstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LFCG Reset"]
    #[inline(always)]
    pub fn lfcg_rst(&self) -> LfcgRstR {
        LfcgRstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Development Reset"]
    #[inline(always)]
    pub fn dev_rst(&self) -> DevRstR {
        DevRstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SYSCFG Reset"]
    #[inline(always)]
    pub fn syscfg_rst(&self) -> SyscfgRstR {
        SyscfgRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - BBRM Reset"]
    #[inline(always)]
    pub fn bbrm_rst(&self) -> BbrmRstR {
        BbrmRstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SHAb Reset"]
    #[inline(always)]
    pub fn shab_rst(&self) -> ShabRstR {
        ShabRstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OTPI Reset"]
    #[inline(always)]
    pub fn otpi_rst(&self) -> OtpiRstR {
        OtpiRstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RNG Reset"]
    #[inline(always)]
    pub fn rng_rst(&self) -> RngRstR {
        RngRstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SHAa Reset"]
    #[inline(always)]
    pub fn shaa_rst(&self) -> ShaaRstR {
        ShaaRstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PKA Reset"]
    #[inline(always)]
    pub fn pka_rst(&self) -> PkaRstR {
        PkaRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AES Reset"]
    #[inline(always)]
    pub fn aes_rst(&self) -> AesRstR {
        AesRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRST_CTL3")
            .field("siocfg_rst", &self.siocfg_rst())
            .field("serport_rst", &self.serport_rst())
            .field("i3c1_rst", &self.i3c1_rst())
            .field("i3c2_rst", &self.i3c2_rst())
            .field("i3c3_rst", &self.i3c3_rst())
            .field("mswc_rst", &self.mswc_rst())
            .field("shm_rst", &self.shm_rst())
            .field("pmch1_rst", &self.pmch1_rst())
            .field("pmch2_rst", &self.pmch2_rst())
            .field("pmch3_rst", &self.pmch3_rst())
            .field("pmch4_rst", &self.pmch4_rst())
            .field("kbc_rst", &self.kbc_rst())
            .field("c2host_rst", &self.c2host_rst())
            .field("crurt3_rst", &self.crurt3_rst())
            .field("crurt4_rst", &self.crurt4_rst())
            .field("lfcg_rst", &self.lfcg_rst())
            .field("dev_rst", &self.dev_rst())
            .field("syscfg_rst", &self.syscfg_rst())
            .field("bbrm_rst", &self.bbrm_rst())
            .field("shab_rst", &self.shab_rst())
            .field("otpi_rst", &self.otpi_rst())
            .field("rng_rst", &self.rng_rst())
            .field("shaa_rst", &self.shaa_rst())
            .field("pka_rst", &self.pka_rst())
            .field("aes_rst", &self.aes_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SuperI/O Configuration Reset"]
    #[inline(always)]
    pub fn siocfg_rst(&mut self) -> SiocfgRstW<SwrstCtl3Spec> {
        SiocfgRstW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial Port Reset"]
    #[inline(always)]
    pub fn serport_rst(&mut self) -> SerportRstW<SwrstCtl3Spec> {
        SerportRstW::new(self, 1)
    }
    #[doc = "Bit 4 - I3CI1 Reset"]
    #[inline(always)]
    pub fn i3c1_rst(&mut self) -> I3c1RstW<SwrstCtl3Spec> {
        I3c1RstW::new(self, 4)
    }
    #[doc = "Bit 5 - I3CI2 Reset"]
    #[inline(always)]
    pub fn i3c2_rst(&mut self) -> I3c2RstW<SwrstCtl3Spec> {
        I3c2RstW::new(self, 5)
    }
    #[doc = "Bit 6 - I3CI3 Reset"]
    #[inline(always)]
    pub fn i3c3_rst(&mut self) -> I3c3RstW<SwrstCtl3Spec> {
        I3c3RstW::new(self, 6)
    }
    #[doc = "Bit 8 - MSWC Reset"]
    #[inline(always)]
    pub fn mswc_rst(&mut self) -> MswcRstW<SwrstCtl3Spec> {
        MswcRstW::new(self, 8)
    }
    #[doc = "Bit 9 - SHM Reset"]
    #[inline(always)]
    pub fn shm_rst(&mut self) -> ShmRstW<SwrstCtl3Spec> {
        ShmRstW::new(self, 9)
    }
    #[doc = "Bit 10 - PM Channel 1 Reset"]
    #[inline(always)]
    pub fn pmch1_rst(&mut self) -> Pmch1RstW<SwrstCtl3Spec> {
        Pmch1RstW::new(self, 10)
    }
    #[doc = "Bit 11 - PM Channel 2 Reset"]
    #[inline(always)]
    pub fn pmch2_rst(&mut self) -> Pmch2RstW<SwrstCtl3Spec> {
        Pmch2RstW::new(self, 11)
    }
    #[doc = "Bit 12 - PM Channel 3 Reset"]
    #[inline(always)]
    pub fn pmch3_rst(&mut self) -> Pmch3RstW<SwrstCtl3Spec> {
        Pmch3RstW::new(self, 12)
    }
    #[doc = "Bit 13 - PM Channel 4 Reset"]
    #[inline(always)]
    pub fn pmch4_rst(&mut self) -> Pmch4RstW<SwrstCtl3Spec> {
        Pmch4RstW::new(self, 13)
    }
    #[doc = "Bit 15 - KBC Reset"]
    #[inline(always)]
    pub fn kbc_rst(&mut self) -> KbcRstW<SwrstCtl3Spec> {
        KbcRstW::new(self, 15)
    }
    #[doc = "Bit 16 - Core-to-Host Reset"]
    #[inline(always)]
    pub fn c2host_rst(&mut self) -> C2hostRstW<SwrstCtl3Spec> {
        C2hostRstW::new(self, 16)
    }
    #[doc = "Bit 18 - CR_UART 3 Reset"]
    #[inline(always)]
    pub fn crurt3_rst(&mut self) -> Crurt3RstW<SwrstCtl3Spec> {
        Crurt3RstW::new(self, 18)
    }
    #[doc = "Bit 19 - CR_UART 4 Reset"]
    #[inline(always)]
    pub fn crurt4_rst(&mut self) -> Crurt4RstW<SwrstCtl3Spec> {
        Crurt4RstW::new(self, 19)
    }
    #[doc = "Bit 20 - LFCG Reset"]
    #[inline(always)]
    pub fn lfcg_rst(&mut self) -> LfcgRstW<SwrstCtl3Spec> {
        LfcgRstW::new(self, 20)
    }
    #[doc = "Bit 22 - Development Reset"]
    #[inline(always)]
    pub fn dev_rst(&mut self) -> DevRstW<SwrstCtl3Spec> {
        DevRstW::new(self, 22)
    }
    #[doc = "Bit 23 - SYSCFG Reset"]
    #[inline(always)]
    pub fn syscfg_rst(&mut self) -> SyscfgRstW<SwrstCtl3Spec> {
        SyscfgRstW::new(self, 23)
    }
    #[doc = "Bit 25 - BBRM Reset"]
    #[inline(always)]
    pub fn bbrm_rst(&mut self) -> BbrmRstW<SwrstCtl3Spec> {
        BbrmRstW::new(self, 25)
    }
    #[doc = "Bit 26 - SHAb Reset"]
    #[inline(always)]
    pub fn shab_rst(&mut self) -> ShabRstW<SwrstCtl3Spec> {
        ShabRstW::new(self, 26)
    }
    #[doc = "Bit 27 - OTPI Reset"]
    #[inline(always)]
    pub fn otpi_rst(&mut self) -> OtpiRstW<SwrstCtl3Spec> {
        OtpiRstW::new(self, 27)
    }
    #[doc = "Bit 28 - RNG Reset"]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RngRstW<SwrstCtl3Spec> {
        RngRstW::new(self, 28)
    }
    #[doc = "Bit 29 - SHAa Reset"]
    #[inline(always)]
    pub fn shaa_rst(&mut self) -> ShaaRstW<SwrstCtl3Spec> {
        ShaaRstW::new(self, 29)
    }
    #[doc = "Bit 30 - PKA Reset"]
    #[inline(always)]
    pub fn pka_rst(&mut self) -> PkaRstW<SwrstCtl3Spec> {
        PkaRstW::new(self, 30)
    }
    #[doc = "Bit 31 - AES Reset"]
    #[inline(always)]
    pub fn aes_rst(&mut self) -> AesRstW<SwrstCtl3Spec> {
        AesRstW::new(self, 31)
    }
}
#[doc = "Software Reset Control 3 Register (SWRST_CTL3)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCtl3Spec;
impl crate::RegisterSpec for SwrstCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_ctl3::R`](R) reader structure"]
impl crate::Readable for SwrstCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`swrst_ctl3::W`](W) writer structure"]
impl crate::Writable for SwrstCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRST_CTL3 to value 0"]
impl crate::Resettable for SwrstCtl3Spec {
    const RESET_VALUE: u32 = 0;
}
