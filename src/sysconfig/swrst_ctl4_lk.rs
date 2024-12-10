#[doc = "Register `SWRST_CTL4_LK` reader"]
pub type R = crate::R<SwrstCtl4LkSpec>;
#[doc = "Register `SWRST_CTL4_LK` writer"]
pub type W = crate::W<SwrstCtl4LkSpec>;
#[doc = "Field `MDC_RST_LK` reader - MDC Reset Lock"]
pub type MdcRstLkR = crate::BitReader;
#[doc = "Field `MDC_RST_LK` writer - MDC Reset Lock"]
pub type MdcRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU0_RST_LK` reader - FIU0 Reset Lock"]
pub type Fiu0RstLkR = crate::BitReader;
#[doc = "Field `FIU0_RST_LK` writer - FIU0 Reset Lock"]
pub type Fiu0RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU1_RST_LK` reader - FIU1 Reset Lock"]
pub type Fiu1RstLkR = crate::BitReader;
#[doc = "Field `FIU1_RST_LK` writer - FIU1 Reset Lock"]
pub type Fiu1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA1_RST_LK` reader - MDMA1 Reset Lock"]
pub type Mdma1RstLkR = crate::BitReader;
#[doc = "Field `MDMA1_RST_LK` writer - MDMA1 Reset Lock"]
pub type Mdma1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA2_RST_LK` reader - MDMA2 Reset Lock"]
pub type Mdma2RstLkR = crate::BitReader;
#[doc = "Field `MDMA2_RST_LK` writer - MDMA2 Reset Lock"]
pub type Mdma2RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA3_RST_LK` reader - MDMA3 Reset Lock"]
pub type Mdma3RstLkR = crate::BitReader;
#[doc = "Field `MDMA3_RST_LK` writer - MDMA3 Reset Lock"]
pub type Mdma3RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA4_RST_LK` reader - MDMA4 Reset Lock"]
pub type Mdma4RstLkR = crate::BitReader;
#[doc = "Field `MDMA4_RST_LK` writer - MDMA4 Reset Lock"]
pub type Mdma4RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA5_RST_LK` reader - MDMA5 Reset Lock"]
pub type Mdma5RstLkR = crate::BitReader;
#[doc = "Field `MDMA5_RST_LK` writer - MDMA5 Reset Lock"]
pub type Mdma5RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA6_RST_LK` reader - MDMA6 Reset Lock"]
pub type Mdma6RstLkR = crate::BitReader;
#[doc = "Field `MDMA6_RST_LK` writer - MDMA6 Reset Lock"]
pub type Mdma6RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA7_RST_LK` reader - MDMA7 Reset Lock"]
pub type Mdma7RstLkR = crate::BitReader;
#[doc = "Field `MDMA7_RST_LK` writer - MDMA7 Reset Lock"]
pub type Mdma7RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_RST_LK` reader - FLM Reset Lock"]
pub type FlmRstLkR = crate::BitReader;
#[doc = "Field `FLM_RST_LK` writer - FLM Reset Lock"]
pub type FlmRstLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - MDC Reset Lock"]
    #[inline(always)]
    pub fn mdc_rst_lk(&self) -> MdcRstLkR {
        MdcRstLkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FIU0 Reset Lock"]
    #[inline(always)]
    pub fn fiu0_rst_lk(&self) -> Fiu0RstLkR {
        Fiu0RstLkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FIU1 Reset Lock"]
    #[inline(always)]
    pub fn fiu1_rst_lk(&self) -> Fiu1RstLkR {
        Fiu1RstLkR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - MDMA1 Reset Lock"]
    #[inline(always)]
    pub fn mdma1_rst_lk(&self) -> Mdma1RstLkR {
        Mdma1RstLkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MDMA2 Reset Lock"]
    #[inline(always)]
    pub fn mdma2_rst_lk(&self) -> Mdma2RstLkR {
        Mdma2RstLkR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MDMA3 Reset Lock"]
    #[inline(always)]
    pub fn mdma3_rst_lk(&self) -> Mdma3RstLkR {
        Mdma3RstLkR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MDMA4 Reset Lock"]
    #[inline(always)]
    pub fn mdma4_rst_lk(&self) -> Mdma4RstLkR {
        Mdma4RstLkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MDMA5 Reset Lock"]
    #[inline(always)]
    pub fn mdma5_rst_lk(&self) -> Mdma5RstLkR {
        Mdma5RstLkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MDMA6 Reset Lock"]
    #[inline(always)]
    pub fn mdma6_rst_lk(&self) -> Mdma6RstLkR {
        Mdma6RstLkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MDMA7 Reset Lock"]
    #[inline(always)]
    pub fn mdma7_rst_lk(&self) -> Mdma7RstLkR {
        Mdma7RstLkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLM Reset Lock"]
    #[inline(always)]
    pub fn flm_rst_lk(&self) -> FlmRstLkR {
        FlmRstLkR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRST_CTL4_LK")
            .field("mdc_rst_lk", &self.mdc_rst_lk())
            .field("fiu0_rst_lk", &self.fiu0_rst_lk())
            .field("fiu1_rst_lk", &self.fiu1_rst_lk())
            .field("mdma1_rst_lk", &self.mdma1_rst_lk())
            .field("mdma2_rst_lk", &self.mdma2_rst_lk())
            .field("mdma3_rst_lk", &self.mdma3_rst_lk())
            .field("mdma4_rst_lk", &self.mdma4_rst_lk())
            .field("mdma5_rst_lk", &self.mdma5_rst_lk())
            .field("mdma6_rst_lk", &self.mdma6_rst_lk())
            .field("mdma7_rst_lk", &self.mdma7_rst_lk())
            .field("flm_rst_lk", &self.flm_rst_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 15 - MDC Reset Lock"]
    #[inline(always)]
    pub fn mdc_rst_lk(&mut self) -> MdcRstLkW<SwrstCtl4LkSpec> {
        MdcRstLkW::new(self, 15)
    }
    #[doc = "Bit 16 - FIU0 Reset Lock"]
    #[inline(always)]
    pub fn fiu0_rst_lk(&mut self) -> Fiu0RstLkW<SwrstCtl4LkSpec> {
        Fiu0RstLkW::new(self, 16)
    }
    #[doc = "Bit 17 - FIU1 Reset Lock"]
    #[inline(always)]
    pub fn fiu1_rst_lk(&mut self) -> Fiu1RstLkW<SwrstCtl4LkSpec> {
        Fiu1RstLkW::new(self, 17)
    }
    #[doc = "Bit 24 - MDMA1 Reset Lock"]
    #[inline(always)]
    pub fn mdma1_rst_lk(&mut self) -> Mdma1RstLkW<SwrstCtl4LkSpec> {
        Mdma1RstLkW::new(self, 24)
    }
    #[doc = "Bit 25 - MDMA2 Reset Lock"]
    #[inline(always)]
    pub fn mdma2_rst_lk(&mut self) -> Mdma2RstLkW<SwrstCtl4LkSpec> {
        Mdma2RstLkW::new(self, 25)
    }
    #[doc = "Bit 26 - MDMA3 Reset Lock"]
    #[inline(always)]
    pub fn mdma3_rst_lk(&mut self) -> Mdma3RstLkW<SwrstCtl4LkSpec> {
        Mdma3RstLkW::new(self, 26)
    }
    #[doc = "Bit 27 - MDMA4 Reset Lock"]
    #[inline(always)]
    pub fn mdma4_rst_lk(&mut self) -> Mdma4RstLkW<SwrstCtl4LkSpec> {
        Mdma4RstLkW::new(self, 27)
    }
    #[doc = "Bit 28 - MDMA5 Reset Lock"]
    #[inline(always)]
    pub fn mdma5_rst_lk(&mut self) -> Mdma5RstLkW<SwrstCtl4LkSpec> {
        Mdma5RstLkW::new(self, 28)
    }
    #[doc = "Bit 29 - MDMA6 Reset Lock"]
    #[inline(always)]
    pub fn mdma6_rst_lk(&mut self) -> Mdma6RstLkW<SwrstCtl4LkSpec> {
        Mdma6RstLkW::new(self, 29)
    }
    #[doc = "Bit 30 - MDMA7 Reset Lock"]
    #[inline(always)]
    pub fn mdma7_rst_lk(&mut self) -> Mdma7RstLkW<SwrstCtl4LkSpec> {
        Mdma7RstLkW::new(self, 30)
    }
    #[doc = "Bit 31 - FLM Reset Lock"]
    #[inline(always)]
    pub fn flm_rst_lk(&mut self) -> FlmRstLkW<SwrstCtl4LkSpec> {
        FlmRstLkW::new(self, 31)
    }
}
#[doc = "Software Reset Control 4 Lock Register (SWRST_CTL4_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl4_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl4_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCtl4LkSpec;
impl crate::RegisterSpec for SwrstCtl4LkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_ctl4_lk::R`](R) reader structure"]
impl crate::Readable for SwrstCtl4LkSpec {}
#[doc = "`write(|w| ..)` method takes [`swrst_ctl4_lk::W`](W) writer structure"]
impl crate::Writable for SwrstCtl4LkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRST_CTL4_LK to value 0"]
impl crate::Resettable for SwrstCtl4LkSpec {
    const RESET_VALUE: u32 = 0;
}
