#[doc = "Register `DEVALTH` reader"]
pub type R = crate::R<DevalthSpec>;
#[doc = "Register `DEVALTH` writer"]
pub type W = crate::W<DevalthSpec>;
#[doc = "Field `FCSI_TYP` reader - FLM_CSI Type"]
pub type FcsiTypR = crate::BitReader;
#[doc = "Field `FCSI_TYP` writer - FLM_CSI Type"]
pub type FcsiTypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCY_IN_SL` reader - DCY_IN Select"]
pub type DcyInSlR = crate::BitReader;
#[doc = "Field `DCY_IN_SL` writer - DCY_IN Select"]
pub type DcyInSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCY_ECHO_OD` reader - DCY_ECHO Open-Drain Select"]
pub type DcyEchoOdR = crate::BitReader;
#[doc = "Field `DCY_ECHO_OD` writer - DCY_ECHO Open-Drain Select"]
pub type DcyEchoOdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCY_ECHO_SL` reader - DCY_ECHO Select"]
pub type DcyEchoSlR = crate::BitReader;
#[doc = "Field `DCY_ECHO_SL` writer - DCY_ECHO Select"]
pub type DcyEchoSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_QUAD` reader - FLM Quad Interface-Select"]
pub type FlmQuadR = crate::BitReader;
#[doc = "Field `FLM_QUAD` writer - FLM Quad Interface-Select"]
pub type FlmQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_MON_MD` reader - FLM in Monitoring Mode"]
pub type FlmMonMdR = crate::BitReader;
#[doc = "Field `FLM_MON_MD` writer - FLM in Monitoring Mode"]
pub type FlmMonMdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLM_SL` reader - FLM Interface-Select"]
pub type FlmSlR = crate::BitReader;
#[doc = "Field `FLM_SL` writer - FLM Interface-Select"]
pub type FlmSlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FLM_CSI Type"]
    #[inline(always)]
    pub fn fcsi_typ(&self) -> FcsiTypR {
        FcsiTypR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCY_IN Select"]
    #[inline(always)]
    pub fn dcy_in_sl(&self) -> DcyInSlR {
        DcyInSlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DCY_ECHO Open-Drain Select"]
    #[inline(always)]
    pub fn dcy_echo_od(&self) -> DcyEchoOdR {
        DcyEchoOdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DCY_ECHO Select"]
    #[inline(always)]
    pub fn dcy_echo_sl(&self) -> DcyEchoSlR {
        DcyEchoSlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLM Quad Interface-Select"]
    #[inline(always)]
    pub fn flm_quad(&self) -> FlmQuadR {
        FlmQuadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLM in Monitoring Mode"]
    #[inline(always)]
    pub fn flm_mon_md(&self) -> FlmMonMdR {
        FlmMonMdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLM Interface-Select"]
    #[inline(always)]
    pub fn flm_sl(&self) -> FlmSlR {
        FlmSlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTH")
            .field("fcsi_typ", &self.fcsi_typ())
            .field("dcy_in_sl", &self.dcy_in_sl())
            .field("dcy_echo_od", &self.dcy_echo_od())
            .field("dcy_echo_sl", &self.dcy_echo_sl())
            .field("flm_quad", &self.flm_quad())
            .field("flm_mon_md", &self.flm_mon_md())
            .field("flm_sl", &self.flm_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - FLM_CSI Type"]
    #[inline(always)]
    pub fn fcsi_typ(&mut self) -> FcsiTypW<DevalthSpec> {
        FcsiTypW::new(self, 1)
    }
    #[doc = "Bit 2 - DCY_IN Select"]
    #[inline(always)]
    pub fn dcy_in_sl(&mut self) -> DcyInSlW<DevalthSpec> {
        DcyInSlW::new(self, 2)
    }
    #[doc = "Bit 3 - DCY_ECHO Open-Drain Select"]
    #[inline(always)]
    pub fn dcy_echo_od(&mut self) -> DcyEchoOdW<DevalthSpec> {
        DcyEchoOdW::new(self, 3)
    }
    #[doc = "Bit 4 - DCY_ECHO Select"]
    #[inline(always)]
    pub fn dcy_echo_sl(&mut self) -> DcyEchoSlW<DevalthSpec> {
        DcyEchoSlW::new(self, 4)
    }
    #[doc = "Bit 5 - FLM Quad Interface-Select"]
    #[inline(always)]
    pub fn flm_quad(&mut self) -> FlmQuadW<DevalthSpec> {
        FlmQuadW::new(self, 5)
    }
    #[doc = "Bit 6 - FLM in Monitoring Mode"]
    #[inline(always)]
    pub fn flm_mon_md(&mut self) -> FlmMonMdW<DevalthSpec> {
        FlmMonMdW::new(self, 6)
    }
    #[doc = "Bit 7 - FLM Interface-Select"]
    #[inline(always)]
    pub fn flm_sl(&mut self) -> FlmSlW<DevalthSpec> {
        FlmSlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function H Register (DEVALTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`devalth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devalth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevalthSpec;
impl crate::RegisterSpec for DevalthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devalth::R`](R) reader structure"]
impl crate::Readable for DevalthSpec {}
#[doc = "`write(|w| ..)` method takes [`devalth::W`](W) writer structure"]
impl crate::Writable for DevalthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTH to value 0"]
impl crate::Resettable for DevalthSpec {
    const RESET_VALUE: u8 = 0;
}
