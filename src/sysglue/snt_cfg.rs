#[doc = "Register `SNT_CFG` reader"]
pub type R = crate::R<SntCfgSpec>;
#[doc = "Register `SNT_CFG` writer"]
pub type W = crate::W<SntCfgSpec>;
#[doc = "Field `SNT_EN` reader - Snooze Timer Function Enable"]
pub type SntEnR = crate::BitReader;
#[doc = "Field `SNT_EN` writer - Snooze Timer Function Enable"]
pub type SntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNT_WFE` reader - Snooze Timer Event Enable"]
pub type SntWfeR = crate::BitReader;
#[doc = "Field `SNT_WFE` writer - Snooze Timer Event Enable"]
pub type SntWfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNT_PRSC` reader - Snooze Timer Prescaler"]
pub type SntPrscR = crate::FieldReader;
#[doc = "Field `SNT_PRSC` writer - Snooze Timer Prescaler"]
pub type SntPrscW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Snooze Timer Function Enable"]
    #[inline(always)]
    pub fn snt_en(&self) -> SntEnR {
        SntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Snooze Timer Event Enable"]
    #[inline(always)]
    pub fn snt_wfe(&self) -> SntWfeR {
        SntWfeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Snooze Timer Prescaler"]
    #[inline(always)]
    pub fn snt_prsc(&self) -> SntPrscR {
        SntPrscR::new((self.bits >> 2) & 0x3f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNT_CFG")
            .field("snt_en", &self.snt_en())
            .field("snt_wfe", &self.snt_wfe())
            .field("snt_prsc", &self.snt_prsc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Snooze Timer Function Enable"]
    #[inline(always)]
    pub fn snt_en(&mut self) -> SntEnW<SntCfgSpec> {
        SntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Snooze Timer Event Enable"]
    #[inline(always)]
    pub fn snt_wfe(&mut self) -> SntWfeW<SntCfgSpec> {
        SntWfeW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Snooze Timer Prescaler"]
    #[inline(always)]
    pub fn snt_prsc(&mut self) -> SntPrscW<SntCfgSpec> {
        SntPrscW::new(self, 2)
    }
}
#[doc = "Snooze Timer Configuration Register (SNT_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`snt_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snt_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SntCfgSpec;
impl crate::RegisterSpec for SntCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snt_cfg::R`](R) reader structure"]
impl crate::Readable for SntCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`snt_cfg::W`](W) writer structure"]
impl crate::Writable for SntCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SNT_CFG to value 0"]
impl crate::Resettable for SntCfgSpec {
    const RESET_VALUE: u8 = 0;
}
