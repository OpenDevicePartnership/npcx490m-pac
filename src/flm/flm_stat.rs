#[doc = "Register `FLM_STAT` reader"]
pub type R = crate::R<FlmStatSpec>;
#[doc = "Register `FLM_STAT` writer"]
pub type W = crate::W<FlmStatSpec>;
#[doc = "Field `RJ_EV` reader - Rejection Event"]
pub type RjEvR = crate::BitReader;
#[doc = "Field `RJ_EV` writer - Rejection Event"]
pub type RjEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_EV` reader - Chip-Select Active Low Event"]
pub type CsiEvR = crate::BitReader;
#[doc = "Field `CSI_EV` writer - Chip-Select Active Low Event"]
pub type CsiEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCR_EV` reader - Transaction Counter Event"]
pub type TcrEvR = crate::BitReader;
#[doc = "Field `TCR_EV` writer - Transaction Counter Event"]
pub type TcrEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RJ_TP1` reader - Rejection Type 1"]
pub type RjTp1R = crate::FieldReader;
#[doc = "Field `RJ_TP1` writer - Rejection Type 1"]
pub type RjTp1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RJ_TP2` reader - Rejection Type 2"]
pub type RjTp2R = crate::FieldReader;
#[doc = "Field `RJ_TP2` writer - Rejection Type 2"]
pub type RjTp2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RJ_NO` reader - Rejection Number"]
pub type RjNoR = crate::FieldReader<u16>;
#[doc = "Field `RJ_NO` writer - Rejection Number"]
pub type RjNoW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Rejection Event"]
    #[inline(always)]
    pub fn rj_ev(&self) -> RjEvR {
        RjEvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Chip-Select Active Low Event"]
    #[inline(always)]
    pub fn csi_ev(&self) -> CsiEvR {
        CsiEvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Transaction Counter Event"]
    #[inline(always)]
    pub fn tcr_ev(&self) -> TcrEvR {
        TcrEvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Rejection Type 1"]
    #[inline(always)]
    pub fn rj_tp1(&self) -> RjTp1R {
        RjTp1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Rejection Type 2"]
    #[inline(always)]
    pub fn rj_tp2(&self) -> RjTp2R {
        RjTp2R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Rejection Number"]
    #[inline(always)]
    pub fn rj_no(&self) -> RjNoR {
        RjNoR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_STAT")
            .field("rj_ev", &self.rj_ev())
            .field("csi_ev", &self.csi_ev())
            .field("tcr_ev", &self.tcr_ev())
            .field("rj_tp1", &self.rj_tp1())
            .field("rj_tp2", &self.rj_tp2())
            .field("rj_no", &self.rj_no())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rejection Event"]
    #[inline(always)]
    #[must_use]
    pub fn rj_ev(&mut self) -> RjEvW<FlmStatSpec> {
        RjEvW::new(self, 0)
    }
    #[doc = "Bit 1 - Chip-Select Active Low Event"]
    #[inline(always)]
    #[must_use]
    pub fn csi_ev(&mut self) -> CsiEvW<FlmStatSpec> {
        CsiEvW::new(self, 1)
    }
    #[doc = "Bit 3 - Transaction Counter Event"]
    #[inline(always)]
    #[must_use]
    pub fn tcr_ev(&mut self) -> TcrEvW<FlmStatSpec> {
        TcrEvW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Rejection Type 1"]
    #[inline(always)]
    #[must_use]
    pub fn rj_tp1(&mut self) -> RjTp1W<FlmStatSpec> {
        RjTp1W::new(self, 4)
    }
    #[doc = "Bits 7:9 - Rejection Type 2"]
    #[inline(always)]
    #[must_use]
    pub fn rj_tp2(&mut self) -> RjTp2W<FlmStatSpec> {
        RjTp2W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Rejection Number"]
    #[inline(always)]
    #[must_use]
    pub fn rj_no(&mut self) -> RjNoW<FlmStatSpec> {
        RjNoW::new(self, 16)
    }
}
#[doc = "FLM Status Register (FLM_STAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmStatSpec;
impl crate::RegisterSpec for FlmStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_stat::R`](R) reader structure"]
impl crate::Readable for FlmStatSpec {}
#[doc = "`write(|w| ..)` method takes [`flm_stat::W`](W) writer structure"]
impl crate::Writable for FlmStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_STAT to value 0"]
impl crate::Resettable for FlmStatSpec {
    const RESET_VALUE: u32 = 0;
}
