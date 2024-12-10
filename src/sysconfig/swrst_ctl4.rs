#[doc = "Register `SWRST_CTL4` reader"]
pub type R = crate::R<SwrstCtl4Spec>;
#[doc = "Register `SWRST_CTL4` writer"]
pub type W = crate::W<SwrstCtl4Spec>;
#[doc = "Field `MDC_RST` reader - MDC Reset"]
pub type MdcRstR = crate::BitReader;
#[doc = "Field `MDC_RST` writer - MDC Reset"]
pub type MdcRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU0_RST` reader - FIU0 Reset"]
pub type Fiu0RstR = crate::BitReader;
#[doc = "Field `FIU0_RST` writer - FIU0 Reset"]
pub type Fiu0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIU1_RST` reader - FIU1 Reset"]
pub type Fiu1RstR = crate::BitReader;
#[doc = "Field `FIU1_RST` writer - FIU1 Reset"]
pub type Fiu1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA1_RST` reader - MDMA1 Reset"]
pub type Mdma1RstR = crate::BitReader;
#[doc = "Field `MDMA1_RST` writer - MDMA1 Reset"]
pub type Mdma1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA2_RST` reader - MDMA2 Reset"]
pub type Mdma2RstR = crate::BitReader;
#[doc = "Field `MDMA2_RST` writer - MDMA2 Reset"]
pub type Mdma2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA3_RST` reader - MDMA3 Reset"]
pub type Mdma3RstR = crate::BitReader;
#[doc = "Field `MDMA3_RST` writer - MDMA3 Reset"]
pub type Mdma3RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA4_RST` reader - MDMA4 Reset"]
pub type Mdma4RstR = crate::BitReader;
#[doc = "Field `MDMA4_RST` writer - MDMA4 Reset"]
pub type Mdma4RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA5_RST` reader - MDMA5 Reset"]
pub type Mdma5RstR = crate::BitReader;
#[doc = "Field `MDMA5_RST` writer - MDMA5 Reset"]
pub type Mdma5RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA6_RST` reader - MDMA6 Reset"]
pub type Mdma6RstR = crate::BitReader;
#[doc = "Field `MDMA6_RST` writer - MDMA6 Reset"]
pub type Mdma6RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA7_RST` reader - MDMA7 Reset"]
pub type Mdma7RstR = crate::BitReader;
#[doc = "Field `MDMA7_RST` writer - MDMA7 Reset"]
pub type Mdma7RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_RST` reader - FLM Reset"]
pub type FlmRstR = crate::BitReader;
#[doc = "Field `FLM_RST` writer - FLM Reset"]
pub type FlmRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - MDC Reset"]
    #[inline(always)]
    pub fn mdc_rst(&self) -> MdcRstR {
        MdcRstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FIU0 Reset"]
    #[inline(always)]
    pub fn fiu0_rst(&self) -> Fiu0RstR {
        Fiu0RstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FIU1 Reset"]
    #[inline(always)]
    pub fn fiu1_rst(&self) -> Fiu1RstR {
        Fiu1RstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - MDMA1 Reset"]
    #[inline(always)]
    pub fn mdma1_rst(&self) -> Mdma1RstR {
        Mdma1RstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MDMA2 Reset"]
    #[inline(always)]
    pub fn mdma2_rst(&self) -> Mdma2RstR {
        Mdma2RstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MDMA3 Reset"]
    #[inline(always)]
    pub fn mdma3_rst(&self) -> Mdma3RstR {
        Mdma3RstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MDMA4 Reset"]
    #[inline(always)]
    pub fn mdma4_rst(&self) -> Mdma4RstR {
        Mdma4RstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MDMA5 Reset"]
    #[inline(always)]
    pub fn mdma5_rst(&self) -> Mdma5RstR {
        Mdma5RstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MDMA6 Reset"]
    #[inline(always)]
    pub fn mdma6_rst(&self) -> Mdma6RstR {
        Mdma6RstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MDMA7 Reset"]
    #[inline(always)]
    pub fn mdma7_rst(&self) -> Mdma7RstR {
        Mdma7RstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLM Reset"]
    #[inline(always)]
    pub fn flm_rst(&self) -> FlmRstR {
        FlmRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRST_CTL4")
            .field("mdc_rst", &self.mdc_rst())
            .field("fiu0_rst", &self.fiu0_rst())
            .field("fiu1_rst", &self.fiu1_rst())
            .field("mdma1_rst", &self.mdma1_rst())
            .field("mdma2_rst", &self.mdma2_rst())
            .field("mdma3_rst", &self.mdma3_rst())
            .field("mdma4_rst", &self.mdma4_rst())
            .field("mdma5_rst", &self.mdma5_rst())
            .field("mdma6_rst", &self.mdma6_rst())
            .field("mdma7_rst", &self.mdma7_rst())
            .field("flm_rst", &self.flm_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 15 - MDC Reset"]
    #[inline(always)]
    pub fn mdc_rst(&mut self) -> MdcRstW<SwrstCtl4Spec> {
        MdcRstW::new(self, 15)
    }
    #[doc = "Bit 16 - FIU0 Reset"]
    #[inline(always)]
    pub fn fiu0_rst(&mut self) -> Fiu0RstW<SwrstCtl4Spec> {
        Fiu0RstW::new(self, 16)
    }
    #[doc = "Bit 17 - FIU1 Reset"]
    #[inline(always)]
    pub fn fiu1_rst(&mut self) -> Fiu1RstW<SwrstCtl4Spec> {
        Fiu1RstW::new(self, 17)
    }
    #[doc = "Bit 24 - MDMA1 Reset"]
    #[inline(always)]
    pub fn mdma1_rst(&mut self) -> Mdma1RstW<SwrstCtl4Spec> {
        Mdma1RstW::new(self, 24)
    }
    #[doc = "Bit 25 - MDMA2 Reset"]
    #[inline(always)]
    pub fn mdma2_rst(&mut self) -> Mdma2RstW<SwrstCtl4Spec> {
        Mdma2RstW::new(self, 25)
    }
    #[doc = "Bit 26 - MDMA3 Reset"]
    #[inline(always)]
    pub fn mdma3_rst(&mut self) -> Mdma3RstW<SwrstCtl4Spec> {
        Mdma3RstW::new(self, 26)
    }
    #[doc = "Bit 27 - MDMA4 Reset"]
    #[inline(always)]
    pub fn mdma4_rst(&mut self) -> Mdma4RstW<SwrstCtl4Spec> {
        Mdma4RstW::new(self, 27)
    }
    #[doc = "Bit 28 - MDMA5 Reset"]
    #[inline(always)]
    pub fn mdma5_rst(&mut self) -> Mdma5RstW<SwrstCtl4Spec> {
        Mdma5RstW::new(self, 28)
    }
    #[doc = "Bit 29 - MDMA6 Reset"]
    #[inline(always)]
    pub fn mdma6_rst(&mut self) -> Mdma6RstW<SwrstCtl4Spec> {
        Mdma6RstW::new(self, 29)
    }
    #[doc = "Bit 30 - MDMA7 Reset"]
    #[inline(always)]
    pub fn mdma7_rst(&mut self) -> Mdma7RstW<SwrstCtl4Spec> {
        Mdma7RstW::new(self, 30)
    }
    #[doc = "Bit 31 - FLM Reset"]
    #[inline(always)]
    pub fn flm_rst(&mut self) -> FlmRstW<SwrstCtl4Spec> {
        FlmRstW::new(self, 31)
    }
}
#[doc = "Software Reset Control 4 Register (SWRST_CTL4)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCtl4Spec;
impl crate::RegisterSpec for SwrstCtl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_ctl4::R`](R) reader structure"]
impl crate::Readable for SwrstCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`swrst_ctl4::W`](W) writer structure"]
impl crate::Writable for SwrstCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRST_CTL4 to value 0"]
impl crate::Resettable for SwrstCtl4Spec {
    const RESET_VALUE: u32 = 0;
}
