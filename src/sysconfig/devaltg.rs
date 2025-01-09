#[doc = "Register `DEVALTG` reader"]
pub type R = crate::R<DevaltgSpec>;
#[doc = "Register `DEVALTG` writer"]
pub type W = crate::W<DevaltgSpec>;
#[doc = "Field `VCC1_RST_PUD_LK` reader - VCC1_RST Pull-Up Disable Lock"]
pub type Vcc1RstPudLkR = crate::BitReader;
#[doc = "Field `VCC1_RST_PUD_LK` writer - VCC1_RST Pull-Up Disable Lock"]
pub type Vcc1RstPudLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCC1_RST_PUD` reader - VCC1_RST Pull-Up Disable"]
pub type Vcc1RstPudR = crate::BitReader;
#[doc = "Field `VCC1_RST_PUD` writer - VCC1_RST Pull-Up Disable"]
pub type Vcc1RstPudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_OUT_SL` reader - PSL Output Select"]
pub type PslOutSlR = crate::BitReader;
#[doc = "Field `PSL_OUT_SL` writer - PSL Output Select"]
pub type PslOutSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSL_GPO_SL` reader - PSL-Controlled GPO Select"]
pub type PslGpoSlR = crate::BitReader;
#[doc = "Field `PSL_GPO_SL` writer - PSL-Controlled GPO Select"]
pub type PslGpoSlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - VCC1_RST Pull-Up Disable Lock"]
    #[inline(always)]
    pub fn vcc1_rst_pud_lk(&self) -> Vcc1RstPudLkR {
        Vcc1RstPudLkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VCC1_RST Pull-Up Disable"]
    #[inline(always)]
    pub fn vcc1_rst_pud(&self) -> Vcc1RstPudR {
        Vcc1RstPudR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSL Output Select"]
    #[inline(always)]
    pub fn psl_out_sl(&self) -> PslOutSlR {
        PslOutSlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSL-Controlled GPO Select"]
    #[inline(always)]
    pub fn psl_gpo_sl(&self) -> PslGpoSlR {
        PslGpoSlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTG")
            .field("psl_gpo_sl", &self.psl_gpo_sl())
            .field("psl_out_sl", &self.psl_out_sl())
            .field("vcc1_rst_pud", &self.vcc1_rst_pud())
            .field("vcc1_rst_pud_lk", &self.vcc1_rst_pud_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - VCC1_RST Pull-Up Disable Lock"]
    #[inline(always)]
    pub fn vcc1_rst_pud_lk(&mut self) -> Vcc1RstPudLkW<DevaltgSpec> {
        Vcc1RstPudLkW::new(self, 4)
    }
    #[doc = "Bit 5 - VCC1_RST Pull-Up Disable"]
    #[inline(always)]
    pub fn vcc1_rst_pud(&mut self) -> Vcc1RstPudW<DevaltgSpec> {
        Vcc1RstPudW::new(self, 5)
    }
    #[doc = "Bit 6 - PSL Output Select"]
    #[inline(always)]
    pub fn psl_out_sl(&mut self) -> PslOutSlW<DevaltgSpec> {
        PslOutSlW::new(self, 6)
    }
    #[doc = "Bit 7 - PSL-Controlled GPO Select"]
    #[inline(always)]
    pub fn psl_gpo_sl(&mut self) -> PslGpoSlW<DevaltgSpec> {
        PslGpoSlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function G Register (DEVALTG)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltgSpec;
impl crate::RegisterSpec for DevaltgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltg::R`](R) reader structure"]
impl crate::Readable for DevaltgSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltg::W`](W) writer structure"]
impl crate::Writable for DevaltgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTG to value 0xc0"]
impl crate::Resettable for DevaltgSpec {
    const RESET_VALUE: u8 = 0xc0;
}
