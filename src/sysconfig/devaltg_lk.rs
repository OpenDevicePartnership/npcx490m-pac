#[doc = "Register `DEVALTG_LK` reader"]
pub type R = crate::R<DevaltgLkSpec>;
#[doc = "Register `DEVALTG_LK` writer"]
pub type W = crate::W<DevaltgLkSpec>;
#[doc = "Field `NO_VCC1_RST_LK` reader - VSBY NO_VCC1_RST Bit Lock"]
pub type NoVcc1RstLkR = crate::BitReader;
#[doc = "Field `NO_VCC1_RST_LK` writer - VSBY NO_VCC1_RST Bit Lock"]
pub type NoVcc1RstLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_OUT_SL_LK` reader - PSL Output Select Lock"]
pub type PslOutSlLkR = crate::BitReader;
#[doc = "Field `PSL_OUT_SL_LK` writer - PSL Output Select Lock"]
pub type PslOutSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_GPO_SL_LK` reader - PSL-Controlled GPO Select Lock"]
pub type PslGpoSlLkR = crate::BitReader;
#[doc = "Field `PSL_GPO_SL_LK` writer - PSL-Controlled GPO Select Lock"]
pub type PslGpoSlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - VSBY NO_VCC1_RST Bit Lock"]
    #[inline(always)]
    pub fn no_vcc1_rst_lk(&self) -> NoVcc1RstLkR {
        NoVcc1RstLkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSL Output Select Lock"]
    #[inline(always)]
    pub fn psl_out_sl_lk(&self) -> PslOutSlLkR {
        PslOutSlLkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSL-Controlled GPO Select Lock"]
    #[inline(always)]
    pub fn psl_gpo_sl_lk(&self) -> PslGpoSlLkR {
        PslGpoSlLkR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTG_LK")
            .field("psl_gpo_sl_lk", &self.psl_gpo_sl_lk())
            .field("psl_out_sl_lk", &self.psl_out_sl_lk())
            .field("no_vcc1_rst_lk", &self.no_vcc1_rst_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - VSBY NO_VCC1_RST Bit Lock"]
    #[inline(always)]
    pub fn no_vcc1_rst_lk(&mut self) -> NoVcc1RstLkW<DevaltgLkSpec> {
        NoVcc1RstLkW::new(self, 5)
    }
    #[doc = "Bit 6 - PSL Output Select Lock"]
    #[inline(always)]
    pub fn psl_out_sl_lk(&mut self) -> PslOutSlLkW<DevaltgLkSpec> {
        PslOutSlLkW::new(self, 6)
    }
    #[doc = "Bit 7 - PSL-Controlled GPO Select Lock"]
    #[inline(always)]
    pub fn psl_gpo_sl_lk(&mut self) -> PslGpoSlLkW<DevaltgLkSpec> {
        PslGpoSlLkW::new(self, 7)
    }
}
#[doc = "Device Alternate Function G Lock Register (DEVALTG_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltg_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltg_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltgLkSpec;
impl crate::RegisterSpec for DevaltgLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltg_lk::R`](R) reader structure"]
impl crate::Readable for DevaltgLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltg_lk::W`](W) writer structure"]
impl crate::Writable for DevaltgLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTG_LK to value 0"]
impl crate::Resettable for DevaltgLkSpec {
    const RESET_VALUE: u8 = 0;
}
