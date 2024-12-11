#[doc = "Register `PSL_MCTL1` reader"]
pub type R = crate::R<PslMctl1Spec>;
#[doc = "Register `PSL_MCTL1` writer"]
pub type W = crate::W<PslMctl1Spec>;
#[doc = "Field `OD_EN` reader - Open-Drain Output Enable"]
pub type OdEnR = crate::BitReader;
#[doc = "Field `OD_EN` writer - Open-Drain Output Enable"]
pub type OdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS_EN` reader - Pulse Mode Enable"]
pub type PlsEnR = crate::BitReader;
#[doc = "Field `PLS_EN` writer - Pulse Mode Enable"]
pub type PlsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCC1_RST_EV` reader - VCC1_RST Event"]
pub type Vcc1RstEvR = crate::BitReader;
#[doc = "Field `VCC1_RST_EV` writer - VCC1_RST Event"]
pub type Vcc1RstEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCT_EV` reader - LCT Event"]
pub type LctEvR = crate::BitReader;
#[doc = "Field `LCT_EV` writer - LCT Event"]
pub type LctEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_GPO2_CTL` reader - PSL_GPO2 Control"]
pub type PslGpo2CtlR = crate::BitReader;
#[doc = "Field `PSL_GPO2_CTL` writer - PSL_GPO2 Control"]
pub type PslGpo2CtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_GPO1_CTL` reader - PSL_GPO1 Control"]
pub type PslGpo1CtlR = crate::BitReader;
#[doc = "Field `PSL_GPO1_CTL` writer - PSL_GPO1 Control"]
pub type PslGpo1CtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCC1_RST_PSL` reader - VCC1_RST Enable to PSL"]
pub type Vcc1RstPslR = crate::BitReader;
#[doc = "Field `VCC1_RST_PSL` writer - VCC1_RST Enable to PSL"]
pub type Vcc1RstPslW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Open-Drain Output Enable"]
    #[inline(always)]
    pub fn od_en(&self) -> OdEnR {
        OdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Mode Enable"]
    #[inline(always)]
    pub fn pls_en(&self) -> PlsEnR {
        PlsEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - VCC1_RST Event"]
    #[inline(always)]
    pub fn vcc1_rst_ev(&self) -> Vcc1RstEvR {
        Vcc1RstEvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCT Event"]
    #[inline(always)]
    pub fn lct_ev(&self) -> LctEvR {
        LctEvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSL_GPO2 Control"]
    #[inline(always)]
    pub fn psl_gpo2_ctl(&self) -> PslGpo2CtlR {
        PslGpo2CtlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSL_GPO1 Control"]
    #[inline(always)]
    pub fn psl_gpo1_ctl(&self) -> PslGpo1CtlR {
        PslGpo1CtlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VCC1_RST Enable to PSL"]
    #[inline(always)]
    pub fn vcc1_rst_psl(&self) -> Vcc1RstPslR {
        Vcc1RstPslR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSL_MCTL1")
            .field("od_en", &self.od_en())
            .field("pls_en", &self.pls_en())
            .field("vcc1_rst_ev", &self.vcc1_rst_ev())
            .field("lct_ev", &self.lct_ev())
            .field("psl_gpo2_ctl", &self.psl_gpo2_ctl())
            .field("psl_gpo1_ctl", &self.psl_gpo1_ctl())
            .field("vcc1_rst_psl", &self.vcc1_rst_psl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Open-Drain Output Enable"]
    #[inline(always)]
    pub fn od_en(&mut self) -> OdEnW<PslMctl1Spec> {
        OdEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse Mode Enable"]
    #[inline(always)]
    pub fn pls_en(&mut self) -> PlsEnW<PslMctl1Spec> {
        PlsEnW::new(self, 1)
    }
    #[doc = "Bit 3 - VCC1_RST Event"]
    #[inline(always)]
    pub fn vcc1_rst_ev(&mut self) -> Vcc1RstEvW<PslMctl1Spec> {
        Vcc1RstEvW::new(self, 3)
    }
    #[doc = "Bit 4 - LCT Event"]
    #[inline(always)]
    pub fn lct_ev(&mut self) -> LctEvW<PslMctl1Spec> {
        LctEvW::new(self, 4)
    }
    #[doc = "Bit 5 - PSL_GPO2 Control"]
    #[inline(always)]
    pub fn psl_gpo2_ctl(&mut self) -> PslGpo2CtlW<PslMctl1Spec> {
        PslGpo2CtlW::new(self, 5)
    }
    #[doc = "Bit 6 - PSL_GPO1 Control"]
    #[inline(always)]
    pub fn psl_gpo1_ctl(&mut self) -> PslGpo1CtlW<PslMctl1Spec> {
        PslGpo1CtlW::new(self, 6)
    }
    #[doc = "Bit 7 - VCC1_RST Enable to PSL"]
    #[inline(always)]
    pub fn vcc1_rst_psl(&mut self) -> Vcc1RstPslW<PslMctl1Spec> {
        Vcc1RstPslW::new(self, 7)
    }
}
#[doc = "PSL Miscellaneous Control 1 Register (PSL_MCTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_mctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_mctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PslMctl1Spec;
impl crate::RegisterSpec for PslMctl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psl_mctl1::R`](R) reader structure"]
impl crate::Readable for PslMctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`psl_mctl1::W`](W) writer structure"]
impl crate::Writable for PslMctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSL_MCTL1 to value 0"]
impl crate::Resettable for PslMctl1Spec {
    const RESET_VALUE: u8 = 0;
}
