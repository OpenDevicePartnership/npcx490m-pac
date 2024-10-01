#[doc = "Register `PSL_MCTL2` reader"]
pub type R = crate::R<PslMctl2Spec>;
#[doc = "Register `PSL_MCTL2` writer"]
pub type W = crate::W<PslMctl2Spec>;
#[doc = "Field `AC_IN_SEL` reader - AC_IN Input Select"]
pub type AcInSelR = crate::FieldReader;
#[doc = "Field `AC_IN_SEL` writer - AC_IN Input Select"]
pub type AcInSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AC_IN_BLOCK_EN` reader - AC_IN Blocking of LCT PSL Event, Enable"]
pub type AcInBlockEnR = crate::BitReader;
#[doc = "Field `AC_IN_BLOCK_EN` writer - AC_IN Blocking of LCT PSL Event, Enable"]
pub type AcInBlockEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_DB_EN` reader - PSL Debouncer Enable"]
pub type PslDbEnR = crate::BitReader;
#[doc = "Field `PSL_DB_EN` writer - PSL Debouncer Enable"]
pub type PslDbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_GP_EN` reader - PSL Glitch Protection Enable"]
pub type PslGpEnR = crate::BitReader;
#[doc = "Field `PSL_GP_EN` writer - PSL Glitch Protection Enable"]
pub type PslGpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCC1_RST_PSL_LK` reader - VCC1_RST Enable to PSL Lock"]
pub type Vcc1RstPslLkR = crate::BitReader;
#[doc = "Field `VCC1_RST_PSL_LK` writer - VCC1_RST Enable to PSL Lock"]
pub type Vcc1RstPslLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - AC_IN Input Select"]
    #[inline(always)]
    pub fn ac_in_sel(&self) -> AcInSelR {
        AcInSelR::new(self.bits & 3)
    }
    #[doc = "Bit 3 - AC_IN Blocking of LCT PSL Event, Enable"]
    #[inline(always)]
    pub fn ac_in_block_en(&self) -> AcInBlockEnR {
        AcInBlockEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSL Debouncer Enable"]
    #[inline(always)]
    pub fn psl_db_en(&self) -> PslDbEnR {
        PslDbEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - PSL Glitch Protection Enable"]
    #[inline(always)]
    pub fn psl_gp_en(&self) -> PslGpEnR {
        PslGpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VCC1_RST Enable to PSL Lock"]
    #[inline(always)]
    pub fn vcc1_rst_psl_lk(&self) -> Vcc1RstPslLkR {
        Vcc1RstPslLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSL_MCTL2")
            .field("ac_in_sel", &self.ac_in_sel())
            .field("ac_in_block_en", &self.ac_in_block_en())
            .field("psl_db_en", &self.psl_db_en())
            .field("psl_gp_en", &self.psl_gp_en())
            .field("vcc1_rst_psl_lk", &self.vcc1_rst_psl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - AC_IN Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn ac_in_sel(&mut self) -> AcInSelW<PslMctl2Spec> {
        AcInSelW::new(self, 0)
    }
    #[doc = "Bit 3 - AC_IN Blocking of LCT PSL Event, Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac_in_block_en(&mut self) -> AcInBlockEnW<PslMctl2Spec> {
        AcInBlockEnW::new(self, 3)
    }
    #[doc = "Bit 4 - PSL Debouncer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psl_db_en(&mut self) -> PslDbEnW<PslMctl2Spec> {
        PslDbEnW::new(self, 4)
    }
    #[doc = "Bit 6 - PSL Glitch Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psl_gp_en(&mut self) -> PslGpEnW<PslMctl2Spec> {
        PslGpEnW::new(self, 6)
    }
    #[doc = "Bit 7 - VCC1_RST Enable to PSL Lock"]
    #[inline(always)]
    #[must_use]
    pub fn vcc1_rst_psl_lk(&mut self) -> Vcc1RstPslLkW<PslMctl2Spec> {
        Vcc1RstPslLkW::new(self, 7)
    }
}
#[doc = "PSL Miscellaneous Control 2 Register (PSL_MCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_mctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_mctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PslMctl2Spec;
impl crate::RegisterSpec for PslMctl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psl_mctl2::R`](R) reader structure"]
impl crate::Readable for PslMctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`psl_mctl2::W`](W) writer structure"]
impl crate::Writable for PslMctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSL_MCTL2 to value 0"]
impl crate::Resettable for PslMctl2Spec {
    const RESET_VALUE: u8 = 0;
}
