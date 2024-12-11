#[doc = "Register `DEVALTH_LK` reader"]
pub type R = crate::R<DevalthLkSpec>;
#[doc = "Register `DEVALTH_LK` writer"]
pub type W = crate::W<DevalthLkSpec>;
#[doc = "Field `FCSI_TYP_LK` reader - FLM_CSI Type Lock"]
pub type FcsiTypLkR = crate::BitReader;
#[doc = "Field `FCSI_TYP_LK` writer - FLM_CSI Type Lock"]
pub type FcsiTypLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCY_IN_SL_LK` reader - DCY_IN Select Lock"]
pub type DcyInSlLkR = crate::BitReader;
#[doc = "Field `DCY_IN_SL_LK` writer - DCY_IN Select Lock"]
pub type DcyInSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCY_ECHO_OD_LK` reader - DCY_ECHO Open-Drain Select Lock"]
pub type DcyEchoOdLkR = crate::BitReader;
#[doc = "Field `DCY_ECHO_OD_LK` writer - DCY_ECHO Open-Drain Select Lock"]
pub type DcyEchoOdLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCY_ECHO_SL_LK` reader - DCY_ECHO Select Lock"]
pub type DcyEchoSlLkR = crate::BitReader;
#[doc = "Field `DCY_ECHO_SL_LK` writer - DCY_ECHO Select Lock"]
pub type DcyEchoSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_QUAD_LK` reader - FLM Quad Interface-Select Lock"]
pub type FlmQuadLkR = crate::BitReader;
#[doc = "Field `FLM_QUAD_LK` writer - FLM Quad Interface-Select Lock"]
pub type FlmQuadLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_MON_MD_LK` reader - FLM in Monitoring Mode Lock"]
pub type FlmMonMdLkR = crate::BitReader;
#[doc = "Field `FLM_MON_MD_LK` writer - FLM in Monitoring Mode Lock"]
pub type FlmMonMdLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_SL_LK` reader - FLM Interface-Select Lock"]
pub type FlmSlLkR = crate::BitReader;
#[doc = "Field `FLM_SL_LK` writer - FLM Interface-Select Lock"]
pub type FlmSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FLM_CSI Type Lock"]
    #[inline(always)]
    pub fn fcsi_typ_lk(&self) -> FcsiTypLkR {
        FcsiTypLkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCY_IN Select Lock"]
    #[inline(always)]
    pub fn dcy_in_sl_lk(&self) -> DcyInSlLkR {
        DcyInSlLkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DCY_ECHO Open-Drain Select Lock"]
    #[inline(always)]
    pub fn dcy_echo_od_lk(&self) -> DcyEchoOdLkR {
        DcyEchoOdLkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DCY_ECHO Select Lock"]
    #[inline(always)]
    pub fn dcy_echo_sl_lk(&self) -> DcyEchoSlLkR {
        DcyEchoSlLkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLM Quad Interface-Select Lock"]
    #[inline(always)]
    pub fn flm_quad_lk(&self) -> FlmQuadLkR {
        FlmQuadLkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLM in Monitoring Mode Lock"]
    #[inline(always)]
    pub fn flm_mon_md_lk(&self) -> FlmMonMdLkR {
        FlmMonMdLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLM Interface-Select Lock"]
    #[inline(always)]
    pub fn flm_sl_lk(&self) -> FlmSlLkR {
        FlmSlLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTH_LK")
            .field("fcsi_typ_lk", &self.fcsi_typ_lk())
            .field("dcy_in_sl_lk", &self.dcy_in_sl_lk())
            .field("dcy_echo_od_lk", &self.dcy_echo_od_lk())
            .field("dcy_echo_sl_lk", &self.dcy_echo_sl_lk())
            .field("flm_quad_lk", &self.flm_quad_lk())
            .field("flm_mon_md_lk", &self.flm_mon_md_lk())
            .field("flm_sl_lk", &self.flm_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - FLM_CSI Type Lock"]
    #[inline(always)]
    pub fn fcsi_typ_lk(&mut self) -> FcsiTypLkW<DevalthLkSpec> {
        FcsiTypLkW::new(self, 1)
    }
    #[doc = "Bit 2 - DCY_IN Select Lock"]
    #[inline(always)]
    pub fn dcy_in_sl_lk(&mut self) -> DcyInSlLkW<DevalthLkSpec> {
        DcyInSlLkW::new(self, 2)
    }
    #[doc = "Bit 3 - DCY_ECHO Open-Drain Select Lock"]
    #[inline(always)]
    pub fn dcy_echo_od_lk(&mut self) -> DcyEchoOdLkW<DevalthLkSpec> {
        DcyEchoOdLkW::new(self, 3)
    }
    #[doc = "Bit 4 - DCY_ECHO Select Lock"]
    #[inline(always)]
    pub fn dcy_echo_sl_lk(&mut self) -> DcyEchoSlLkW<DevalthLkSpec> {
        DcyEchoSlLkW::new(self, 4)
    }
    #[doc = "Bit 5 - FLM Quad Interface-Select Lock"]
    #[inline(always)]
    pub fn flm_quad_lk(&mut self) -> FlmQuadLkW<DevalthLkSpec> {
        FlmQuadLkW::new(self, 5)
    }
    #[doc = "Bit 6 - FLM in Monitoring Mode Lock"]
    #[inline(always)]
    pub fn flm_mon_md_lk(&mut self) -> FlmMonMdLkW<DevalthLkSpec> {
        FlmMonMdLkW::new(self, 6)
    }
    #[doc = "Bit 7 - FLM Interface-Select Lock"]
    #[inline(always)]
    pub fn flm_sl_lk(&mut self) -> FlmSlLkW<DevalthLkSpec> {
        FlmSlLkW::new(self, 7)
    }
}
#[doc = "Alternate Function H Lock Register (DEVALTH_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalth_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalth_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevalthLkSpec;
impl crate::RegisterSpec for DevalthLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalth_lk::R`](R) reader structure"]
impl crate::Readable for DevalthLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devalth_lk::W`](W) writer structure"]
impl crate::Writable for DevalthLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTH_LK to value 0"]
impl crate::Resettable for DevalthLkSpec {
    const RESET_VALUE: u8 = 0;
}
