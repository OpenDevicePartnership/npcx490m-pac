#[doc = "Register `FLM_CFG` reader"]
pub type R = crate::R<FlmCfgSpec>;
#[doc = "Register `FLM_CFG` writer"]
pub type W = crate::W<FlmCfgSpec>;
#[doc = "Field `DEVSIZ` reader - Device Size"]
pub type DevsizR = crate::FieldReader;
#[doc = "Field `DEVSIZ` writer - Device Size"]
pub type DevsizW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EBCHKDIS` reader - Extra Bits Check Disable"]
pub type EbchkdisR = crate::BitReader;
#[doc = "Field `EBCHKDIS` writer - Extra Bits Check Disable"]
pub type EbchkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOBYP` reader - FLM_CSIO Cut-Off Bypass"]
pub type CsobypR = crate::BitReader;
#[doc = "Field `CSOBYP` writer - FLM_CSIO Cut-Off Bypass"]
pub type CsobypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIAS` reader - Alias Flash Access Above Range"]
pub type AliasR = crate::BitReader;
#[doc = "Field `ALIAS` writer - Alias Flash Access Above Range"]
pub type AliasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Device Size"]
    #[inline(always)]
    pub fn devsiz(&self) -> DevsizR {
        DevsizR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Extra Bits Check Disable"]
    #[inline(always)]
    pub fn ebchkdis(&self) -> EbchkdisR {
        EbchkdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 14 - FLM_CSIO Cut-Off Bypass"]
    #[inline(always)]
    pub fn csobyp(&self) -> CsobypR {
        CsobypR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 29 - Alias Flash Access Above Range"]
    #[inline(always)]
    pub fn alias(&self) -> AliasR {
        AliasR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CFG")
            .field("devsiz", &self.devsiz())
            .field("ebchkdis", &self.ebchkdis())
            .field("csobyp", &self.csobyp())
            .field("alias", &self.alias())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Device Size"]
    #[inline(always)]
    pub fn devsiz(&mut self) -> DevsizW<FlmCfgSpec> {
        DevsizW::new(self, 0)
    }
    #[doc = "Bit 3 - Extra Bits Check Disable"]
    #[inline(always)]
    pub fn ebchkdis(&mut self) -> EbchkdisW<FlmCfgSpec> {
        EbchkdisW::new(self, 3)
    }
    #[doc = "Bit 14 - FLM_CSIO Cut-Off Bypass"]
    #[inline(always)]
    pub fn csobyp(&mut self) -> CsobypW<FlmCfgSpec> {
        CsobypW::new(self, 14)
    }
    #[doc = "Bit 29 - Alias Flash Access Above Range"]
    #[inline(always)]
    pub fn alias(&mut self) -> AliasW<FlmCfgSpec> {
        AliasW::new(self, 29)
    }
}
#[doc = "FLM Configuration Register (FLM_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCfgSpec;
impl crate::RegisterSpec for FlmCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cfg::R`](R) reader structure"]
impl crate::Readable for FlmCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_cfg::W`](W) writer structure"]
impl crate::Writable for FlmCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CFG to value 0"]
impl crate::Resettable for FlmCfgSpec {
    const RESET_VALUE: u32 = 0;
}
