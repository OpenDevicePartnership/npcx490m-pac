#[doc = "Register `FLM_IE` reader"]
pub type R = crate::R<FlmIeSpec>;
#[doc = "Register `FLM_IE` writer"]
pub type W = crate::W<FlmIeSpec>;
#[doc = "Field `RJ_IE` reader - RJ_EV Interrupt Enable"]
pub type RjIeR = crate::BitReader;
#[doc = "Field `RJ_IE` writer - RJ_EV Interrupt Enable"]
pub type RjIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_IE` reader - CSI_EV Interrupt Enable"]
pub type CsiIeR = crate::BitReader;
#[doc = "Field `CSI_IE` writer - CSI_EV Interrupt Enable"]
pub type CsiIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCR_IE` reader - TCR_EV Interrupt Enable"]
pub type TcrIeR = crate::BitReader;
#[doc = "Field `TCR_IE` writer - TCR_EV Interrupt Enable"]
pub type TcrIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RJ_EV Interrupt Enable"]
    #[inline(always)]
    pub fn rj_ie(&self) -> RjIeR {
        RjIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSI_EV Interrupt Enable"]
    #[inline(always)]
    pub fn csi_ie(&self) -> CsiIeR {
        CsiIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - TCR_EV Interrupt Enable"]
    #[inline(always)]
    pub fn tcr_ie(&self) -> TcrIeR {
        TcrIeR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_IE")
            .field("rj_ie", &self.rj_ie())
            .field("csi_ie", &self.csi_ie())
            .field("tcr_ie", &self.tcr_ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RJ_EV Interrupt Enable"]
    #[inline(always)]
    pub fn rj_ie(&mut self) -> RjIeW<FlmIeSpec> {
        RjIeW::new(self, 0)
    }
    #[doc = "Bit 1 - CSI_EV Interrupt Enable"]
    #[inline(always)]
    pub fn csi_ie(&mut self) -> CsiIeW<FlmIeSpec> {
        CsiIeW::new(self, 1)
    }
    #[doc = "Bit 3 - TCR_EV Interrupt Enable"]
    #[inline(always)]
    pub fn tcr_ie(&mut self) -> TcrIeW<FlmIeSpec> {
        TcrIeW::new(self, 3)
    }
}
#[doc = "FLM Interrupt Enable Register (FLM_IE)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmIeSpec;
impl crate::RegisterSpec for FlmIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_ie::R`](R) reader structure"]
impl crate::Readable for FlmIeSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_ie::W`](W) writer structure"]
impl crate::Writable for FlmIeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_IE to value 0"]
impl crate::Resettable for FlmIeSpec {
    const RESET_VALUE: u32 = 0;
}
